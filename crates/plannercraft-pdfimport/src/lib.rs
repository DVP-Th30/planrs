//! Import d'un planning Gantt reçu en PDF (ADR-0032, lot P1a).
//!
//! Typiquement l'impression PDF d'une vue Gantt de MS Project jointe à un
//! dossier de consultation : un tableau à gauche (N°, nom, durée, début,
//! fin, prédécesseurs…), le graphique à droite. Le PDF doit être
//! **vectoriel** (texte extractible) ; un scan, un PDF sans tableau
//! reconnaissable, sans dates ou à fausse hiérarchie est **refusé** avec
//! une erreur typée, jamais importé de travers.
//!
//! Pipeline (`docs/plans/pdf-import.md`) :
//! 1. [`extract`] : mots positionnés, aplats et traits par page
//!    (`hayro-interpret`, Rust pur) ; seul module à flottants, tout est
//!    ensuite en entiers (centièmes de point) ;
//! 2. [`lines`] : lignes de texte ;
//! 3. [`header`] : en-tête et colonnes (quadrillage, sinon projection) ;
//! 4. [`table`] : une ligne par tâche, noms repliés rattachés ;
//! 5. [`values`] : durées, dates, prédécesseurs (analyseurs de
//!    `plannercraft-automation`) ;
//! 6. [`plan`] : [`ImportedPlan`] neutre (dates **lues**, avertissements,
//!    confiance) ;
//! 7. [`convert`] : projet construit par les commandes du moteur, dates
//!    **calculées** par le moteur et comparées aux dates lues (règle 3) ;
//! 8. [`report`] : rapport lisible et JSON.
//!
//! Lot P1b : [`calendar`] (jours fériés français, R-274) et [`deduce`]
//! (périodes chômées déduites des écarts, R-276, R-277), posés comme
//! exceptions du calendrier du projet par [`convert`] (R-275).
//!
//! Aucune entrée/sortie : des octets en entrée. Compile en
//! `wasm32-unknown-unknown`.
#![forbid(unsafe_code)]

pub mod calendar;
pub mod convert;
pub mod deduce;
pub mod extract;
pub mod header;
pub mod lines;
pub mod links;
pub mod plan;
pub mod report;
pub mod table;
pub mod values;

pub use convert::{ConvertError, ConvertOptions, Converted, DateGap, NonWorkingDays, convert};
pub use deduce::{DeduceLimits, Deduction, ProposedPeriod, deduce};
pub use links::{DrawnLink, DrawnLinks, LinkDoubt, LinkDrop, LinkLimits};
pub use plan::{ImportedPlan, ImportedTask, Warning, WarningCode};
pub use values::NumericOrder;

/// Bornes de l'import (entrée hostile, règle 10).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImportLimits {
    /// Taille du document, en octets.
    pub max_bytes: usize,
    /// Bornes de l'extraction (pages, glyphes, aplats).
    pub extract: extract::ExtractLimits,
    /// Tâches importées au plus.
    pub max_tasks: usize,
    /// Bornes du relevé des liens dessinés (lot P1c).
    pub links: links::LinkLimits,
}

impl Default for ImportLimits {
    fn default() -> Self {
        Self {
            max_bytes: 64 * 1024 * 1024,
            extract: extract::ExtractLimits::default(),
            max_tasks: 20_000,
            links: links::LinkLimits::default(),
        }
    }
}

/// Options de l'import.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ImportOptions {
    /// Bornes.
    pub limits: ImportLimits,
    /// Ordre jour/mois imposé (sinon décidé sur le document).
    pub date_order: Option<NumericOrder>,
    /// Importer quand même un tableau dont la hiérarchie n'est pas lisible
    /// (liste sans retrait avec des titres en gras, ou retraits incohérents
    /// avec les dates ; sinon refus `pdfimport.flat_hierarchy` ou
    /// `pdfimport.inconsistent_hierarchy`) : tout au niveau 1.
    pub allow_flat_hierarchy: bool,
}

/// Refus du document entier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportError {
    /// Document plus grand que la borne.
    TooLarge,
    /// Document chiffré.
    Encrypted,
    /// Document illisible.
    Invalid,
    /// Le lecteur PDF a échoué (panique interceptée).
    ReaderFailure,
    /// Aucun texte extractible (polices sans table Unicode, dessin seul).
    NoText,
    /// Document numérisé (images pleine page, texte absent ou invisible).
    Scanned,
    /// Aucun en-tête de tableau Gantt reconnu (nom de tâche avec durée ou
    /// dates).
    NoTable,
    /// Tableau sans colonne ni valeur de date.
    NoDates,
    /// Liste sans retrait avec des titres en gras : hiérarchie non lisible
    /// sans interprétation.
    FlatHierarchy,
    /// Retraits incohérents avec les dates : la plupart des récapitulatives
    /// lues ne contiennent pas les dates de leurs enfants.
    InconsistentHierarchy,
    /// Tableau reconnu mais aucune tâche lue.
    NoTasks,
    /// Plus de tâches que la borne.
    TooManyTasks(usize),
}

impl ImportError {
    /// Code stable (`pdfimport.…`).
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::TooLarge => "pdfimport.too_large",
            Self::Encrypted => "pdfimport.encrypted",
            Self::Invalid => "pdfimport.invalid",
            Self::ReaderFailure => "pdfimport.reader_failure",
            Self::NoText => "pdfimport.no_text",
            Self::Scanned => "pdfimport.scanned",
            Self::NoTable => "pdfimport.no_table",
            Self::NoDates => "pdfimport.no_dates",
            Self::FlatHierarchy => "pdfimport.flat_hierarchy",
            Self::InconsistentHierarchy => "pdfimport.inconsistent_hierarchy",
            Self::NoTasks => "pdfimport.no_tasks",
            Self::TooManyTasks(_) => "pdfimport.too_many_tasks",
        }
    }

    /// Message pour l'utilisateur.
    #[must_use]
    pub fn message(self, lang: plannercraft_automation::text::Language) -> String {
        use plannercraft_automation::text::Language::{En, Fr};
        let s = match (self, lang) {
            (Self::TooLarge, Fr) => "PDF trop volumineux (64 Mo au plus)",
            (Self::TooLarge, En) => "PDF too large (64 MB at most)",
            (Self::Encrypted, Fr) => "PDF protégé par un mot de passe ou des droits : non lisible",
            (Self::Encrypted, En) => "PDF protected by a password or permissions: not readable",
            (Self::Invalid, Fr) => "fichier PDF illisible",
            (Self::Invalid, En) => "unreadable PDF file",
            (Self::ReaderFailure, Fr) => "le lecteur PDF a échoué sur ce fichier",
            (Self::ReaderFailure, En) => "the PDF reader failed on this file",
            (Self::NoText, Fr) => "aucun texte lisible dans ce PDF (dessin ou polices sans texte)",
            (Self::NoText, En) => "no readable text in this PDF (drawing or fonts without text)",
            (Self::Scanned, Fr) => {
                "PDF numérisé (image) : l'import lit seulement les PDF exportés depuis un logiciel de planning"
            }
            (Self::Scanned, En) => {
                "scanned PDF (image): import only reads PDFs exported from scheduling software"
            }
            (Self::NoTable, Fr) => {
                "aucun tableau de planning reconnu (en-tête « Nom de la tâche » avec « Durée », « Début » ou « Fin »)"
            }
            (Self::NoTable, En) => {
                "no schedule table found (a \"Task Name\" header with \"Duration\", \"Start\" or \"Finish\")"
            }
            (Self::NoDates, Fr) => {
                "le tableau ne contient pas de dates de début ou de fin : import impossible sans elles"
            }
            (Self::NoDates, En) => "the table has no start or finish dates: cannot import without them",
            (Self::FlatHierarchy, Fr) => {
                "liste sans retrait avec des titres en gras : la hiérarchie ne peut pas être lue sûrement"
            }
            (Self::FlatHierarchy, En) => "flat list with bold titles: the outline cannot be read reliably",
            (Self::InconsistentHierarchy, Fr) => {
                "les retraits des noms ne correspondent pas aux dates des récapitulatives : la hiérarchie ne peut pas être lue sûrement"
            }
            (Self::InconsistentHierarchy, En) => {
                "name indents do not match the summary dates: the outline cannot be read reliably"
            }
            (Self::NoTasks, Fr) => "tableau reconnu, mais aucune tâche lue",
            (Self::NoTasks, En) => "table found, but no task read",
            (Self::TooManyTasks(n), Fr) => return format!("{n} tâches : plus que la borne de l'import"),
            (Self::TooManyTasks(n), En) => return format!("{n} tasks: more than the import limit"),
        };
        s.to_owned()
    }
}

impl std::fmt::Display for ImportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message(plannercraft_automation::text::Language::Fr))
    }
}

impl std::error::Error for ImportError {}

/// Lit un PDF et rend le planning lu (sans calcul).
///
/// # Errors
///
/// [`ImportError`] : document refusé en entier.
pub fn import(bytes: &[u8], options: &ImportOptions) -> Result<ImportedPlan, ImportError> {
    if bytes.len() > options.limits.max_bytes {
        return Err(ImportError::TooLarge);
    }
    let extracted = extract::extract(bytes, &options.limits.extract).map_err(|e| match e {
        extract::ExtractError::Encrypted => ImportError::Encrypted,
        extract::ExtractError::Invalid => ImportError::Invalid,
        extract::ExtractError::ReaderFailure => ImportError::ReaderFailure,
    })?;
    plan::build(&extracted, options)
}

#[cfg(test)]
mod dev_dump {
    #![allow(
        clippy::unwrap_used,
        clippy::print_stdout,
        clippy::indexing_slicing,
        clippy::arithmetic_side_effects,
        reason = "outil de mise au point, test ignoré"
    )]

    /// `PDFIMPORT_DUMP=fichier.pdf cargo test -p plannercraft-pdfimport dump -- --ignored --nocapture`
    #[test]
    #[ignore = "outil de mise au point"]
    fn dump() {
        let path = std::env::var("PDFIMPORT_DUMP").unwrap();
        let bytes = std::fs::read(path).unwrap();
        let ex = crate::extract::extract(&bytes, &crate::extract::ExtractLimits::default()).unwrap();
        let page_no: usize = std::env::var("PDFIMPORT_PAGE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        println!(
            "pages {} producer {:?} creator {:?}",
            ex.page_count, ex.producer, ex.creator
        );
        if let Some(p) = ex.pages.get(page_no) {
            println!(
                "== page {} {}x{} glyphs {} unmapped {} img {} rects {} segs {}",
                p.index,
                p.width,
                p.height,
                p.glyphs,
                p.unmapped_glyphs,
                p.image_permille,
                p.rects.len(),
                p.segments.len()
            );
            let lines = crate::lines::group_lines(&p.words);
            let small = p
                .rects
                .iter()
                .filter(|r| !r.is_rect && r.x1 - r.x0 < 600 && r.y1 - r.y0 < 600)
                .count();
            let bars = p
                .rects
                .iter()
                .filter(|r| r.is_rect && r.y1 - r.y0 > 300 && r.y1 - r.y0 < 2000)
                .count();
            let mut colors: std::collections::BTreeMap<String, u32> = std::collections::BTreeMap::new();
            for r in p
                .rects
                .iter()
                .filter(|r| r.is_rect && r.y1 - r.y0 > 300 && r.y1 - r.y0 < 2000)
            {
                *colors.entry(r.color.hex()).or_default() += 1;
            }
            let vseg = p.segments.iter().filter(|s| s.is_vertical()).count();
            println!(
                "SHAPES small_nonrect {small} bars {bars} segs {} vertical {vseg} colors {colors:?}",
                p.segments.len()
            );
            if let Some(t) = crate::header::find_table(p, &lines) {
                for c in &t.columns {
                    println!(
                        "COL {:?} {} {}..{} ruled {}",
                        c.kind, c.header, c.x0, c.x1, c.ruled
                    );
                }
                println!(
                    "header_bottom {} table_bottom {:?}",
                    t.header_bottom, t.table_bottom
                );
            }
            let skip: usize = std::env::var("PDFIMPORT_SKIP")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0);
            for l in lines.iter().skip(skip).take(80) {
                let text: Vec<String> = l
                    .words
                    .iter()
                    .map(|w| format!("{}@{}{}", w.text, w.x0, if w.bold { "B" } else { "" }))
                    .collect();
                println!("{:>7} {}", l.baseline, text.join(" "));
            }
        }
    }

    /// Tracés d'une page (lot P1c) : traits par style, formes pleines.
    /// `PDFIMPORT_DUMP=fichier.pdf PDFIMPORT_PAGE=0 cargo test -p plannercraft-pdfimport dump_paths -- --ignored --nocapture`
    #[test]
    #[ignore = "outil de mise au point"]
    fn dump_paths() {
        let path = std::env::var("PDFIMPORT_DUMP").unwrap();
        let bytes = std::fs::read(path).unwrap();
        let ex = crate::extract::extract(&bytes, &crate::extract::ExtractLimits::default()).unwrap();
        let page_no: usize = std::env::var("PDFIMPORT_PAGE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        let Some(p) = ex.pages.get(page_no) else { return };
        let mut styles: std::collections::BTreeMap<(String, i32, bool, bool), u32> =
            std::collections::BTreeMap::new();
        for s in &p.segments {
            *styles
                .entry((s.color.hex(), s.width, s.dashed, s.is_vertical()))
                .or_default() += 1;
        }
        for (k, n) in &styles {
            println!("SEGSTYLE {k:?} {n}");
        }
        let mut shapes: std::collections::BTreeMap<(String, usize, i32, i32), u32> =
            std::collections::BTreeMap::new();
        for s in &p.shapes {
            *shapes
                .entry((s.color.hex(), s.points.len(), s.x1 - s.x0, s.y1 - s.y0))
                .or_default() += 1;
        }
        for (k, n) in &shapes {
            println!("SHAPE {k:?} {n}");
        }
        if let Ok(window) = std::env::var("PDFIMPORT_WINDOW") {
            let v: Vec<i32> = window.split(',').map(|x| x.parse().unwrap()).collect();
            let inside = |x: i32, y: i32| x >= v[0] && x <= v[2] && y >= v[1] && y <= v[3];
            for s in p
                .segments
                .iter()
                .filter(|s| inside(s.x0, s.y0) || inside(s.x1, s.y1))
            {
                println!(
                    "WSEG p{} {} {},{} -> {},{} w{} d{} r{}",
                    s.path,
                    s.color.hex(),
                    s.x0,
                    s.y0,
                    s.x1,
                    s.y1,
                    s.width,
                    s.dashed,
                    s.rising
                );
            }
            for s in p.shapes.iter().filter(|s| inside(s.x0, s.y0)) {
                println!("WSHP p{} {} {:?}", s.path, s.color.hex(), s.points);
            }
            for r in p
                .rects
                .iter()
                .filter(|r| inside(r.x0, r.y0) || inside(r.x1, r.y1))
            {
                println!(
                    "WRECT {} {},{} -> {},{} rect{}",
                    r.color.hex(),
                    r.x0,
                    r.y0,
                    r.x1,
                    r.y1,
                    r.is_rect
                );
            }
        }
        if let Ok(color) = std::env::var("PDFIMPORT_COLOR") {
            for s in p.segments.iter().filter(|s| s.color.hex() == color) {
                println!(
                    "SEG p{} {},{} -> {},{} w{}",
                    s.path, s.x0, s.y0, s.x1, s.y1, s.width
                );
            }
            for s in p.shapes.iter().filter(|s| s.color.hex() == color) {
                println!("SHP p{} {:?}", s.path, s.points);
            }
        }
    }

    /// `PDFIMPORT_DIR=dossier cargo test -p plannercraft-pdfimport run_dir -- --ignored --nocapture`
    #[test]
    #[ignore = "outil de mise au point"]
    fn run_dir() {
        let dir = std::env::var("PDFIMPORT_DIR").unwrap();
        let filter = std::env::var("PDFIMPORT_FILTER").unwrap_or_default();
        let mut paths: Vec<_> = std::fs::read_dir(dir)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        paths.sort();
        let now = plannercraft_core::Moment::from_minutes(0).unwrap();
        for path in paths {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            if !path.extension().is_some_and(|e| e.eq_ignore_ascii_case("pdf")) || !name.contains(&filter) {
                continue;
            }
            let bytes = std::fs::read(&path).unwrap();
            let t0 = std::time::Instant::now();
            let lang = plannercraft_automation::text::Language::Fr;
            match crate::import(&bytes, &crate::ImportOptions::default()) {
                Err(e) => println!("##### {name} : REFUS {} ({:?})", e.code(), t0.elapsed()),
                Ok(plan) => {
                    let conv = crate::convert(
                        &plan,
                        &crate::ConvertOptions {
                            name: name.clone(),
                            now,
                            lang,
                            pin_read_starts: std::env::var("PDFIMPORT_PIN").is_ok(),
                            non_working: Vec::new(),
                        },
                    );
                    println!("##### {name} ({:?})", t0.elapsed());
                    print!("{}", crate::report::text(&plan, conv.as_ref().ok(), lang));
                    if std::env::var("PDFIMPORT_TASKS").is_ok() {
                        for t in &plan.tasks {
                            println!(
                                "  {:>4} L{} {}{} | {} | {:?} | {:?} | {} | c{}",
                                t.id,
                                t.level,
                                if t.summary { "S " } else { "" },
                                t.name,
                                t.duration_text,
                                t.start.map(|m| m.to_string()),
                                t.finish.map(|m| m.to_string()),
                                t.predecessors_text,
                                t.confidence
                            );
                        }
                    }
                }
            }
        }
    }
}
