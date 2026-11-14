//! Liens relus dans le dessin du Gantt (lot P1c, ADR-0034).
//!
//! Les plannings reçus en PDF n'ont souvent pas de colonne Prédécesseurs :
//! les liens n'existent que comme flèches dessinées dans la zone graphique.
//! Ce module les relit, en entiers (centièmes de point), à partir des
//! tracés extraits ([`crate::extract`]) :
//!
//! 1. **pointes** : petites formes pleines à trois sommets dont deux sont
//!    alignés (base) et le troisième au-delà (pointe), dans la zone
//!    graphique ; sens donné par la pointe ;
//! 2. **traits de lien** : traits horizontaux, verticaux et petits coins
//!    obliques, pleins (non pointillés), fins, de la couleur d'une pointe de
//!    la page, dans la zone graphique, sans monter dans l'en-tête de
//!    l'échelle de temps ; écartés : contours des barres et tracés internes
//!    des pointes ;
//! 3. **chemins** : depuis chaque pointe, on remonte le tracé de proche en
//!    proche, d'une extrémité de trait à l'extrémité d'un autre (coin ou
//!    prolongement), sans jamais revenir en arrière sur un trait superposé
//!    (les liens d'un même prédécesseur partagent leur début) ; faute
//!    d'extrémité voisine, une jonction en T (tronçon commun dessiné une
//!    seule fois) est franchie dans les deux sens ; un bout libre qui n'est
//!    ni une pointe ni une jonction est un **départ** ;
//! 4. **rattachement** : la tâche est la ligne du tableau de la même
//!    ordonnée ; la pointe vers la droite, le bas ou le haut entre par le
//!    début du successeur, vers la gauche par sa fin ; le départ quitte la
//!    fin du prédécesseur s'il part vers la droite, son début s'il part vers
//!    la gauche ; d'où FD, DD, FF, DF. Chaque bout est contrôlé contre les
//!    barres de sa ligne (confiance et motif de doute).
//!
//! Bornes (entrée hostile, règle 10) : traits candidats et pointes par page,
//! pas de calcul par document (au-delà : relevé partiel, `truncated`).
//! Déterministe : aucune table de hachage, ordre des pages et des tracés.

use std::collections::{BTreeMap, BTreeSet};

use plannercraft_core::LinkType;
use serde::Serialize;

use crate::extract::{Cpt, FilledShape, PageContent, Rgb, Segment};

/// Point en centièmes de point (x, y vers le bas).
type P = (i64, i64);

/// Écart de jonction entre deux extrémités de traits (0,5 pt) : les coins
/// arrondis sont relus comme de petits tronçons obliques (cordes des
/// courbes), les extrémités d'une polyligne coïncident.
const JOIN: i64 = 50;
/// Écart entre une pointe et le bout du trait qui y mène (1,5 pt).
const AT_ARROW: i64 = 150;
/// Écart toléré entre un bout de lien et le bord d'une barre (3 pt).
const ON_BAR: i64 = 300;
/// Épaisseur au plus d'un trait de lien (2 pt).
const MAX_WIDTH: Cpt = 200;
/// Côtés d'une pointe de flèche : 0,8 à 15 pt.
const ARROW_MIN: i64 = 80;
const ARROW_MAX: i64 = 1500;
/// Taille d'une case de l'index des extrémités (5 pt).
const CELL: i64 = 500;

/// Bornes du relevé.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkLimits {
    /// Traits candidats gardés par page.
    pub max_segments_per_page: usize,
    /// Pointes gardées par page.
    pub max_arrows_per_page: usize,
    /// Pas de calcul (examens d'extrémités voisines) par document.
    pub max_steps: u64,
}

impl Default for LinkLimits {
    fn default() -> Self {
        Self {
            max_segments_per_page: 20_000,
            max_arrows_per_page: 5_000,
            max_steps: 20_000_000,
        }
    }
}

/// Ligne du tableau d'une page : ordonnée et N° de la tâche.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RowRef {
    /// Ligne de base de la première ligne de texte.
    pub baseline: Cpt,
    /// Corps du texte.
    pub size: Cpt,
    /// N° de la tâche.
    pub id: u32,
}

/// Page à relire : contenu, zone graphique, lignes du tableau.
#[derive(Clone, Debug)]
pub struct PageFrame<'a> {
    /// Contenu extrait.
    pub page: &'a PageContent,
    /// Bord gauche de la zone graphique (bord droit du tableau).
    pub chart_left: Cpt,
    /// Haut des lignes (bas de l'en-tête).
    pub top: Cpt,
    /// Lignes du tableau de la page.
    pub rows: Vec<RowRef>,
}

/// Motif de doute d'un lien relu.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkDoubt {
    /// Le départ ne touche aucune barre de sa ligne.
    SourceOffBar,
    /// La pointe ne touche aucune barre de sa ligne.
    TargetOffBar,
    /// Départ vertical (jalon, barre courte) : côté supposé (fin).
    SourceSideGuessed,
}

impl LinkDoubt {
    /// Points de confiance retirés.
    const fn penalty(self) -> u8 {
        match self {
            // Mesure (lot P1c) : bouts hors barre justes 75 fois sur 75,
            // côté deviné faux 2 fois sur 2 (départ vertical en haut de
            // page) : sous le seuil des liens créés.
            Self::SourceOffBar | Self::TargetOffBar => 10,
            Self::SourceSideGuessed => 35,
        }
    }

    /// Code stable.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::SourceOffBar => "source_off_bar",
            Self::TargetOffBar => "target_off_bar",
            Self::SourceSideGuessed => "source_side_guessed",
        }
    }
}

/// Motif d'un lien (ou d'une pointe) écarté.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LinkDrop {
    /// Pointe sans trait de lien qui y mène (forme isolée).
    NoLine,
    /// Tracé sans départ libre (boucle, tracé qui revient sur une pointe).
    NoSource,
    /// Tracé qui se perd sur le corps d'un autre trait (jonction en T ou
    /// croisement) : départ indécidable.
    Junction,
    /// Tracé coupé : départ vertical qui ne touche aucune barre (lien venu
    /// d'une autre page ou d'une tâche masquée).
    Cut,
    /// Départ hors des lignes du tableau de la page (autre page, ligne
    /// masquée).
    SourceOutside,
    /// Pointe hors des lignes du tableau de la page.
    TargetOutside,
    /// Lien d'une tâche vers elle-même.
    SelfLink,
    /// Même couple relu plusieurs fois (pages, tracés superposés) : gardé
    /// une fois.
    Duplicate,
    /// Bornes du relevé atteintes.
    Bounded,
}

impl LinkDrop {
    /// Code stable.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::NoLine => "no_line",
            Self::NoSource => "no_source",
            Self::Junction => "junction",
            Self::Cut => "cut",
            Self::SourceOutside => "source_outside",
            Self::TargetOutside => "target_outside",
            Self::SelfLink => "self_link",
            Self::Duplicate => "duplicate",
            Self::Bounded => "bounded",
        }
    }

    /// Libellé court.
    #[must_use]
    pub const fn label(self, lang: plannercraft_automation::text::Language) -> &'static str {
        use plannercraft_automation::text::Language::{En, Fr};
        match (self, lang) {
            (Self::NoLine, Fr) => "pointe sans trait",
            (Self::NoLine, En) => "arrowhead without a line",
            (Self::NoSource, Fr) => "tracé sans départ",
            (Self::NoSource, En) => "path without a start",
            (Self::Junction, Fr) => "tracé perdu à une jonction",
            (Self::Junction, En) => "path lost at a junction",
            (Self::Cut, Fr) => "tracé coupé (autre page)",
            (Self::Cut, En) => "path cut (another page)",
            (Self::SourceOutside, Fr) => "départ hors du tableau",
            (Self::SourceOutside, En) => "start outside the table",
            (Self::TargetOutside, Fr) => "pointe hors du tableau",
            (Self::TargetOutside, En) => "arrowhead outside the table",
            (Self::SelfLink, Fr) => "tâche liée à elle-même",
            (Self::SelfLink, En) => "task linked to itself",
            (Self::Duplicate, Fr) => "relu deux fois",
            (Self::Duplicate, En) => "read twice",
            (Self::Bounded, Fr) => "bornes atteintes",
            (Self::Bounded, En) => "limits reached",
        }
    }
}

/// Lien relu dans le dessin.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DrawnLink {
    /// N° du prédécesseur.
    pub pred: u32,
    /// N° du successeur.
    pub succ: u32,
    /// Type (côtés de départ et d'arrivée).
    pub kind: LinkType,
    /// Confiance, de 0 à 100.
    pub confidence: u8,
    /// Motifs de doute, triés.
    pub doubts: Vec<LinkDoubt>,
    /// Page (0 = première).
    pub page: usize,
    /// Départ du tracé dans la page (centièmes de point).
    pub from: (i64, i64),
    /// Pointe de la flèche.
    pub to: (i64, i64),
}

/// Seuil de confiance au-dessous duquel un lien relu est douteux.
pub const DOUBTFUL_LINK_BELOW: u8 = 70;

impl DrawnLink {
    /// Lien douteux (confiance sous [`DOUBTFUL_LINK_BELOW`]).
    #[must_use]
    pub const fn doubtful(&self) -> bool {
        self.confidence < DOUBTFUL_LINK_BELOW
    }
}

/// Relevé des liens dessinés d'un document.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DrawnLinks {
    /// Liens relus, triés par (successeur, prédécesseur).
    pub links: Vec<DrawnLink>,
    /// Pointes de flèche trouvées.
    pub arrows: u32,
    /// Écartés, par motif.
    pub dropped: BTreeMap<LinkDrop, u32>,
    /// Bornes atteintes : relevé partiel.
    pub truncated: bool,
    /// Pointes écartées : page, position de la pointe, motif (200 au plus,
    /// pour le rapport et la mise au point).
    pub dropped_at: Vec<DroppedArrow>,
    /// Tâches dont la ligne porte une barre ou une forme dans la zone
    /// graphique (mesure : un lien entre deux tâches sans barre visible ne
    /// peut pas être dessiné).
    pub rows_with_bars: BTreeSet<u32>,
}

/// Pointe écartée et où.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct DroppedArrow {
    /// Page (0 = première).
    pub page: usize,
    /// Abscisse de la pointe (centièmes de point).
    pub x: i64,
    /// Ordonnée de la pointe.
    pub y: i64,
    /// Motif.
    pub why: LinkDrop,
}

/// Pointes écartées gardées au plus dans le relevé.
const MAX_DROPPED_AT: usize = 200;

impl DrawnLinks {
    /// Écartés, tous motifs confondus (doublons compris).
    #[must_use]
    pub fn dropped_total(&self) -> u32 {
        self.dropped.values().fold(0_u32, |a, b| a.saturating_add(*b))
    }

    fn drop(&mut self, why: LinkDrop) {
        let n = self.dropped.entry(why).or_insert(0);
        *n = n.saturating_add(1);
    }

    fn drop_at(&mut self, why: LinkDrop, page: usize, at: P) {
        self.drop(why);
        if self.dropped_at.len() < MAX_DROPPED_AT {
            self.dropped_at.push(DroppedArrow {
                page,
                x: at.0,
                y: at.1,
                why,
            });
        }
    }
}

/// Sens d'une pointe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Dir {
    Right,
    Left,
    Down,
    Up,
}

/// Pointe de flèche.
#[derive(Clone, Copy, Debug)]
struct Arrow {
    tip: P,
    base: P,
    dir: Dir,
    color: Rgb,
    bbox: (i64, i64, i64, i64),
}

/// Trait candidat : extrémités, couleur.
#[derive(Clone, Copy, Debug)]
struct Seg {
    a: P,
    b: P,
    color: Rgb,
}

impl Seg {
    fn end(&self, k: usize) -> P {
        if k == 0 { self.a } else { self.b }
    }
}

/// Barre (ou forme) de la zone graphique.
#[derive(Clone, Copy, Debug)]
struct Bar {
    x0: i64,
    y0: i64,
    x1: i64,
    y1: i64,
}

/// Couleurs voisines (4 unités au plus par composante) : un trait et sa
/// pointe sortent parfois d'un pilote d'impression avec une composante
/// arrondie différemment (`#267bd3` / `#267cd3`, observé).
fn same_color(a: Rgb, b: Rgb) -> bool {
    a.0.iter().zip(b.0).all(|(x, y)| x.abs_diff(y) <= 4)
}

fn half(a: i64, b: i64) -> i64 {
    a.saturating_add(b) / 2
}

fn sign(v: i64) -> i64 {
    v.signum()
}

/// Extrémités d'un trait extrait : vertical, horizontal ou petit coin
/// oblique (sens donné par `rising`).
fn ends(s: &Segment) -> (P, P) {
    let (x0, y0, x1, y1) = (i64::from(s.x0), i64::from(s.y0), i64::from(s.x1), i64::from(s.y1));
    let (w, h) = (x1.saturating_sub(x0), y1.saturating_sub(y0));
    if w <= 10 || w.saturating_mul(4) <= h {
        let x = half(x0, x1);
        ((x, y0), (x, y1))
    } else if h <= 10 || h.saturating_mul(4) <= w {
        let y = half(y0, y1);
        ((x0, y), (x1, y))
    } else if s.rising {
        ((x0, y1), (x1, y0))
    } else {
        ((x0, y0), (x1, y1))
    }
}

/// Pointe d'une forme à trois sommets : deux sommets alignés (base), le
/// troisième au-delà, dans l'axe de la base.
fn arrow_of(sh: &FilledShape) -> Option<Arrow> {
    let pts: Vec<P> = sh
        .points
        .iter()
        .map(|(x, y)| (i64::from(*x), i64::from(*y)))
        .collect();
    let [p0, p1, p2] = <[P; 3]>::try_from(pts).ok()?;
    let (x0, y0, x1, y1) = (
        i64::from(sh.x0),
        i64::from(sh.y0),
        i64::from(sh.x1),
        i64::from(sh.y1),
    );
    let (w, h) = (x1.saturating_sub(x0), y1.saturating_sub(y0));
    if !(ARROW_MIN..=ARROW_MAX).contains(&w) || !(ARROW_MIN..=ARROW_MAX).contains(&h) {
        return None;
    }
    for (tip, a, b) in [(p0, p1, p2), (p1, p2, p0), (p2, p0, p1)] {
        let base = (half(a.0, b.0), half(a.1, b.1));
        let within =
            |v: i64, p: i64, q: i64| v >= p.min(q).saturating_sub(30) && v <= p.max(q).saturating_add(30);
        if a.1.abs_diff(b.1) <= 30 && tip.1.abs_diff(a.1) > 60 && within(tip.0, a.0, b.0) {
            let dir = if tip.1 > a.1 { Dir::Down } else { Dir::Up };
            return Some(Arrow {
                tip,
                base,
                dir,
                color: sh.color,
                bbox: (x0, y0, x1, y1),
            });
        }
        if a.0.abs_diff(b.0) <= 30 && tip.0.abs_diff(a.0) > 60 && within(tip.1, a.1, b.1) {
            let dir = if tip.0 > a.0 { Dir::Right } else { Dir::Left };
            return Some(Arrow {
                tip,
                base,
                dir,
                color: sh.color,
                bbox: (x0, y0, x1, y1),
            });
        }
    }
    None
}

/// Distance (norme infinie) d'un point à un trait.
fn dist_to_seg(p: P, s: &Seg) -> i64 {
    let (lo_x, hi_x) = (s.a.0.min(s.b.0), s.a.0.max(s.b.0));
    let (lo_y, hi_y) = (s.a.1.min(s.b.1), s.a.1.max(s.b.1));
    let dx = if p.0 < lo_x {
        lo_x.saturating_sub(p.0)
    } else {
        p.0.saturating_sub(hi_x).max(0)
    };
    let dy = if p.1 < lo_y {
        lo_y.saturating_sub(p.1)
    } else {
        p.1.saturating_sub(hi_y).max(0)
    };
    dx.max(dy)
}

fn near(p: P, q: P, tol: i64) -> bool {
    p.0.abs_diff(q.0) <= tol.unsigned_abs() && p.1.abs_diff(q.1) <= tol.unsigned_abs()
}

/// Bandes des lignes du tableau : centre de chaque ligne (ligne de base
/// moins 35 % du corps), bornes aux milieux entre centres voisins.
struct Bands {
    /// (haut, bas, N°), dans l'ordre vertical.
    bands: Vec<(i64, i64, u32)>,
}

impl Bands {
    fn new(rows: &[RowRef]) -> Self {
        let mut rows: Vec<RowRef> = rows.to_vec();
        rows.sort_by_key(|r| (r.baseline, r.id));
        let centers: Vec<(i64, u32, i64)> = rows
            .iter()
            .map(|r| {
                let size = i64::from(r.size.max(100));
                (
                    i64::from(r.baseline).saturating_sub(size.saturating_mul(35) / 100),
                    r.id,
                    size,
                )
            })
            .collect();
        let mut bands = Vec::with_capacity(centers.len());
        for (k, (c, id, size)) in centers.iter().enumerate() {
            let prev = k.checked_sub(1).and_then(|j| centers.get(j)).map(|p| p.0);
            let next = centers.get(k.saturating_add(1)).map(|n| n.0);
            let pitch = match (prev, next) {
                (_, Some(n)) => n.saturating_sub(*c),
                (Some(p), None) => c.saturating_sub(p),
                (None, None) => size.saturating_mul(2),
            }
            .clamp(*size, size.saturating_mul(6));
            let top = prev.map_or_else(|| c.saturating_sub(pitch / 2), |p| half(p, *c));
            let bottom = next.map_or_else(|| c.saturating_add(pitch / 2), |n| half(*c, n));
            bands.push((top, bottom, *id));
        }
        Self { bands }
    }

    /// N° de la ligne qui contient l'ordonnée `y`.
    fn row_at(&self, y: i64) -> Option<u32> {
        let k = self.bands.partition_point(|(_, bottom, _)| *bottom <= y);
        self.bands
            .get(k)
            .filter(|(top, bottom, _)| y >= *top && y < *bottom)
            .map(|(_, _, id)| *id)
    }
}

/// Relevé d'une page.
struct PageReader<'a> {
    segs: Vec<Seg>,
    arrows: Vec<Arrow>,
    bars: Vec<Bar>,
    /// Extrémités par case : (trait, extrémité).
    cells: BTreeMap<(i64, i64), Vec<(usize, usize)>>,
    bands: Bands,
    page: &'a PageContent,
}

/// Départ trouvé en remontant le tracé d'une pointe.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Start {
    point: P,
    /// Sens de marche à l'arrivée sur le départ (de la pointe vers lui).
    walk: P,
}

fn cell_of(p: P) -> (i64, i64) {
    (p.0.div_euclid(CELL), p.1.div_euclid(CELL))
}

impl<'a> PageReader<'a> {
    #[allow(
        clippy::too_many_lines,
        reason = "filtres successifs des tracés, commentés dans l'ordre"
    )]
    fn new(frame: &PageFrame<'a>, limits: &LinkLimits, out: &mut DrawnLinks) -> Self {
        let page = frame.page;
        let left = i64::from(frame.chart_left);
        let top = i64::from(frame.top);
        // 1. Pointes de la zone graphique, dédoublonnées (pointes superposées
        // de liens qui arrivent au même endroit).
        let mut arrows: Vec<Arrow> = Vec::new();
        let mut seen: BTreeSet<(P, u8)> = BTreeSet::new();
        for sh in &page.shapes {
            let Some(a) = arrow_of(sh) else { continue };
            if a.bbox.0 < left.saturating_sub(JOIN) || a.bbox.1 < top.saturating_sub(JOIN) {
                continue;
            }
            let key = (
                (a.tip.0.div_euclid(20), a.tip.1.div_euclid(20)),
                match a.dir {
                    Dir::Right => 0,
                    Dir::Left => 1,
                    Dir::Down => 2,
                    Dir::Up => 3,
                },
            );
            if !seen.insert(key) {
                continue;
            }
            if arrows.len() >= limits.max_arrows_per_page {
                out.truncated = true;
                break;
            }
            arrows.push(a);
        }
        let colors: BTreeSet<Rgb> = arrows.iter().map(|a| a.color).collect();
        // Barres et formes de la zone graphique (contours écartés des traits,
        // rattachement des bouts).
        let mut bars: Vec<Bar> = page
            .rects
            .iter()
            .filter(|r| {
                let h = r.y1.saturating_sub(r.y0);
                r.x1 > frame.chart_left && r.y0 >= frame.top && (100..=2500).contains(&h)
            })
            .map(|r| Bar {
                x0: i64::from(r.x0),
                y0: i64::from(r.y0),
                x1: i64::from(r.x1),
                y1: i64::from(r.y1),
            })
            .collect();
        bars.extend(
            page.shapes
                .iter()
                .filter(|s| s.points.len() >= 4 && s.x1 > frame.chart_left && s.y0 >= frame.top)
                .map(|s| Bar {
                    x0: i64::from(s.x0),
                    y0: i64::from(s.y0),
                    x1: i64::from(s.x1),
                    y1: i64::from(s.y1),
                }),
        );
        // Barres très étroites (aplat de moins de 1,5 pt, relu comme un trait)
        // et contours de barres non remplies : côtés verticaux courts, d'une
        // autre couleur que les liens.
        bars.extend(page.segments.iter().filter_map(|s| {
            let (a, b) = ends(s);
            let len = b.1.saturating_sub(a.1);
            let vertical = a.0 == b.0 && (100..=2500).contains(&len);
            let linked = colors.iter().any(|c| same_color(*c, s.color));
            (vertical && !linked && a.0 > left && a.1 >= top).then_some(Bar {
                x0: a.0,
                y0: a.1,
                x1: a.0,
                y1: b.1,
            })
        }));
        bars.sort_by_key(|b| (b.y0, b.x0, b.y1, b.x1));
        let mut bar_cells: BTreeMap<i64, Vec<usize>> = BTreeMap::new();
        for (k, b) in bars.iter().enumerate() {
            let mut c = b.y0.div_euclid(1000);
            while c <= b.y1.div_euclid(1000) {
                bar_cells.entry(c).or_default().push(k);
                c = c.saturating_add(1);
            }
        }
        let on_bar_edge = |a: P, b: P| -> bool {
            let horizontal = a.1 == b.1;
            let vertical = a.0 == b.0;
            if vertical && a.1.abs_diff(b.1) > 2600 {
                return false;
            }
            let Some(list) = bar_cells.get(&half(a.1, b.1).div_euclid(1000)) else {
                return false;
            };
            list.iter().filter_map(|k| bars.get(*k)).any(|bar| {
                let (lx, hx) = (a.0.min(b.0), a.0.max(b.0));
                let (ly, hy) = (a.1.min(b.1), a.1.max(b.1));
                let inside_x = lx >= bar.x0.saturating_sub(40) && hx <= bar.x1.saturating_add(40);
                let inside_y = ly >= bar.y0.saturating_sub(40) && hy <= bar.y1.saturating_add(40);
                (horizontal && inside_x && (a.1.abs_diff(bar.y0) <= 40 || a.1.abs_diff(bar.y1) <= 40))
                    || (vertical && inside_y && (a.0.abs_diff(bar.x0) <= 40 || a.0.abs_diff(bar.x1) <= 40))
            })
        };
        // 2. Traits de lien.
        let mut segs: Vec<Seg> = Vec::new();
        for s in &page.segments {
            if s.dashed || s.closed || s.width > MAX_WIDTH || !colors.iter().any(|c| same_color(*c, s.color))
            {
                continue;
            }
            let (a, b) = ends(s);
            if a == b {
                continue;
            }
            if a.0.min(b.0) < left.saturating_sub(JOIN) || a.1.min(b.1) < top.saturating_sub(JOIN) {
                continue;
            }
            // Tracé interne d'une pointe (base, contour) : tout trait contenu
            // dans la pointe qui n'est pas dans son axe.
            let in_arrow = arrows.iter().any(|ar| {
                let (x0, y0, x1, y1) = ar.bbox;
                let inside = [a, b].iter().all(|p| {
                    p.0 >= x0.saturating_sub(40)
                        && p.0 <= x1.saturating_add(40)
                        && p.1 >= y0.saturating_sub(40)
                        && p.1 <= y1.saturating_add(40)
                });
                let along = if matches!(ar.dir, Dir::Down | Dir::Up) {
                    a.0 == b.0
                } else {
                    a.1 == b.1
                };
                inside && !along
            });
            if in_arrow || on_bar_edge(a, b) {
                continue;
            }
            if segs.len() >= limits.max_segments_per_page {
                out.truncated = true;
                break;
            }
            segs.push(Seg { a, b, color: s.color });
        }
        let mut cells: BTreeMap<(i64, i64), Vec<(usize, usize)>> = BTreeMap::new();
        for (k, s) in segs.iter().enumerate() {
            for e in 0..2 {
                cells.entry(cell_of(s.end(e))).or_default().push((k, e));
            }
        }
        Self {
            segs,
            arrows,
            bars,
            cells,
            bands: Bands::new(&frame.rows),
            page,
        }
    }

    /// Extrémités de traits (de même couleur) à moins de `JOIN` de `p`.
    fn ends_near(&self, p: P, color: Rgb, steps: &mut u64) -> Vec<(usize, usize)> {
        let (cx, cy) = cell_of(p);
        let mut out = Vec::new();
        for dx in -1..=1_i64 {
            for dy in -1..=1_i64 {
                let Some(list) = self.cells.get(&(cx.saturating_add(dx), cy.saturating_add(dy))) else {
                    continue;
                };
                for (k, e) in list {
                    *steps = steps.saturating_add(1);
                    if let Some(s) = self.segs.get(*k)
                        && same_color(s.color, color)
                        && near(s.end(*e), p, JOIN)
                    {
                        out.push((*k, *e));
                    }
                }
            }
        }
        out.sort_unstable();
        out
    }

    /// Point au bout d'une pointe (base ou pointe) atteint en marchant dans
    /// son sens : la remontée a suivi le tracé d'un autre lien jusqu'à sa
    /// pointe, elle s'y arrête. Une pointe croisée dans un autre sens (barre
    /// très courte dont la pointe d'entrée touche le départ) ne compte pas.
    fn at_arrow(&self, p: P, color: Rgb, walk: P) -> bool {
        self.arrows.iter().any(|a| {
            let along = match a.dir {
                Dir::Down => walk == (0, 1),
                Dir::Up => walk == (0, -1),
                Dir::Right => walk == (1, 0),
                Dir::Left => walk == (-1, 0),
            };
            along
                && same_color(a.color, color)
                && match a.dir {
                    Dir::Down | Dir::Up => {
                        p.0.abs_diff(a.tip.0) <= AT_ARROW.unsigned_abs()
                            && p.1 >= a.tip.1.min(a.base.1).saturating_sub(AT_ARROW)
                            && p.1 <= a.tip.1.max(a.base.1).saturating_add(AT_ARROW)
                    }
                    Dir::Left | Dir::Right => {
                        p.1.abs_diff(a.tip.1) <= AT_ARROW.unsigned_abs()
                            && p.0 >= a.tip.0.min(a.base.0).saturating_sub(AT_ARROW)
                            && p.0 <= a.tip.0.max(a.base.0).saturating_add(AT_ARROW)
                    }
                }
        })
    }

    /// Départs atteints en remontant le tracé de la pointe `arrow`, et
    /// nombre de bouts perdus sur le corps d'un autre trait (jonction en T :
    /// tronçon commun dessiné une seule fois, ou croisement ; écartés, la
    /// mesure les donne faux 3 fois sur 4).
    fn trace(&self, arrow: &Arrow, steps: &mut u64, budget: u64) -> (Vec<Start>, u32) {
        // Traits qui arrivent à la pointe : une extrémité entre la base et
        // la pointe (dans l'axe), l'autre du côté de la base.
        let back: P = match arrow.dir {
            Dir::Down => (0, -1),
            Dir::Up => (0, 1),
            Dir::Right => (-1, 0),
            Dir::Left => (1, 0),
        };
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for (k, s) in self.segs.iter().enumerate() {
            *steps = steps.saturating_add(1);
            if !same_color(s.color, arrow.color) {
                continue;
            }
            for e in 0..2 {
                let p = s.end(e);
                let other = s.end(1_usize.saturating_sub(e));
                let on_axis = match arrow.dir {
                    Dir::Down | Dir::Up => {
                        p.0.abs_diff(arrow.tip.0) <= AT_ARROW.unsigned_abs()
                            && p.1 >= arrow.tip.1.min(arrow.base.1).saturating_sub(AT_ARROW)
                            && p.1 <= arrow.tip.1.max(arrow.base.1).saturating_add(AT_ARROW)
                    }
                    Dir::Left | Dir::Right => {
                        p.1.abs_diff(arrow.tip.1) <= AT_ARROW.unsigned_abs()
                            && p.0 >= arrow.tip.0.min(arrow.base.0).saturating_sub(AT_ARROW)
                            && p.0 <= arrow.tip.0.max(arrow.base.0).saturating_add(AT_ARROW)
                    }
                };
                let away = sign(other.0.saturating_sub(p.0)) == back.0 && back.0 != 0
                    || sign(other.1.saturating_sub(p.1)) == back.1 && back.1 != 0;
                if on_axis && away {
                    stack.push((k, e));
                }
            }
        }
        let mut visited: BTreeSet<(usize, usize)> = BTreeSet::new();
        let mut starts: BTreeSet<Start> = BTreeSet::new();
        let mut junctions = 0_u32;
        while let Some((k, entry)) = stack.pop() {
            if *steps > budget {
                break;
            }
            if !visited.insert((k, entry)) {
                continue;
            }
            let Some(seg) = self.segs.get(k) else { continue };
            let exit = 1_usize.saturating_sub(entry);
            let (from, here) = (seg.end(entry), seg.end(exit));
            let walk = (
                sign(here.0.saturating_sub(from.0)),
                sign(here.1.saturating_sub(from.1)),
            );
            let mut next = Vec::new();
            for (n, e) in self.ends_near(here, seg.color, steps) {
                if n == k {
                    continue;
                }
                let Some(ns) = self.segs.get(n) else { continue };
                let (near_end, far_end) = (ns.end(e), ns.end(1_usize.saturating_sub(e)));
                // Le trait voisin doit s'éloigner de `q` (un tronçon plus
                // court que l'écart de jonction n'y revient pas).
                let gap = |r: P| r.0.abs_diff(here.0).max(r.1.abs_diff(here.1));
                if gap(far_end) <= gap(near_end) {
                    continue;
                }
                let dir = (
                    sign(far_end.0.saturating_sub(near_end.0)),
                    sign(far_end.1.saturating_sub(near_end.1)),
                );
                // Retour en arrière sur un trait superposé (autre lien du même
                // départ) : ignoré.
                let dot = dir
                    .0
                    .saturating_mul(walk.0)
                    .saturating_add(dir.1.saturating_mul(walk.1));
                if dot < 0 {
                    continue;
                }
                next.push((n, e));
            }
            if !next.is_empty() {
                stack.extend(next);
                continue;
            }
            if self.at_arrow(here, seg.color, walk) {
                continue;
            }
            // Jonction en T : le bout tombe sur le corps d'un autre trait.
            let tee = self.segs.iter().enumerate().any(|(n, other)| {
                *steps = steps.saturating_add(1);
                n != k
                    && same_color(other.color, seg.color)
                    && dist_to_seg(here, other) <= JOIN
                    && !near(other.a, here, JOIN)
                    && !near(other.b, here, JOIN)
            });
            if tee {
                junctions = junctions.saturating_add(1);
            } else {
                starts.insert(Start { point: here, walk });
            }
        }
        (starts.into_iter().collect(), junctions)
    }

    /// Barres qui contiennent l'ordonnée `y` (à `tol` près).
    fn bars_at(&self, y: i64, tol: i64) -> impl Iterator<Item = &Bar> {
        self.bars
            .iter()
            .filter(move |b| y >= b.y0.saturating_sub(tol) && y <= b.y1.saturating_add(tol))
    }

    /// Côté du prédécesseur (fin = vrai) et doutes, d'après le départ ;
    /// `None` : départ vertical qui ne touche aucune barre ni forme (tracé
    /// coupé au bord de la page ou venu d'une tâche masquée).
    fn source_side(&self, start: &Start, doubts: &mut Vec<LinkDoubt>) -> Option<bool> {
        let q = start.point;
        let leaves_right = start.walk.0 < 0 && start.walk.1 == 0;
        let leaves_left = start.walk.0 > 0 && start.walk.1 == 0;
        let near_x = |x: i64| q.0.abs_diff(x) <= ON_BAR.unsigned_abs();
        if leaves_right || leaves_left {
            let anchored = self
                .bars_at(q.1, 60)
                .any(|b| if leaves_right { near_x(b.x1) } else { near_x(b.x0) });
            if !anchored {
                doubts.push(LinkDoubt::SourceOffBar);
            }
            return Some(leaves_right);
        }
        // Départ vertical (sous un jalon, d'une barre courte) : côté par la
        // barre touchée, sinon fin supposée.
        let touched: Vec<&Bar> = self
            .bars_at(q.1, 100)
            .filter(|b| q.0 >= b.x0.saturating_sub(100) && q.0 <= b.x1.saturating_add(100))
            .collect();
        if touched.is_empty() {
            return None;
        }
        let wide = touched
            .iter()
            .all(|b| b.x1.saturating_sub(b.x0) > ON_BAR.saturating_mul(2));
        if wide && touched.iter().all(|b| near_x(b.x0) && !near_x(b.x1)) {
            return Some(false);
        }
        if !(wide && touched.iter().all(|b| near_x(b.x1) && !near_x(b.x0))) {
            doubts.push(LinkDoubt::SourceSideGuessed);
        }
        Some(true)
    }

    /// Ligne et côté (fin = vrai) du successeur, d'après la pointe.
    fn target(&self, arrow: &Arrow, doubts: &mut Vec<LinkDoubt>) -> (Option<u32>, bool) {
        let (inside, finish) = match arrow.dir {
            Dir::Down => ((arrow.tip.0, arrow.tip.1.saturating_add(100)), false),
            Dir::Up => ((arrow.tip.0, arrow.tip.1.saturating_sub(100)), false),
            Dir::Right => (arrow.tip, false),
            Dir::Left => (arrow.tip, true),
        };
        let t = arrow.tip;
        let anchored = match arrow.dir {
            Dir::Right => self
                .bars_at(t.1, 60)
                .any(|b| t.0.abs_diff(b.x0) <= ON_BAR.unsigned_abs()),
            Dir::Left => self
                .bars_at(t.1, 60)
                .any(|b| t.0.abs_diff(b.x1) <= ON_BAR.unsigned_abs()),
            Dir::Down | Dir::Up => self.bars.iter().any(|b| {
                t.0 >= b.x0.saturating_sub(ON_BAR)
                    && t.0 <= b.x1.saturating_add(ON_BAR)
                    && (t.1.abs_diff(b.y0) <= 150 || t.1.abs_diff(b.y1) <= 150)
            }),
        };
        if !anchored {
            doubts.push(LinkDoubt::TargetOffBar);
        }
        (self.bands.row_at(inside.1), finish)
    }
}

const fn kind_of(source_finish: bool, target_finish: bool) -> LinkType {
    match (source_finish, target_finish) {
        (true, false) => LinkType::FinishToStart,
        (false, false) => LinkType::StartToStart,
        (true, true) => LinkType::FinishToFinish,
        (false, true) => LinkType::StartToFinish,
    }
}

/// Relit les liens dessinés des pages données ; `known` : N° des tâches
/// lues (un bout dans une autre ligne est hors du tableau).
#[must_use]
pub fn read(frames: &[PageFrame<'_>], known: &BTreeSet<u32>, limits: &LinkLimits) -> DrawnLinks {
    let mut out = DrawnLinks::default();
    let mut steps: u64 = 0;
    let mut best: BTreeMap<(u32, u32), DrawnLink> = BTreeMap::new();
    for frame in frames {
        let reader = PageReader::new(frame, limits, &mut out);
        let page_index = reader.page.index;
        for b in &reader.bars {
            if let Some(id) = reader.bands.row_at(half(b.y0, b.y1)) {
                out.rows_with_bars.insert(id);
            }
        }
        out.arrows = out
            .arrows
            .saturating_add(u32::try_from(reader.arrows.len()).unwrap_or(u32::MAX));
        for arrow in &reader.arrows {
            if steps > limits.max_steps {
                out.truncated = true;
                out.drop(LinkDrop::Bounded);
                continue;
            }
            let (starts, junctions) = reader.trace(arrow, &mut steps, limits.max_steps);
            let has_line = reader.segs.iter().any(|s| {
                same_color(s.color, arrow.color)
                    && (dist_to_seg(arrow.base, s) <= AT_ARROW || dist_to_seg(arrow.tip, s) <= AT_ARROW)
            });
            if !has_line {
                out.drop_at(LinkDrop::NoLine, page_index, arrow.tip);
                continue;
            }
            if starts.is_empty() {
                let why = if junctions > 0 {
                    LinkDrop::Junction
                } else {
                    LinkDrop::NoSource
                };
                out.drop_at(why, page_index, arrow.tip);
                continue;
            }
            let mut target_doubts = Vec::new();
            let (succ, target_finish) = reader.target(arrow, &mut target_doubts);
            let Some(succ) = succ.filter(|id| known.contains(id)) else {
                out.drop_at(LinkDrop::TargetOutside, page_index, arrow.tip);
                continue;
            };
            // Un départ par lien ; les bouts écartés d'une pointe qui a donné
            // au moins un lien (tracés d'autres liens parcourus en remontant)
            // ne sont pas comptés.
            let mut found: Vec<DrawnLink> = Vec::new();
            let mut first_drop: Option<(LinkDrop, P)> = None;
            for start in starts {
                let mut doubts = target_doubts.clone();
                let Some(source_finish) = reader.source_side(&start, &mut doubts) else {
                    first_drop.get_or_insert((LinkDrop::Cut, start.point));
                    continue;
                };
                let Some(pred) = reader.bands.row_at(start.point.1).filter(|id| known.contains(id)) else {
                    first_drop.get_or_insert((LinkDrop::SourceOutside, start.point));
                    continue;
                };
                if pred == succ {
                    first_drop.get_or_insert((LinkDrop::SelfLink, start.point));
                    continue;
                }
                doubts.sort_unstable();
                doubts.dedup();
                let penalty = doubts.iter().fold(0_u8, |a, d| a.saturating_add(d.penalty()));
                found.push(DrawnLink {
                    pred,
                    succ,
                    kind: kind_of(source_finish, target_finish),
                    confidence: 100_u8.saturating_sub(penalty),
                    doubts,
                    page: page_index,
                    from: start.point,
                    to: arrow.tip,
                });
            }
            if found.is_empty() {
                if let Some((why, at)) = first_drop {
                    out.drop_at(why, page_index, at);
                }
                continue;
            }
            for link in found {
                match best.get(&(link.succ, link.pred)) {
                    Some(old) if old.confidence >= link.confidence => out.drop(LinkDrop::Duplicate),
                    Some(_) => {
                        out.drop(LinkDrop::Duplicate);
                        best.insert((link.succ, link.pred), link);
                    }
                    None => {
                        best.insert((link.succ, link.pred), link);
                    }
                }
            }
        }
    }
    out.links = best.into_values().collect();
    out
}

#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::indexing_slicing,
        clippy::arithmetic_side_effects,
        reason = "tests : les interdits de la règle 10 ne s'appliquent pas, un échec doit paniquer"
    )]

    use super::*;
    use crate::extract::FilledRect;

    const BLUE: Rgb = Rgb([0, 0, 229]);
    const GREY: Rgb = Rgb([128, 128, 128]);
    /// Hauteur d'une ligne du tableau.
    const ROW: i32 = 1500;

    /// Centre de la ligne `k` (1 = première).
    fn center(k: i32) -> i32 {
        2000 + k * ROW - 280
    }

    fn rows(n: u32) -> Vec<RowRef> {
        (1..=n)
            .map(|id| RowRef {
                baseline: 2000 + i32::try_from(id).unwrap() * ROW,
                size: 800,
                id,
            })
            .collect()
    }

    /// Page de test : barres, traits et formes ajoutés par les cas.
    struct Draw {
        page: PageContent,
        serial: u32,
    }

    impl Draw {
        fn new() -> Self {
            Self {
                page: PageContent {
                    width: 120_000,
                    height: 80_000,
                    ..PageContent::default()
                },
                serial: 0,
            }
        }

        /// Barre de la ligne `k`, de `x0` à `x1`.
        fn bar(&mut self, k: i32, x0: i32, x1: i32) -> &mut Self {
            self.page.rects.push(FilledRect {
                x0,
                y0: center(k) - 300,
                x1,
                y1: center(k) + 300,
                color: GREY,
                is_rect: true,
            });
            self
        }

        /// Losange de jalon centré en `(x, centre de la ligne k)`.
        fn diamond(&mut self, k: i32, x: i32) -> &mut Self {
            let y = center(k);
            self.page.shapes.push(FilledShape {
                x0: x - 300,
                y0: y - 300,
                x1: x + 300,
                y1: y + 300,
                color: Rgb([0, 0, 0]),
                points: vec![(x, y - 300), (x + 300, y), (x, y + 300), (x - 300, y)],
                path: 0,
            });
            self
        }

        /// Polyligne de lien (points successifs), un sous-chemin.
        fn line(&mut self, points: &[(i32, i32)]) -> &mut Self {
            self.serial += 1;
            for w in points.windows(2) {
                let (a, b) = (w[0], w[1]);
                self.page.segments.push(Segment {
                    x0: a.0.min(b.0),
                    y0: a.1.min(b.1),
                    x1: a.0.max(b.0),
                    y1: a.1.max(b.1),
                    color: BLUE,
                    width: 12,
                    dashed: false,
                    path: self.serial,
                    rising: (b.0 - a.0) * (b.1 - a.1) < 0,
                    closed: false,
                });
            }
            self
        }

        /// Pointe de 3,84 pt de long dont la pointe est `tip`.
        fn arrow(&mut self, tip: (i32, i32), dir: Dir) -> &mut Self {
            let (x, y) = tip;
            let h = 384;
            let points = match dir {
                Dir::Down => vec![(x, y), (x + h, y - h), (x - h, y - h)],
                Dir::Up => vec![(x, y), (x - h, y + h), (x + h, y + h)],
                Dir::Right => vec![(x, y), (x - h, y + h), (x - h, y - h)],
                Dir::Left => vec![(x, y), (x + h, y - h), (x + h, y + h)],
            };
            let xs = points.iter().map(|p| p.0);
            let ys = points.iter().map(|p| p.1);
            self.page.shapes.push(FilledShape {
                x0: xs.clone().min().unwrap(),
                y0: ys.clone().min().unwrap(),
                x1: xs.max().unwrap(),
                y1: ys.max().unwrap(),
                color: BLUE,
                points,
                path: 0,
            });
            self
        }

        fn read(&self, n: u32) -> DrawnLinks {
            self.read_known(n, &(1..=n).collect())
        }

        fn read_known(&self, n: u32, known: &BTreeSet<u32>) -> DrawnLinks {
            let frame = PageFrame {
                page: &self.page,
                chart_left: 10_000,
                top: 500,
                rows: rows(n),
            };
            read(&[frame], known, &LinkLimits::default())
        }
    }

    fn pairs(d: &DrawnLinks) -> Vec<(u32, u32, LinkType)> {
        d.links.iter().map(|l| (l.pred, l.succ, l.kind)).collect()
    }

    /// Lien fin → début dessiné à la manière de MS Project : du bout de la
    /// barre vers la droite, coin, descente sur le haut du successeur.
    fn fs(d: &mut Draw, from: i32, p1: i32, to: i32, x: i32) {
        let top = center(to) - 300;
        d.line(&[(p1, center(from)), (x, center(from)), (x, top)])
            .arrow((x, top), Dir::Down);
    }

    #[test]
    fn finish_to_start_link_is_read_from_bar_end_to_arrow() {
        let mut d = Draw::new();
        d.bar(1, 12_000, 15_000).bar(2, 15_500, 18_000);
        fs(&mut d, 1, 15_000, 2, 15_600);
        let out = d.read(2);
        assert_eq!(pairs(&out), [(1, 2, LinkType::FinishToStart)]);
        assert_eq!(out.links[0].confidence, 100);
        assert_eq!(out.arrows, 1);
        // Déterminisme : même relevé.
        assert_eq!(out, d.read(2));
    }

    #[test]
    fn links_sharing_their_start_are_read_one_by_one() {
        // Deux liens d'un même prédécesseur, dessinés chacun en entier : le
        // début (sortie de la barre, descente) est superposé.
        let mut d = Draw::new();
        d.bar(1, 12_000, 15_000)
            .bar(2, 15_500, 18_000)
            .bar(3, 15_500, 16_000);
        fs(&mut d, 1, 15_000, 2, 15_600);
        fs(&mut d, 1, 15_000, 3, 15_600);
        let out = d.read(3);
        assert_eq!(
            pairs(&out),
            [(1, 2, LinkType::FinishToStart), (1, 3, LinkType::FinishToStart)]
        );
    }

    #[test]
    fn two_links_on_one_vertical_line_are_both_read() {
        // Deux prédécesseurs qui finissent au même endroit : leurs descentes
        // vers le même successeur se superposent sur le dernier trait.
        let mut d = Draw::new();
        d.bar(1, 12_000, 15_000)
            .bar(2, 13_000, 15_000)
            .bar(3, 15_500, 18_000);
        fs(&mut d, 1, 15_000, 3, 15_600);
        fs(&mut d, 2, 15_000, 3, 15_600);
        let out = d.read(3);
        assert_eq!(
            pairs(&out),
            [(1, 3, LinkType::FinishToStart), (2, 3, LinkType::FinishToStart)]
        );
    }

    #[test]
    fn t_junction_of_a_trunk_drawn_once_is_dropped_not_guessed() {
        // Tronçon commun dessiné une seule fois : la branche vers la ligne 2
        // part du milieu du tronçon (jonction en T) ; elle est écartée et
        // comptée, la descente jusqu'à la ligne 3 est lue.
        let mut d = Draw::new();
        d.bar(1, 12_000, 15_000)
            .bar(2, 17_000, 18_000)
            .bar(3, 15_500, 16_000);
        fs(&mut d, 1, 15_000, 3, 15_600);
        d.line(&[(15_600, center(2)), (17_000, center(2))])
            .arrow((17_000, center(2)), Dir::Right);
        let out = d.read(3);
        assert_eq!(pairs(&out), [(1, 3, LinkType::FinishToStart)]);
        assert_eq!(out.dropped.get(&LinkDrop::Junction), Some(&1));
    }

    #[test]
    fn arrow_into_the_finish_gives_finish_to_finish_and_start_to_finish() {
        let mut d = Draw::new();
        d.bar(1, 12_000, 15_000)
            .bar(2, 11_000, 14_000)
            .bar(3, 20_000, 22_000)
            .bar(4, 16_000, 19_000);
        // FF : de la fin de 1 vers la droite, descente, entrée par la fin de
        // 2 vers la gauche.
        d.line(&[
            (15_000, center(1)),
            (15_600, center(1)),
            (15_600, center(2)),
            (14_000, center(2)),
        ])
        .arrow((14_000, center(2)), Dir::Left);
        // DF : du début de 3 vers la gauche, descente, entrée par la fin
        // de 4 vers la gauche.
        d.line(&[
            (20_000, center(3)),
            (19_400, center(3)),
            (19_400, center(4)),
            (19_000, center(4)),
        ])
        .arrow((19_000, center(4)), Dir::Left);
        let out = d.read(4);
        assert_eq!(
            pairs(&out),
            [(1, 2, LinkType::FinishToFinish), (3, 4, LinkType::StartToFinish)]
        );
    }

    #[test]
    fn start_to_start_enters_the_start_of_the_successor() {
        let mut d = Draw::new();
        d.bar(1, 12_000, 15_000).bar(2, 13_000, 16_000);
        d.line(&[
            (12_000, center(1)),
            (11_400, center(1)),
            (11_400, center(2)),
            (13_000, center(2)),
        ])
        .arrow((13_000, center(2)), Dir::Right);
        assert_eq!(pairs(&d.read(2)), [(1, 2, LinkType::StartToStart)]);
    }

    #[test]
    fn milestone_start_is_read_with_a_guessed_side() {
        // Départ vertical sous un losange : côté supposé (fin), confiance
        // sous le seuil.
        let mut d = Draw::new();
        d.diamond(1, 14_000).bar(2, 14_200, 16_000);
        let top = center(2) - 300;
        d.line(&[(14_000, center(1) + 300), (14_000, top)])
            .arrow((14_000, top), Dir::Down);
        let out = d.read(2);
        assert_eq!(pairs(&out), [(1, 2, LinkType::FinishToStart)]);
        assert_eq!(out.links[0].doubts, [LinkDoubt::SourceSideGuessed]);
        assert!(out.links[0].doubtful());
    }

    #[test]
    fn link_to_a_task_absent_from_the_table_is_dropped_and_counted() {
        let mut d = Draw::new();
        d.bar(1, 12_000, 15_000).bar(2, 15_500, 18_000);
        fs(&mut d, 1, 15_000, 2, 15_600);
        // Ligne 2 sans tâche lue (filtrée, vide) : pointe hors du tableau.
        let only_one: BTreeSet<u32> = [1].into_iter().collect();
        let out = d.read_known(2, &only_one);
        assert_eq!(pairs(&out), []);
        assert_eq!(out.dropped.get(&LinkDrop::TargetOutside), Some(&1));
        // Départ hors du tableau : ligne 1 inconnue.
        let only_two: BTreeSet<u32> = [2].into_iter().collect();
        let out = d.read_known(2, &only_two);
        assert_eq!(out.dropped.get(&LinkDrop::SourceOutside), Some(&1));
    }

    #[test]
    fn path_coming_from_another_page_is_cut() {
        // Descente qui part du haut de la zone (lien venu de la page
        // précédente) : aucune barre au départ.
        let mut d = Draw::new();
        d.bar(2, 15_500, 18_000);
        let top = center(2) - 300;
        d.line(&[(15_600, 600), (15_600, top)])
            .arrow((15_600, top), Dir::Down);
        let out = d.read(2);
        assert_eq!(pairs(&out), []);
        assert_eq!(out.dropped.get(&LinkDrop::Cut), Some(&1));
    }

    #[test]
    fn hostile_page_is_bounded() {
        let mut d = Draw::new();
        d.bar(1, 12_000, 15_000).bar(2, 15_500, 18_000);
        fs(&mut d, 1, 15_000, 2, 15_600);
        for k in 0..30_000 {
            d.line(&[(20_000 + k % 1000, 3000), (20_000 + k % 1000, 3100 + k % 7)]);
        }
        let out = d.read(2);
        assert!(out.truncated);
        let limits = LinkLimits {
            max_steps: 10,
            ..LinkLimits::default()
        };
        let frame = PageFrame {
            page: &d.page,
            chart_left: 10_000,
            top: 500,
            rows: rows(2),
        };
        let out = read(&[frame], &[1, 2].into_iter().collect(), &limits);
        assert!(out.truncated);
    }

    #[test]
    fn grid_lines_and_bar_outlines_are_not_link_lines() {
        // Contour de barre de la couleur des liens, quadrillage gris : pas de
        // lien inventé.
        let mut d = Draw::new();
        d.bar(1, 12_000, 15_000).bar(2, 15_500, 18_000);
        fs(&mut d, 1, 15_000, 2, 15_600);
        d.line(&[
            (12_000, center(1) - 300),
            (15_000, center(1) - 300),
            (15_000, center(1) + 300),
            (12_000, center(1) + 300),
        ]);
        d.page.segments.push(Segment {
            x0: 14_000,
            y0: 500,
            x1: 14_000,
            y1: 70_000,
            color: GREY,
            width: 5,
            dashed: true,
            path: 99,
            rising: false,
            closed: false,
        });
        let out = d.read(2);
        assert_eq!(pairs(&out), [(1, 2, LinkType::FinishToStart)]);
    }
}
