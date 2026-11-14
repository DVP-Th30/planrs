//! Corpus de l'import PDF (ADR-0032, lot P1a).
//!
//! Corpus public : `tests/corpus-pdf/manifest.toml`, fichiers téléchargés
//! par `cargo xtask corpus fetch-pdf` (jamais commités). Chaque fichier
//! présent est importé : refus attendu (`expect` = code), ou import avec
//! exactement `tasks` tâches et au moins `min_dates_equal` dates calculées
//! égales aux dates lues. Un fichier absent ne compte ni pour ni contre ;
//! `PLANNERCRAFT_PDF_CORPUS=required` fait échouer le test si aucun n'est
//! présent.
//!
//! Plannings du propriétaire : `PLANNERCRAFT_PDF_SAMPLES=<dossier>` (hors
//! du dépôt) ; mesurés sous les noms R1, R2… (ordre des noms de fichier),
//! sans aucun texte des fichiers dans la sortie ; chacun doit s'importer.
//!
//! Tableau de mesure (Markdown) sur la sortie (`--nocapture`) et dans
//! `CARGO_TARGET_TMPDIR/pdf-corpus-report.md`.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::print_stdout,
    reason = "tests : les interdits de la règle 10 ne s'appliquent pas, un échec doit paniquer"
)]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use plannercraft_automation::text::Language;
use plannercraft_core::Moment;
use plannercraft_pdfimport::{
    ConvertOptions, Converted, DeduceLimits, Deduction, ImportOptions, ImportedPlan, NonWorkingDays,
    calendar, convert, deduce, import,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    file: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
struct Entry {
    name: String,
    url: String,
    expect: String,
    tasks: usize,
    min_dates_equal: u32,
}

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// Ligne de mesure d'un fichier importé.
struct Measure {
    tasks: usize,
    equal: u32,
    row: String,
}

fn measure(label: &str, bytes: &[u8]) -> Result<Measure, String> {
    let t0 = Instant::now();
    let plan: ImportedPlan = import(bytes, &ImportOptions::default()).map_err(|e| e.code().to_owned())?;
    let options = |pin| ConvertOptions {
        name: "corpus".to_owned(),
        now: "2026-11-13T08:00".parse::<Moment>().unwrap(),
        lang: Language::Fr,
        pin_read_starts: pin,
        non_working: Vec::new(),
        drawn_links: false,
    };
    let strict = convert(&plan, &options(false)).map_err(|e| e.to_string())?;
    let ms = t0.elapsed().as_millis();
    let pinned = convert(&plan, &options(true)).map_err(|e| e.to_string())?;
    let s = &plan.stats;
    let n = plan.tasks.len();
    let row = format!(
        "| {label} | {} | {} | {n} | {}{} | {}/{} | {} ({}) | {}/{n} · {}/{n} | {} | {}/{} | {}/{} | {ms} |",
        plan.source.pages,
        s.max_id,
        s.levels,
        if s.levels_from_wbs { " WBS" } else { "" },
        s.summaries_consistent,
        s.summaries_checked,
        s.links_read,
        s.links_dropped,
        s.starts_read,
        s.finishes_read,
        s.doubtful_rows,
        strict.dates_equal,
        strict.dates_compared,
        pinned.dates_equal,
        pinned.dates_compared,
    );
    Ok(Measure {
        tasks: n,
        equal: strict.dates_equal,
        row,
    })
}

const HEADER: &str = "| Fichier | Pages | N° max | Tâches lues | Niveaux | Récap. cohérentes | Liens (ignorés) | Débuts · fins relus | Douteuses | Dates égales (R-271) | Dates égales (--pin-starts) | ms (debug) |\n|---|---|---|---|---|---|---|---|---|---|---|---|\n";

#[test]
fn pdf_corpus() {
    let dir = repo().join("tests").join("corpus-pdf");
    let manifest: Manifest =
        toml::from_str(&std::fs::read_to_string(dir.join("manifest.toml")).unwrap()).unwrap();
    let mut report = String::from("## Corpus public\n\n");
    report.push_str(HEADER);
    let mut refused = String::from("\n### Refus\n\n| Fichier | Refus attendu | Obtenu |\n|---|---|---|\n");
    let mut present = 0_usize;
    let mut failures = Vec::new();
    for e in &manifest.file {
        let Ok(bytes) = std::fs::read(dir.join("files").join(&e.name)) else {
            continue;
        };
        present += 1;
        let label = format!("[{}]({})", e.name.trim_end_matches(".pdf"), e.url);
        match measure(&label, &bytes) {
            Ok(m) => {
                let _ = writeln!(report, "{}", m.row);
                if e.expect != "import" {
                    failures.push(format!("{} : importé, refus {} attendu", e.name, e.expect));
                } else if e.tasks > 0 && m.tasks != e.tasks {
                    failures.push(format!("{} : {} tâches, {} attendues", e.name, m.tasks, e.tasks));
                } else if m.equal < e.min_dates_equal {
                    failures.push(format!(
                        "{} : {} dates égales < plancher {}",
                        e.name, m.equal, e.min_dates_equal
                    ));
                }
            }
            Err(code) => {
                let _ = writeln!(refused, "| {label} | {} | {code} |", e.expect);
                if code != e.expect {
                    failures.push(format!("{} : {code}, attendu {}", e.name, e.expect));
                }
            }
        }
    }
    report.push_str(&refused);
    if let Ok(samples) = std::env::var("PLANNERCRAFT_PDF_SAMPLES") {
        report.push_str("\n## Plannings du propriétaire (R1 à Rn)\n\n");
        report.push_str(HEADER);
        let mut paths: Vec<PathBuf> = std::fs::read_dir(&samples)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("pdf")))
            .collect();
        paths.sort();
        for (k, p) in paths.iter().enumerate() {
            let label = format!("R{}", k + 1);
            match measure(&label, &std::fs::read(p).unwrap()) {
                Ok(m) => {
                    let _ = writeln!(report, "{}", m.row);
                }
                Err(code) => failures.push(format!("{label} : {code}")),
            }
        }
    }
    println!("{report}");
    std::fs::write(
        Path::new(env!("CARGO_TARGET_TMPDIR")).join("pdf-corpus-report.md"),
        &report,
    )
    .unwrap();
    if std::env::var("PLANNERCRAFT_PDF_CORPUS").as_deref() == Ok("required") {
        assert!(
            present > 0,
            "aucun fichier du corpus PDF : cargo xtask corpus fetch-pdf"
        );
    }
    assert!(failures.is_empty(), "{failures:#?}");
}

/// Conversion avec les jours chômés `non_working`.
fn convert_with(plan: &ImportedPlan, non_working: Vec<NonWorkingDays>) -> Converted {
    convert(
        plan,
        &ConvertOptions {
            name: "corpus".to_owned(),
            now: "2026-11-13T08:00".parse::<Moment>().unwrap(),
            lang: Language::Fr,
            pin_read_starts: false,
            non_working,
            drawn_links: false,
        },
    )
    .unwrap()
}

/// Périodes en clair, sans jour précis : mois du début (et de la fin s'il
/// diffère) et jours ouvrés.
fn periods_text(d: &Deduction) -> String {
    const MONTHS: [&str; 12] = [
        "janv.", "févr.", "mars", "avr.", "mai", "juin", "juil.", "août", "sept.", "oct.", "nov.", "déc.",
    ];
    use chrono::Datelike as _;
    let month = |day: plannercraft_core::Day| MONTHS[day.to_naive().month0() as usize];
    d.periods
        .iter()
        .map(|p| {
            let (from, to) = p.placement(p.default_option).unwrap();
            let (a, b) = (month(from), month(to));
            let m = if a == b { a.to_owned() } else { format!("{a}-{b}") };
            let kind = if p.exact() { "" } else { " à placer" };
            format!("{m} {} j{kind} ({} t.)", p.working_days, p.explained)
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// Ligne de mesure du calendrier (lot P1b) : dates égales dans quatre cas.
fn calendar_row(label: &str, bytes: &[u8]) -> Result<String, String> {
    let plan: ImportedPlan = import(bytes, &ImportOptions::default()).map_err(|e| e.code().to_owned())?;
    let limits = DeduceLimits::default();
    let standard = convert_with(&plan, Vec::new());
    let holidays = calendar::french_holidays(&plan, Language::Fr);
    let with_holidays = convert_with(&plan, holidays.clone());
    let t0 = Instant::now();
    let alone = deduce(&plan, standard.session.project(), &limits);
    let both = deduce(&plan, with_holidays.session.project(), &limits);
    let us = t0.elapsed().as_micros() / 2;
    let periods_alone = convert_with(
        &plan,
        calendar::periods(&calendar::default_placements(&alone.periods), "Congés"),
    );
    let mut all = holidays;
    all.extend(calendar::periods(
        &calendar::default_placements(&both.periods),
        "Congés",
    ));
    let periods_both = convert_with(&plan, all);
    let pct = |c: &Converted| format!("{}/{}", c.dates_equal, c.dates_compared);
    // Réglage par défaut de la fenêtre : fériés cochés si l'en-tête est
    // français ; périodes proposées sur ce calendrier-là.
    let french = calendar::suggests_french_holidays(&plan);
    let shown = if french { &both } else { &alone };
    Ok(format!(
        "| {label} | {} | {} | {} | {} ({}) | {} ({}) | {} | {}/{}/{} | {}/{}/{}/{} | {us} |",
        if french { "fr" } else { "en" },
        pct(&standard),
        pct(&with_holidays),
        pct(&periods_alone),
        alone.periods.len(),
        pct(&periods_both),
        both.periods.len(),
        periods_text(shown),
        shown.usable,
        shown.proven,
        shown.equations,
        shown.explained,
        shown.impossible,
        shown.ambiguous,
        shown.partial_days + shown.contradicted,
    ))
}

const CALENDAR_HEADER: &str = "| Fichier | Langue | Standard | Fériés | Périodes seules (n) | Fériés + périodes (n) | Périodes proposées (réglage par défaut) | Exploitables/preuves/équations | Expliquées/impossibles/ambiguës/écartées | µs déduction |\n|---|---|---|---|---|---|---|---|---|---|\n";

/// Mesure du calendrier (lot P1b, R-274 à R-277) : dates égales au jour
/// avec le calendrier Standard, les fériés français, les périodes déduites
/// seules, les deux. Sortie seulement (`--nocapture`), aucun plancher.
#[test]
fn pdf_corpus_calendar() {
    let dir = repo().join("tests").join("corpus-pdf");
    let manifest: Manifest =
        toml::from_str(&std::fs::read_to_string(dir.join("manifest.toml")).unwrap()).unwrap();
    let mut report = String::from("## Calendrier, corpus public\n\n");
    report.push_str(CALENDAR_HEADER);
    for e in manifest.file.iter().filter(|e| e.expect == "import") {
        let Ok(bytes) = std::fs::read(dir.join("files").join(&e.name)) else {
            continue;
        };
        if let Ok(row) = calendar_row(e.name.trim_end_matches(".pdf"), &bytes) {
            let _ = writeln!(report, "{row}");
        }
    }
    if let Ok(samples) = std::env::var("PLANNERCRAFT_PDF_SAMPLES") {
        report.push_str("\n## Calendrier, plannings du propriétaire\n\n");
        report.push_str(CALENDAR_HEADER);
        let mut paths: Vec<PathBuf> = std::fs::read_dir(&samples)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("pdf")))
            .collect();
        paths.sort();
        for (k, p) in paths.iter().enumerate() {
            let label = format!("R{}", k + 1);
            match calendar_row(&label, &std::fs::read(p).unwrap()) {
                Ok(row) => {
                    let _ = writeln!(report, "{row}");
                }
                Err(code) => {
                    let _ = writeln!(report, "| {label} | refus {code} |");
                }
            }
        }
    }
    println!("{report}");
}

/// Successeurs dont la cellule Prédécesseurs est en partie illisible
/// (coupée à l'impression, « … ») : un lien relu vers eux, absent de la
/// colonne, n'est pas vérifiable.
fn unverifiable(plan: &ImportedPlan) -> std::collections::BTreeSet<u32> {
    use plannercraft_pdfimport::WarningCode;
    plan.warnings
        .iter()
        .filter(|w| {
            matches!(
                w.code,
                WarningCode::PredecessorUnreadable | WarningCode::PredecessorExternal
            )
        })
        .filter_map(|w| w.id)
        .chain(
            plan.tasks
                .iter()
                .filter(|t| t.predecessors_text.contains('…') || t.predecessors_text.contains("..."))
                .map(|t| t.id),
        )
        .collect()
}

/// Liens de la colonne Prédécesseurs, complétés par la colonne Successeurs
/// quand le PDF en a une (cellules coupées à l'impression).
fn truth(plan: &ImportedPlan) -> std::collections::BTreeMap<(u32, u32), plannercraft_core::LinkType> {
    use plannercraft_automation::text::TextContext;
    let ctx = TextContext::for_options(&plannercraft_core::ProjectOptions::default(), Language::En);
    let ids: std::collections::BTreeSet<u32> = plan.tasks.iter().map(|t| t.id).collect();
    let mut column = std::collections::BTreeMap::new();
    for t in &plan.tasks {
        for p in &t.predecessors {
            column.insert((p.id, t.id), p.kind);
        }
        for s in plannercraft_pdfimport::values::predecessors(&t.successors_text, &ctx)
            .into_iter()
            .flatten()
        {
            if ids.contains(&s.id) {
                column.entry((t.id, s.id)).or_insert(s.kind);
            }
        }
    }
    column
}

/// Comparaison des liens relus dans le dessin aux liens de la colonne
/// Prédécesseurs (lot P1c) : (justes avec le type, justes sans le type,
/// relus vérifiables, colonne, colonne retrouvée, colonne même page,
/// retrouvée même page, invérifiables).
fn links_truth(plan: &ImportedPlan) -> [usize; 8] {
    use std::collections::{BTreeMap, BTreeSet};
    let page: BTreeMap<u32, usize> = plan.tasks.iter().map(|t| (t.id, t.page)).collect();
    let column = truth(plan);
    let drawn: BTreeSet<(u32, u32)> = plan.drawn.links.iter().map(|l| (l.pred, l.succ)).collect();
    let open = unverifiable(plan);
    let unknown = plan
        .drawn
        .links
        .iter()
        .filter(|l| !column.contains_key(&(l.pred, l.succ)) && open.contains(&l.succ))
        .count();
    let exact = plan
        .drawn
        .links
        .iter()
        .filter(|l| column.get(&(l.pred, l.succ)) == Some(&l.kind))
        .count();
    let pairs = plan
        .drawn
        .links
        .iter()
        .filter(|l| column.contains_key(&(l.pred, l.succ)))
        .count();
    let found = column.keys().filter(|k| drawn.contains(k)).count();
    // Même page, et une barre dessinée sur les deux lignes (sinon le lien
    // ne peut pas être dessiné : tâche hors de l'échelle de temps imprimée).
    let bars = &plan.drawn.rows_with_bars;
    let same: Vec<&(u32, u32)> = column
        .keys()
        .filter(|(a, b)| page.contains_key(a) && page.get(a) == page.get(b))
        .filter(|(a, b)| bars.contains(a) && bars.contains(b))
        .collect();
    let same_found = same.iter().filter(|k| drawn.contains(k)).count();
    [
        exact,
        pairs,
        plan.drawn.links.len() - unknown,
        column.len(),
        found,
        same.len(),
        same_found,
        unknown,
    ]
}

/// Relevé des liens dessinés d'un planning, en clair.
fn drawn_text(plan: &ImportedPlan) -> String {
    let d = &plan.drawn;
    let dropped: Vec<String> = d
        .dropped
        .iter()
        .map(|(k, n)| format!("{} {n}", k.code()))
        .collect();
    let doubtful = d.links.iter().filter(|l| l.doubtful()).count();
    format!(
        "{} | {} | {} | {}{}",
        d.arrows,
        d.links.len(),
        doubtful,
        dropped.join(", "),
        if d.truncated { " (partiel)" } else { "" }
    )
}

/// Temps de lecture (ms) sans le relevé des liens (bornes du relevé à
/// zéro) : la différence avec la lecture complète est le coût du relevé.
fn without_links_ms(bytes: &[u8]) -> u128 {
    let options = ImportOptions {
        limits: plannercraft_pdfimport::ImportLimits {
            links: plannercraft_pdfimport::LinkLimits {
                max_segments_per_page: 0,
                max_arrows_per_page: 0,
                max_steps: 0,
            },
            ..plannercraft_pdfimport::ImportLimits::default()
        },
        ..ImportOptions::default()
    };
    let t0 = Instant::now();
    let _ = import(bytes, &options);
    t0.elapsed().as_millis()
}

/// Mesure des liens dessinés (lot P1c) : précision et rappel contre la
/// colonne Prédécesseurs quand le PDF en a une ; relevé seul sinon.
#[test]
fn pdf_corpus_links() {
    let dir = repo().join("tests").join("corpus-pdf");
    let manifest: Manifest =
        toml::from_str(&std::fs::read_to_string(dir.join("manifest.toml")).unwrap()).unwrap();
    let mut report = String::from(
        "## Liens dessinés, corpus public\n\n| Fichier | Pointes | Liens relus | Douteux | Écartés | Invérifiables | Justes (type) | Justes (couple) | Colonne | Retrouvés | Colonne visible | Retrouvés visibles | ms (sans relevé) |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|\n",
    );
    let mut total = [0_usize; 8];
    for e in manifest.file.iter().filter(|e| e.expect == "import") {
        let Ok(bytes) = std::fs::read(dir.join("files").join(&e.name)) else {
            continue;
        };
        let t0 = Instant::now();
        let Ok(plan) = import(&bytes, &ImportOptions::default()) else {
            continue;
        };
        let ms = t0.elapsed().as_millis();
        let ms = format!("{ms} ({})", without_links_ms(&bytes));
        let m = links_truth(&plan);
        if m[3] > 0 {
            for (t, v) in total.iter_mut().zip(m) {
                *t += v;
            }
        }
        let _ = writeln!(
            report,
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {ms} |",
            e.name.trim_end_matches(".pdf"),
            drawn_text(&plan),
            m[7],
            m[0],
            m[1],
            m[3],
            m[4],
            m[5],
            m[6]
        );
        if std::env::var("PDFIMPORT_LINKS_DETAIL").is_ok_and(|v| e.name.contains(&v)) && m[3] > 0 {
            println!("FICHIER {}", e.name);
            let column = truth(&plan);
            let open = unverifiable(&plan);
            for l in &plan.drawn.links {
                let ok = match column.get(&(l.pred, l.succ)) {
                    Some(k) if *k == l.kind => "OK",
                    Some(_) => "TYPE",
                    None if open.contains(&l.succ) => "INVERIF",
                    None => "FAUX",
                };
                println!(
                    "  {} {}->{} {:?} c{} {:?} p{} {:?}->{:?}",
                    ok, l.pred, l.succ, l.kind, l.confidence, l.doubts, l.page, l.from, l.to
                );
            }
            let drawn: std::collections::BTreeSet<(u32, u32)> =
                plan.drawn.links.iter().map(|l| (l.pred, l.succ)).collect();
            for ((a, b), k) in &column {
                if !drawn.contains(&(*a, *b)) {
                    println!("  MANQUE {a}->{b} {k:?}");
                }
            }
            for d in &plan.drawn.dropped_at {
                println!("  ÉCARTÉ {:?} p{} {},{}", d.why, d.page, d.x, d.y);
            }
        }
    }
    let _ = writeln!(
        report,
        "\nTotal (fichiers à colonne) : justes avec le type {}/{} relus vérifiables ({} invérifiables), couples justes {}, colonne {}, retrouvés {}, visibles (même page, deux barres) {} retrouvés {}",
        total[0], total[2], total[7], total[1], total[3], total[4], total[5], total[6]
    );
    if let Ok(samples) = std::env::var("PLANNERCRAFT_PDF_SAMPLES") {
        report.push_str("\n## Liens dessinés, plannings du propriétaire\n\n| Planning | Pointes | Liens relus | Douteux | Écartés | ms (sans relevé) |\n|---|---|---|---|---|---|\n");
        let mut paths: Vec<PathBuf> = std::fs::read_dir(&samples)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("pdf")))
            .collect();
        paths.sort();
        for (k, p) in paths.iter().enumerate() {
            let bytes = std::fs::read(p).unwrap();
            let t0 = Instant::now();
            let plan = import(&bytes, &ImportOptions::default()).unwrap();
            let ms = format!("{} ({})", t0.elapsed().as_millis(), without_links_ms(&bytes));
            let _ = writeln!(report, "| R{} | {} | {ms} |", k + 1, drawn_text(&plan));
            if std::env::var("PDFIMPORT_LINKS_DETAIL").is_ok_and(|v| v == format!("R{}", k + 1)) {
                for l in &plan.drawn.links {
                    println!(
                        "  {}->{} {:?} c{} {:?} p{} {:?}->{:?}",
                        l.pred, l.succ, l.kind, l.confidence, l.doubts, l.page, l.from, l.to
                    );
                }
                for d in &plan.drawn.dropped_at {
                    println!("  ÉCARTÉ {:?} p{} {},{}", d.why, d.page, d.x, d.y);
                }
            }
        }
    }
    println!("{report}");
}

/// Planning sans sa colonne Prédécesseurs (mesure : liens du dessin à la
/// place de ceux de la colonne).
fn without_column(plan: &ImportedPlan) -> ImportedPlan {
    let mut p = plan.clone();
    for t in &mut p.tasks {
        t.predecessors.clear();
    }
    p.source
        .columns
        .retain(|c| c.kind != plannercraft_pdfimport::header::ColumnKind::Predecessors);
    p.stats.links_read = 0;
    p.stats.links_dropped = 0;
    p
}

fn convert_full(plan: &ImportedPlan, non_working: Vec<NonWorkingDays>, drawn: bool) -> Converted {
    convert(
        plan,
        &ConvertOptions {
            name: "corpus".to_owned(),
            now: "2026-11-13T08:00".parse::<Moment>().unwrap(),
            lang: Language::Fr,
            pin_read_starts: false,
            non_working,
            drawn_links: drawn,
        },
    )
    .unwrap()
}

/// Liens relus dont les dates lues se recouvrent (FD : successeur commencé
/// avant la fin du prédécesseur ; DD, FF, DF : bout d'arrivée avant le bout
/// de départ) : (recouvrements, liens datés).
fn overlaps(plan: &ImportedPlan) -> (usize, usize) {
    use plannercraft_core::LinkType as L;
    let by_id: std::collections::BTreeMap<u32, &plannercraft_pdfimport::ImportedTask> =
        plan.tasks.iter().map(|t| (t.id, t)).collect();
    let mut dated = 0;
    let mut over = 0;
    for l in plan.drawn.links.iter().filter(|l| !l.doubtful()) {
        let (Some(p), Some(s)) = (by_id.get(&l.pred), by_id.get(&l.succ)) else {
            continue;
        };
        let (from, to) = match l.kind {
            L::FinishToStart => (p.finish, s.start),
            L::StartToStart => (p.start, s.start),
            L::FinishToFinish => (p.finish, s.finish),
            L::StartToFinish => (p.start, s.finish),
        };
        let (Some(from), Some(to)) = (from, to) else {
            continue;
        };
        dated += 1;
        if to.day() < from.day() || (l.kind == L::FinishToStart && to.day() == from.day()) {
            over += 1;
        }
    }
    (over, dated)
}

/// Ligne de mesure de l'effet des liens dessinés sur les dates (lot P1c).
fn links_dates_row(label: &str, bytes: &[u8]) -> Option<String> {
    let plan: ImportedPlan = import(bytes, &ImportOptions::default()).ok()?;
    let column = plannercraft_pdfimport::convert::has_predecessor_column(&plan);
    let base = if column {
        without_column(&plan)
    } else {
        plan.clone()
    };
    if base.drawn.links.is_empty() {
        return None;
    }
    let holidays = calendar::french_holidays(&base, Language::Fr);
    let with_holidays = convert_full(&base, holidays.clone(), false);
    let both = deduce(&base, with_holidays.session.project(), &DeduceLimits::default());
    let mut all = holidays.clone();
    all.extend(calendar::periods(
        &calendar::default_placements(&both.periods),
        "Congés",
    ));
    let pct = |c: &Converted| format!("{}/{}", c.dates_equal, c.dates_compared);
    let t0 = Instant::now();
    let linked = convert_full(&base, holidays, true);
    let ms = t0.elapsed().as_millis();
    let linked_all = convert_full(&base, all.clone(), true);
    let plain_all = convert_full(&base, all, false);
    let standard = convert_full(&base, Vec::new(), false);
    let standard_linked = convert_full(&base, Vec::new(), true);
    let col = if column {
        let c = convert_full(&plan, Vec::new(), false);
        format!(" {}", pct(&c))
    } else {
        String::new()
    };
    let used = linked.drawn.clone().unwrap_or_default();
    let (over, dated) = overlaps(&base);
    Some(format!(
        "| {label} | {} | {} | {} | {} | {} | {} | {}{col} | {} | {} | {} | {}/{} | {}/{} | {ms} |",
        used.placed,
        used.refused_total(),
        pct(&with_holidays),
        pct(&linked),
        pct(&plain_all),
        pct(&linked_all),
        pct(&standard),
        pct(&standard_linked),
        used.negative_lags,
        used.constraints,
        over,
        dated,
        used.doubtful,
        base.drawn.links.len(),
    ))
}

const LINKS_DATES_HEADER: &str = "| Fichier | Liens posés | Refusés | Fériés sans liens | Fériés avec liens | Fériés + périodes sans liens | Fériés + périodes avec liens | Standard sans liens (colonne) | Standard avec liens | Décalages négatifs | Contraintes | Recouvrements lus/liens datés | Douteux non posés/relus | ms conversion |\n|---|---|---|---|---|---|---|---|---|---|---|---|---|---|\n";

/// Mesure de l'effet des liens dessinés sur les dates (lot P1c, R-278 à
/// R-280). Sortie seulement (`--nocapture`), aucun plancher.
#[test]
fn pdf_corpus_links_dates() {
    let dir = repo().join("tests").join("corpus-pdf");
    let manifest: Manifest =
        toml::from_str(&std::fs::read_to_string(dir.join("manifest.toml")).unwrap()).unwrap();
    let mut report = String::from("## Liens dessinés et dates, corpus public\n\n");
    report.push_str(LINKS_DATES_HEADER);
    for e in manifest.file.iter().filter(|e| e.expect == "import") {
        let Ok(bytes) = std::fs::read(dir.join("files").join(&e.name)) else {
            continue;
        };
        if let Some(row) = links_dates_row(e.name.trim_end_matches(".pdf"), &bytes) {
            let _ = writeln!(report, "{row}");
        }
    }
    if let Ok(samples) = std::env::var("PLANNERCRAFT_PDF_SAMPLES") {
        report.push_str("\n## Liens dessinés et dates, plannings du propriétaire\n\n");
        report.push_str(LINKS_DATES_HEADER);
        let mut paths: Vec<PathBuf> = std::fs::read_dir(&samples)
            .unwrap()
            .map(|e| e.unwrap().path())
            .filter(|p| p.extension().is_some_and(|x| x.eq_ignore_ascii_case("pdf")))
            .collect();
        paths.sort();
        for (k, p) in paths.iter().enumerate() {
            let label = format!("R{}", k + 1);
            let row = links_dates_row(&label, &std::fs::read(p).unwrap())
                .unwrap_or_else(|| format!("| {label} | aucun lien relu |"));
            let _ = writeln!(report, "{row}");
        }
    }
    println!("{report}");
}
