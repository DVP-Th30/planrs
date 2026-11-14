//! Extraction du contenu des pages (étape 1 du pipeline, ADR-0032) : mots
//! positionnés, rectangles pleins et traits, par page, avec
//! `hayro-interpret` (Rust pur, MIT OR Apache-2.0).
//!
//! **Seul module qui manipule des flottants** : `hayro` rend des
//! coordonnées `f64` en points PDF ; elles sont converties ici, une fois,
//! en entiers de centièmes de point ([`Cpt`]), origine en haut à gauche de
//! la page, y vers le bas. Tout le reste du pipeline (en-tête, colonnes,
//! lignes, retraits) travaille sur ces entiers : mêmes octets, même
//! résultat, sur toutes les plateformes.
//!
//! Bornes (entrée hostile, règle 10) : nombre de pages lu, glyphes,
//! rectangles et traits gardés par page ([`ExtractLimits`]). Au-delà, les
//! éléments sont comptés mais pas gardés (`truncated`).

use hayro_interpret::font::{Glyph, GlyphRun};
use hayro_interpret::hayro_cmap::BfString;
use hayro_interpret::kurbo::{Affine, BezPath, Point, Rect, Shape, Vec2};
use hayro_interpret::{
    BlendMode, ClipPath, Context, Device, DrawMode, DrawProps, Image, ImageDrawProps, InterpreterCache,
    InterpreterSettings, Paint, RenderContent, SoftMask, TransformExt as _, interpret_page,
};
use hayro_syntax::{LoadPdfError, Pdf};

/// Centièmes de point PDF (1 pt = 1/72 pouce), entier.
pub type Cpt = i32;

/// Borne absolue d'une coordonnée (± 100 000 pt, soit 35 m) : au-delà, la
/// valeur est ramenée à la borne.
const MAX_CPT: f64 = 10_000_000.0;

/// Bornes de l'extraction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtractLimits {
    /// Pages lues au plus.
    pub max_pages: usize,
    /// Glyphes gardés par page.
    pub max_glyphs_per_page: usize,
    /// Rectangles pleins gardés par page.
    pub max_rects_per_page: usize,
    /// Traits gardés par page.
    pub max_segments_per_page: usize,
    /// Petites formes pleines (pointes de flèche, losanges) gardées par
    /// page (lot P1c).
    pub max_shapes_per_page: usize,
}

impl Default for ExtractLimits {
    fn default() -> Self {
        Self {
            max_pages: 500,
            max_glyphs_per_page: 200_000,
            max_rects_per_page: 50_000,
            max_segments_per_page: 50_000,
            max_shapes_per_page: 20_000,
        }
    }
}

/// Mot : suite de glyphes visibles sans espace, sur une même ligne de base.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Word {
    /// Texte (Unicode, tel que relu par la table `ToUnicode` de la police).
    pub text: String,
    /// Bord gauche.
    pub x0: Cpt,
    /// Bord droit (fin de l'avance du dernier glyphe).
    pub x1: Cpt,
    /// Ligne de base (y vers le bas).
    pub baseline: Cpt,
    /// Corps du texte (hauteur de l'em).
    pub size: Cpt,
    /// Police grasse (nom PostScript, graisse ou rendu « remplir et
    /// contourer »).
    pub bold: bool,
    /// Bord gauche de chaque glyphe et position (octets) de son texte
    /// dans `text` : coupe d'un mot à cheval sur deux colonnes.
    pub glyphs: Vec<(usize, Cpt)>,
}

impl Word {
    /// Coupe le mot au premier glyphe qui commence à `x` ou après : la
    /// partie gauche et la partie droite (`None` si rien n'est coupé).
    #[must_use]
    pub fn split_at_x(&self, x: Cpt) -> Option<(Self, Self)> {
        let k = self.glyphs.iter().position(|(_, gx)| *gx >= x)?;
        if k == 0 {
            return None;
        }
        let (byte, gx) = *self.glyphs.get(k)?;
        let left_text = self.text.get(..byte)?.to_owned();
        let right_text = self.text.get(byte..)?.to_owned();
        let left = Self {
            text: left_text,
            x0: self.x0,
            x1: gx,
            glyphs: self.glyphs.get(..k)?.to_vec(),
            ..self.clone()
        };
        let right = Self {
            text: right_text,
            x0: gx,
            x1: self.x1,
            glyphs: self
                .glyphs
                .get(k..)?
                .iter()
                .map(|(b, x)| (b.saturating_sub(byte), *x))
                .collect(),
            ..self.clone()
        };
        Some((left, right))
    }
}

impl Word {
    /// Haut approximatif du mot (ligne de base moins 80 % du corps).
    #[must_use]
    pub fn top(&self) -> Cpt {
        self.baseline.saturating_sub(self.size.saturating_mul(4) / 5)
    }
}

/// Couleur RVB d'un aplat (8 bits par composante), alpha à part.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgb(pub [u8; 3]);

impl Rgb {
    /// Écriture `#rrggbb`.
    #[must_use]
    pub fn hex(self) -> String {
        let [r, g, b] = self.0;
        format!("#{r:02x}{g:02x}{b:02x}")
    }
}

/// Rectangle plein (boîte englobante d'un remplissage), avec sa couleur.
/// Les barres du Gantt en sont, et les fonds de cellules.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FilledRect {
    /// Gauche.
    pub x0: Cpt,
    /// Haut.
    pub y0: Cpt,
    /// Droite.
    pub x1: Cpt,
    /// Bas.
    pub y1: Cpt,
    /// Couleur de remplissage (motif : gris moyen).
    pub color: Rgb,
    /// Vrai si le chemin est exactement un rectangle (sinon : boîte
    /// englobante d'une forme, losange de jalon, flèche…).
    pub is_rect: bool,
}

/// Trait horizontal ou vertical (trait de contour fin ou aplat très mince) :
/// quadrillage du tableau, séparation tableau / graphique, tronçons des
/// liens dessinés (lot P1c).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Segment {
    /// Gauche.
    pub x0: Cpt,
    /// Haut.
    pub y0: Cpt,
    /// Droite.
    pub x1: Cpt,
    /// Bas.
    pub y1: Cpt,
    /// Couleur du trait.
    pub color: Rgb,
    /// Épaisseur (trait de contour : épaisseur de plume dans l'espace de
    /// la page ; aplat mince : sa plus petite dimension).
    pub width: Cpt,
    /// Trait pointillé (tableau de tirets non vide).
    pub dashed: bool,
    /// Rang du sous-chemin dans la page : les tronçons d'une même
    /// polyligne dessinée d'un trait le partagent.
    pub path: u32,
    /// Petit tronçon oblique (coin arrondi) montant : ses extrémités sont
    /// `(x0, y1)` et `(x1, y0)` ; sinon `(x0, y0)` et `(x1, y1)`.
    pub rising: bool,
    /// Côté d'un contour fermé (rectangle, losange, pointe dessinés au
    /// trait) : jamais un tronçon de lien, qui est une polyligne ouverte.
    pub closed: bool,
}

/// Petite forme pleine non rectangulaire (pointe de flèche, losange de
/// jalon), avec ses sommets (lot P1c).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilledShape {
    /// Gauche.
    pub x0: Cpt,
    /// Haut.
    pub y0: Cpt,
    /// Droite.
    pub x1: Cpt,
    /// Bas.
    pub y1: Cpt,
    /// Couleur de remplissage.
    pub color: Rgb,
    /// Sommets (extrémités des segments droits), 16 au plus.
    pub points: Vec<(Cpt, Cpt)>,
    /// Rang du sous-chemin dans la page.
    pub path: u32,
}

impl Segment {
    /// Vrai pour un trait vertical.
    #[must_use]
    pub fn is_vertical(&self) -> bool {
        self.x1.saturating_sub(self.x0) < self.y1.saturating_sub(self.y0)
    }
}

/// Contenu d'une page.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PageContent {
    /// Rang de la page (0 = première).
    pub index: usize,
    /// Largeur de rendu (rotation appliquée).
    pub width: Cpt,
    /// Hauteur de rendu.
    pub height: Cpt,
    /// Mots, dans l'ordre de dessin.
    pub words: Vec<Word>,
    /// Aplats.
    pub rects: Vec<FilledRect>,
    /// Traits.
    pub segments: Vec<Segment>,
    /// Petites formes pleines non rectangulaires (au plus 30 pt de côté).
    pub shapes: Vec<FilledShape>,
    /// Glyphes vus (gardés ou non).
    pub glyphs: u64,
    /// Glyphes sans correspondance Unicode.
    pub unmapped_glyphs: u64,
    /// Glyphes invisibles (couche de texte d'un PDF numérisé et reconnu).
    pub invisible_glyphs: u64,
    /// Part de la page couverte par des images, en millièmes (bornée à
    /// 1 000 ; les recouvrements comptent deux fois).
    pub image_permille: u32,
    /// Éléments vus au-delà des bornes, non gardés.
    pub truncated: bool,
}

/// Document lu.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Extracted {
    /// Pages lues (au plus `max_pages`).
    pub pages: Vec<PageContent>,
    /// Pages du document.
    pub page_count: usize,
    /// Producteur déclaré (dictionnaire d'informations), s'il existe.
    pub producer: Option<String>,
    /// Créateur déclaré.
    pub creator: Option<String>,
}

/// Échec de la lecture du document entier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtractError {
    /// Document chiffré (mot de passe ou droits).
    Encrypted,
    /// Document illisible.
    Invalid,
    /// Le lecteur PDF a échoué de façon inattendue (panique interceptée).
    ReaderFailure,
}

/// Produit d'une transformation par une valeur géométrique de kurbo (point,
/// chemin, transformation), écrit en appel de méthode : l'opérateur `*`
/// des types de kurbo tombe sous `arithmetic_side_effects` (règle 10),
/// alors que ce produit est en flottants et ne peut pas déborder.
fn mul<T>(a: Affine, b: T) -> <Affine as std::ops::Mul<T>>::Output
where
    Affine: std::ops::Mul<T>,
{
    std::ops::Mul::mul(a, b)
}

/// Coordonnée flottante → centièmes de point, arrondie et bornée.
#[allow(
    clippy::cast_possible_truncation,
    reason = "valeur bornée à ±1e7 et arrondie juste avant : tient dans un i32"
)]
fn cpt(v: f64) -> Cpt {
    if v.is_nan() {
        return 0;
    }
    (v * 100.0).round().clamp(-MAX_CPT, MAX_CPT) as Cpt
}

/// Composante de couleur [0, 1] → 0..=255.
#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "valeur bornée à [0, 255] et arrondie juste avant"
)]
fn byte(v: f32) -> u8 {
    if v.is_nan() {
        return 0;
    }
    (v * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Lit le document : métadonnées et contenu des `max_pages` premières
/// pages. Sur le desktop, une panique du lecteur est interceptée et rendue
/// comme [`ExtractError::ReaderFailure`] (défense en profondeur : le
/// lecteur est écrit en Rust sans `unsafe`, mais un PDF est une entrée
/// hostile).
///
/// # Errors
///
/// [`ExtractError`] si le document entier est illisible.
pub fn extract(bytes: &[u8], limits: &ExtractLimits) -> Result<Extracted, ExtractError> {
    let data = bytes.to_vec();
    guarded(move || extract_inner(data, limits))
}

#[cfg(not(target_arch = "wasm32"))]
fn guarded<T>(f: impl FnOnce() -> Result<T, ExtractError>) -> Result<T, ExtractError> {
    // Le crochet de panique par défaut écrirait sur la sortie d'erreur : il
    // est laissé tel quel (l'application peut avoir le sien) ; seule la
    // propagation est arrêtée.
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_or(Err(ExtractError::ReaderFailure))
}

#[cfg(target_arch = "wasm32")]
fn guarded<T>(f: impl FnOnce() -> Result<T, ExtractError>) -> Result<T, ExtractError> {
    // En WebAssembly, une panique interrompt le module : pas d'interception
    // possible (limite consignée dans ADR-0032).
    f()
}

fn extract_inner(data: Vec<u8>, limits: &ExtractLimits) -> Result<Extracted, ExtractError> {
    let pdf = Pdf::new(data).map_err(|e| match e {
        LoadPdfError::Decryption(_) => ExtractError::Encrypted,
        LoadPdfError::Invalid => ExtractError::Invalid,
    })?;
    let meta = pdf.metadata();
    let text =
        |v: Option<&[u8]>| -> Option<String> { v.map(decode_pdf_text).filter(|s| !s.trim().is_empty()) };
    let producer = text(meta.producer.as_deref());
    let creator = text(meta.creator.as_deref());
    let all = pdf.pages();
    let page_count = all.len();
    // Les annotations (commentaires, liens) ne font pas partie du planning.
    let settings = InterpreterSettings {
        render_content: RenderContent::PAGE,
        ..InterpreterSettings::default()
    };
    let cache = InterpreterCache::new();
    let mut pages = Vec::new();
    for (index, page) in all.iter().enumerate().take(limits.max_pages) {
        let (w, h) = page.render_dimensions();
        let (w, h) = (f64::from(w), f64::from(h));
        let initial = page.initial_transform(true).to_kurbo();
        let run = |transform: Affine, w: f64, h: f64| {
            let mut device = Collector::new(index, w, h, limits);
            let mut ctx = Context::new(
                transform,
                Rect::new(0.0, 0.0, w, h),
                &cache,
                pdf.xref(),
                settings.clone(),
            );
            interpret_page(page, &mut ctx, &mut device);
            device
        };
        let device = run(initial, w, h);
        // Gantt imprimé de côté sur une page portrait (texte vertical
        // majoritaire) : page relue tournée d'un quart de tour, texte à
        // l'horizontale.
        let rotated = device.rotated_up.max(device.rotated_down);
        let device = if rotated >= 50 && rotated > device.horizontal.saturating_mul(2) {
            let quarter = if device.rotated_up >= device.rotated_down {
                Affine::new([0.0, 1.0, -1.0, 0.0, h, 0.0])
            } else {
                Affine::new([0.0, -1.0, 1.0, 0.0, 0.0, w])
            };
            run(mul(quarter, initial), h, w)
        } else {
            device
        };
        pages.push(device.finish());
    }
    Ok(Extracted {
        pages,
        page_count,
        producer,
        creator,
    })
}

/// Texte d'une chaîne d'informations PDF : UTF-16BE avec BOM, sinon
/// octets lus comme Latin-1 (approximation de `PDFDocEncoding`, suffisante
/// pour un nom de producteur).
fn decode_pdf_text(bytes: &[u8]) -> String {
    if let Some(rest) = bytes.strip_prefix(&[0xFE, 0xFF]) {
        let units: Vec<u16> = rest
            .as_chunks::<2>()
            .0
            .iter()
            .map(|c| u16::from_be_bytes(*c))
            .collect();
        return String::from_utf16_lossy(&units);
    }
    bytes.iter().map(|b| char::from(*b)).collect()
}

/// Glyphe placé, avant regroupement en mots.
#[derive(Clone, Debug)]
struct PlacedChar {
    text: String,
    x0: f64,
    x1: f64,
    baseline: f64,
    size: f64,
    bold: bool,
    space: bool,
    /// Rang de l'appel `draw_glyph_run` (une cellule de tableau en est un).
    run: u64,
    /// Positions des glyphes regroupés (octet du texte, bord gauche).
    glyphs: Vec<(usize, f64)>,
}

/// Appareil de collecte (`hayro_interpret::Device`).
struct Collector {
    page: PageContent,
    limits: ExtractLimits,
    /// Pile des boîtes de découpe (espace de la page).
    clips: Vec<Rect>,
    chars: Vec<PlacedChar>,
    /// Appels `draw_glyph_run` vus.
    runs: u64,
    /// Glyphes horizontaux, et tournés vers le haut ou vers le bas.
    horizontal: u64,
    rotated_up: u64,
    rotated_down: u64,
    /// Surface d'image cumulée (pt²).
    image_area: f64,
    page_area: f64,
    /// Rang du sous-chemin courant.
    path_serial: u32,
}

/// Style d'un trait : couleur, épaisseur, pointillé, contour fermé.
#[derive(Clone, Copy, Debug)]
struct Pen {
    color: Rgb,
    width: f64,
    dashed: bool,
    closed: bool,
}

impl Collector {
    fn new(index: usize, width: f64, height: f64, limits: &ExtractLimits) -> Self {
        Self {
            page: PageContent {
                index,
                width: cpt(width),
                height: cpt(height),
                ..PageContent::default()
            },
            limits: *limits,
            clips: vec![Rect::new(0.0, 0.0, width, height)],
            chars: Vec::new(),
            runs: 0,
            horizontal: 0,
            rotated_up: 0,
            rotated_down: 0,
            image_area: 0.0,
            page_area: (width * height).max(1.0),
            path_serial: 0,
        }
    }

    fn clip(&self) -> Rect {
        self.clips.last().copied().unwrap_or(Rect::ZERO)
    }

    fn finish(mut self) -> PageContent {
        self.page.words = group_words(&self.chars);
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            reason = "rapport borné à [0, 1000] juste avant"
        )]
        let permille = (self.image_area / self.page_area * 1000.0)
            .round()
            .clamp(0.0, 1000.0) as u32;
        self.page.image_permille = permille;
        self.page
    }

    fn push_fill(&mut self, bbox: Rect, color: Rgb, is_rect: bool) {
        let clip = self.clip();
        let b = bbox.intersect(clip);
        if b.width() <= 0.0 || b.height() <= 0.0 {
            return;
        }
        // Aplat très mince : un trait du quadrillage.
        if b.width() < 1.5 || b.height() < 1.5 {
            let pen = Pen {
                color,
                width: b.width().min(b.height()),
                dashed: false,
                closed: false,
            };
            self.push_segment(b, pen, false);
            return;
        }
        if self.page.rects.len() >= self.limits.max_rects_per_page {
            self.page.truncated = true;
            return;
        }
        self.page.rects.push(FilledRect {
            x0: cpt(b.x0),
            y0: cpt(b.y0),
            x1: cpt(b.x1),
            y1: cpt(b.y1),
            color,
            is_rect,
        });
    }

    fn push_segment(&mut self, b: Rect, pen: Pen, rising: bool) {
        // Ni horizontal ni vertical : ignoré (flèches de liens, diagonales).
        if b.width() >= 1.5 && b.height() >= 1.5 {
            return;
        }
        if self.page.segments.len() >= self.limits.max_segments_per_page {
            self.page.truncated = true;
            return;
        }
        self.page.segments.push(Segment {
            x0: cpt(b.x0),
            y0: cpt(b.y0),
            x1: cpt(b.x1),
            y1: cpt(b.y1),
            color: pen.color,
            width: cpt(pen.width),
            dashed: pen.dashed,
            path: self.path_serial,
            rising,
            closed: pen.closed,
        });
    }

    /// Petite forme pleine non rectangulaire (au plus 30 pt de côté, dans
    /// la découpe) : sommets gardés (lot P1c, pointes de flèche).
    fn push_shape(&mut self, sub: &BezPath, transform: Affine, color: Rgb) {
        let bbox = mul(transform, sub.clone()).bounding_box();
        let clip = self.clip();
        let b = bbox.intersect(clip);
        if b.width() <= 0.0 || b.height() <= 0.0 || b.width() > 30.0 || b.height() > 30.0 {
            return;
        }
        // Forme coupée par la découpe : sommets faussés, écartée.
        if b.width() < bbox.width() - 0.01 || b.height() < bbox.height() - 0.01 {
            return;
        }
        if self.page.shapes.len() >= self.limits.max_shapes_per_page {
            self.page.truncated = true;
            return;
        }
        let mut points: Vec<(Cpt, Cpt)> = Vec::new();
        for p in lines_of(sub, transform).into_iter().flat_map(|(a, b)| [a, b]) {
            let p = (cpt(p.x), cpt(p.y));
            if points.len() >= 16 {
                break;
            }
            if !points.contains(&p) {
                points.push(p);
            }
        }
        self.page.shapes.push(FilledShape {
            x0: cpt(b.x0),
            y0: cpt(b.y0),
            x1: cpt(b.x1),
            y1: cpt(b.y1),
            color,
            points,
            path: self.path_serial,
        });
    }
}

/// Sous-chemin fermé : au moins trois côtés, et une fermeture explicite ou
/// un dernier point confondu avec le premier.
fn is_closed(path: &BezPath, lines: &[(Point, Point)]) -> bool {
    use hayro_interpret::kurbo::PathEl;
    if lines.len() < 3 {
        return false;
    }
    let explicit = path.elements().iter().any(|e| matches!(e, PathEl::ClosePath));
    let back = match (lines.first(), lines.last()) {
        (Some((first, _)), Some((_, last))) => first.distance(*last) < 0.05,
        _ => false,
    };
    explicit || back
}

/// Épaisseur d'un trait de contour dans l'espace de la page : épaisseur de
/// plume multipliée par l'échelle moyenne de la transformation.
fn stroke_width(props: &hayro_interpret::StrokeProps, transform: Affine) -> f64 {
    let scale = transform.determinant().abs().sqrt();
    let w = f64::from(props.line_width) * scale;
    if w.is_finite() { w.max(0.0) } else { 0.0 }
}

/// Couleur d'une peinture : RVB d'une couleur unie, gris moyen pour un
/// motif ou un dégradé.
fn paint_color(paint: &Paint<'_>) -> Rgb {
    match paint {
        Paint::Color(c) => {
            let [r, g, b, _] = c.to_rgba().components();
            Rgb([byte(r), byte(g), byte(b)])
        }
        Paint::Pattern(_) => Rgb([128, 128, 128]),
    }
}

/// Sous-chemins d'un chemin (coupés à chaque `MoveTo`), 10 000 au plus.
fn subpaths(path: &BezPath) -> Vec<BezPath> {
    use hayro_interpret::kurbo::PathEl;
    let mut out: Vec<BezPath> = Vec::new();
    for el in path.elements() {
        if matches!(el, PathEl::MoveTo(_)) || out.is_empty() {
            if out.len() >= 10_000 {
                break;
            }
            out.push(BezPath::new());
        }
        if let Some(last) = out.last_mut() {
            last.push(*el);
        }
    }
    out
}

/// Segments droits d'un sous-chemin, dans l'espace de la page (une courbe
/// compte pour sa corde ; la fermeture compte comme un segment).
fn lines_of(path: &BezPath, transform: Affine) -> Vec<(Point, Point)> {
    use hayro_interpret::kurbo::PathEl;
    let mut out = Vec::new();
    let mut start: Option<Point> = None;
    let mut current: Option<Point> = None;
    for el in path.elements() {
        match el {
            PathEl::MoveTo(p) => {
                start = Some(mul(transform, *p));
                current = start;
            }
            // Une courbe (coin arrondi d'un lien dessiné) compte pour sa
            // corde : la polyligne reste continue (lot P1c).
            PathEl::LineTo(p) | PathEl::QuadTo(_, p) | PathEl::CurveTo(_, _, p) => {
                let p = mul(transform, *p);
                if let Some(c) = current {
                    out.push((c, p));
                }
                current = Some(p);
            }
            PathEl::ClosePath => {
                if let (Some(c), Some(s)) = (current, start) {
                    out.push((c, s));
                }
                current = start;
            }
        }
    }
    out
}

/// Vrai si le chemin est un rectangle aligné sur les axes (quatre côtés
/// droits, éventuellement fermé).
fn is_axis_rect(path: &BezPath, transform: Affine) -> bool {
    use hayro_interpret::kurbo::PathEl;
    let mut points = Vec::with_capacity(5);
    for el in path.elements() {
        match el {
            PathEl::MoveTo(p) | PathEl::LineTo(p) => {
                if points.len() >= 6 {
                    return false;
                }
                points.push(mul(transform, *p));
            }
            PathEl::ClosePath => {}
            PathEl::QuadTo(..) | PathEl::CurveTo(..) => return false,
        }
    }
    if points.len() < 4 {
        return false;
    }
    points
        .iter()
        .zip(points.iter().skip(1))
        .all(|(a, b)| (a.x - b.x).abs() < 0.01 || (a.y - b.y).abs() < 0.01)
}

/// Graisse d'un glyphe : nom PostScript (« Bold », « Black », « Heavy »,
/// « Semibold », « Demi ») ou graisse déclarée ≥ 600.
fn glyph_is_bold(glyph: &Glyph<'_>) -> bool {
    let Glyph::Outline(outline) = glyph else {
        return false;
    };
    let Some(data) = outline.font_data() else {
        return false;
    };
    if data.weight.is_some_and(|w| w >= 600) {
        return true;
    }
    data.postscript_name.as_deref().is_some_and(|name| {
        let lower = name.to_ascii_lowercase();
        ["bold", "black", "heavy", "semibold", "demi"]
            .iter()
            .any(|k| lower.contains(k))
    })
}

impl<'a> Device<'a> for Collector {
    fn draw_path(&mut self, path: &BezPath, props: DrawProps<'a>, draw_mode: &DrawMode) {
        // Un chemin peut porter plusieurs sous-chemins (tout le quadrillage
        // d'une page, plusieurs fonds de cellules) : chacun est lu à part.
        let fill = matches!(draw_mode, DrawMode::Fill(_) | DrawMode::FillAndStroke(..));
        let stroke = match draw_mode {
            DrawMode::Stroke(p) | DrawMode::FillAndStroke(_, p) => Some(p),
            DrawMode::Fill(_) | DrawMode::Invisible => None,
        };
        let color = paint_color(&props.paint);
        for sub in subpaths(path) {
            self.path_serial = self.path_serial.saturating_add(1);
            if fill {
                let bbox = mul(props.transform, sub.clone()).bounding_box();
                let rect = is_axis_rect(&sub, props.transform);
                self.push_fill(bbox, color, rect);
                if !rect {
                    self.push_shape(&sub, props.transform, color);
                }
            }
            if let Some(stroke) = stroke {
                let lines = lines_of(&sub, props.transform);
                let pen = Pen {
                    color,
                    width: stroke_width(stroke, props.transform),
                    dashed: !stroke.dash_array.is_empty(),
                    closed: is_closed(&sub, &lines),
                };
                for (a, b) in lines {
                    let seg = Rect::from_points(a, b).intersect(self.clip());
                    // y vers le bas : montant si x et y varient en sens opposés.
                    let rising = (b.x - a.x) * (b.y - a.y) < 0.0;
                    if seg.width() >= 0.0 && seg.height() >= 0.0 {
                        self.push_segment(seg, pen, rising);
                    }
                }
            }
        }
    }

    fn draw_rect(&mut self, rect: &Rect, props: DrawProps<'a>, draw_mode: &DrawMode) {
        let bbox = mul(props.transform, rect.to_path(0.1)).bounding_box();
        self.path_serial = self.path_serial.saturating_add(1);
        match draw_mode {
            DrawMode::Fill(_) | DrawMode::FillAndStroke(..) => {
                self.push_fill(bbox, paint_color(&props.paint), true);
            }
            DrawMode::Stroke(stroke) => {
                let pen = Pen {
                    color: paint_color(&props.paint),
                    width: stroke_width(stroke, props.transform),
                    dashed: !stroke.dash_array.is_empty(),
                    closed: true,
                };
                // Contour d'un rectangle : quatre côtés.
                let b = bbox;
                for side in [
                    Rect::new(b.x0, b.y0, b.x1, b.y0),
                    Rect::new(b.x0, b.y1, b.x1, b.y1),
                    Rect::new(b.x0, b.y0, b.x0, b.y1),
                    Rect::new(b.x1, b.y0, b.x1, b.y1),
                ] {
                    let s = side.intersect(self.clip());
                    if s.width() >= 0.0 && s.height() >= 0.0 {
                        self.push_segment(s, pen, false);
                    }
                }
            }
            DrawMode::Invisible => {}
        }
    }

    fn push_clip_path(&mut self, clip_path: &ClipPath) {
        let next = clip_path.path.bounding_box().intersect(self.clip());
        self.clips.push(next);
    }

    fn push_clip_rect(&mut self, rect: &Rect) {
        let next = rect.intersect(self.clip());
        self.clips.push(next);
    }

    fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'a>>, _: BlendMode) {}

    fn draw_glyph_run(&mut self, glyph_run: &GlyphRun<'_, 'a>, props: DrawProps<'a>, draw_mode: &DrawMode) {
        let invisible = matches!(draw_mode, DrawMode::Invisible);
        let stroke_bold = matches!(draw_mode, DrawMode::FillAndStroke(..));
        let glyphs = glyph_run.glyphs();
        let bold = stroke_bold || glyphs.first().is_some_and(|g| glyph_is_bold(g));
        let clip = self.clip();
        self.runs = self.runs.saturating_add(1);
        for glyph in glyphs {
            self.page.glyphs = self.page.glyphs.saturating_add(1);
            if invisible {
                self.page.invisible_glyphs = self.page.invisible_glyphs.saturating_add(1);
                continue;
            }
            let Some(unicode) = glyph.as_unicode() else {
                self.page.unmapped_glyphs = self.page.unmapped_glyphs.saturating_add(1);
                continue;
            };
            if self.chars.len() >= self.limits.max_glyphs_per_page {
                self.page.truncated = true;
                continue;
            }
            let t = mul(props.transform, glyph.transform());
            let origin = mul(t, Point::ZERO);
            // Corps : longueur de l'em vertical (1 000 unités du glyphe).
            let em_top = mul(t, Point::new(0.0, 1000.0));
            let em = Vec2::new(em_top.x - origin.x, em_top.y - origin.y);
            let size = em.hypot();
            let advance = match &**glyph {
                Glyph::Outline(o) => o.advance_width().map_or(500.0, f64::from),
                Glyph::Type3(g) => g.advance_width().map_or(500.0, f64::from),
            };
            let end = mul(t, Point::new(advance, 0.0));
            // Texte tourné (vertical) : compté (page imprimée de côté), ignoré.
            let run = Vec2::new(end.x - origin.x, end.y - origin.y);
            if run.x.abs() < run.y.abs() {
                if run.y < 0.0 {
                    self.rotated_up = self.rotated_up.saturating_add(1);
                } else {
                    self.rotated_down = self.rotated_down.saturating_add(1);
                }
                continue;
            }
            self.horizontal = self.horizontal.saturating_add(1);
            // Glyphe hors de la zone de découpe (texte coupé à la cellule) :
            // son centre doit y être, à un demi-point près.
            let center = Point::new(f64::midpoint(origin.x, end.x), origin.y - 0.3 * size);
            if center.x < clip.x0 - 0.5
                || center.x > clip.x1 + 0.5
                || center.y < clip.y0 - 0.5
                || center.y > clip.y1 + 0.5
            {
                continue;
            }
            let text = match unicode {
                BfString::Char(c) => c.to_string(),
                BfString::String(s) => s,
            };
            let space = text.chars().all(char::is_whitespace);
            self.chars.push(PlacedChar {
                text,
                x0: origin.x.min(end.x),
                x1: origin.x.max(end.x),
                baseline: origin.y,
                size,
                bold,
                space,
                run: self.runs,
                glyphs: vec![(0, origin.x.min(end.x))],
            });
        }
    }

    fn draw_image(&mut self, image: Image<'a, '_>, props: ImageDrawProps<'a>) {
        let _ = &image;
        let bbox = mul(props.transform, Rect::new(0.0, 0.0, 1.0, 1.0).to_path(0.1))
            .bounding_box()
            .intersect(self.clip());
        if bbox.width() > 0.0 && bbox.height() > 0.0 {
            self.image_area += bbox.area();
        }
    }

    fn pop_clip(&mut self) {
        if self.clips.len() > 1 {
            self.clips.pop();
        }
    }

    fn pop_transparency_group(&mut self) {}
}

/// Regroupe les glyphes en mots, dans l'ordre de dessin : un glyphe
/// rejoint le mot courant s'il est sur la même ligne de base (écart < 20 %
/// du corps), de même graisse, et commence au plus à 25 % du corps après
/// la fin du précédent — 3 % s'il vient d'un autre appel de dessin, car
/// deux cellules voisines sont dessinées par deux appels et peuvent
/// presque se toucher (« 04/05/15 » puis « TCE », « Phase 1 » puis
/// « 20 days » lus « 120 days » à 10 %) — ni loin derrière (un
/// retour en arrière de plus d'un demi-corps commence un mot). Une espace
/// termine le mot.
fn group_words(chars: &[PlacedChar]) -> Vec<Word> {
    let mut words = Vec::new();
    let mut current: Option<PlacedChar> = None;
    for c in chars {
        if c.space {
            if let Some(w) = current.take() {
                words.push(to_word(&w));
            }
            continue;
        }
        match current.as_mut() {
            Some(w)
                if (c.baseline - w.baseline).abs() < 0.2 * w.size.max(c.size)
                    && c.bold == w.bold
                    && c.x0 - w.x1 < if c.run == w.run { 0.25 } else { 0.03 } * w.size.max(c.size)
                    && c.x0 - w.x1 > -0.5 * w.size.max(c.size) =>
            {
                w.glyphs.push((w.text.len(), c.x0));
                w.text.push_str(&c.text);
                w.x1 = w.x1.max(c.x1);
                w.size = w.size.max(c.size);
                w.run = c.run;
            }
            _ => {
                if let Some(w) = current.take() {
                    words.push(to_word(&w));
                }
                current = Some(c.clone());
            }
        }
    }
    if let Some(w) = current.take() {
        words.push(to_word(&w));
    }
    words
}

fn to_word(c: &PlacedChar) -> Word {
    Word {
        text: c.text.clone(),
        x0: cpt(c.x0),
        x1: cpt(c.x1),
        baseline: cpt(c.baseline),
        size: cpt(c.size),
        bold: c.bold,
        glyphs: c.glyphs.iter().map(|(b, x)| (*b, cpt(*x))).collect(),
    }
}
