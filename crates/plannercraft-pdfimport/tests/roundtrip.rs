//! Aller-retour export PDF de PlannerCraft → import PDF (lot P1a) : un
//! planning construit par les commandes est exporté par
//! `project.export_pdf` (`plannercraft-pdf`), relu par
//! `plannercraft-pdfimport`, reconstruit, et le moteur doit retrouver les
//! mêmes dates (même calendrier : rien n'est inconnu).
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::arithmetic_side_effects,
    reason = "tests : les interdits de la règle 10 ne s'appliquent pas, un échec doit paniquer"
)]

use plannercraft_automation::command::params::ExportPdfParams;
use plannercraft_automation::text::Language;
use plannercraft_automation::{AnchorSpec, ApplyMode, Batch, NewProject, Session};
use plannercraft_core::{LinkType, Moment, TaskUid};
use plannercraft_pdfimport::{ConvertOptions, ImportOptions, WarningCode, convert, import};
use plannercraft_render::style::{TableColumn, TemplateId, template};

fn moment(s: &str) -> Moment {
    s.parse().unwrap()
}

/// Projet de test : deux récapitulatives, feuilles, jalon, liens FS, SS
/// avec décalage, FF, durée en semaines, nom long.
fn sample(lang: Language) -> Session {
    let mut s = Session::new_project(
        NewProject {
            name: "Essai".to_owned(),
            anchor: AnchorSpec::Start(moment("2026-03-02T08:00")),
            lang: Some(lang),
        },
        moment("2026-03-01T08:00"),
    )
    .unwrap();
    let batch = serde_json::json!({
        "format_version": 1,
        "lang": "fr",
        "ops": [
            {"op": "task.add", "name": "Gros œuvre", "as": "$go"},
            {"op": "task.add", "name": "Terrassement", "as": "$t", "at": {"last_child_of": "$go"}, "duration": {"text": "5j"}},
            {"op": "task.add", "name": "Fondations", "as": "$f", "at": {"last_child_of": "$go"}, "duration": {"text": "2s"}},
            {"op": "task.add", "name": "Élévation des murs du rez-de-chaussée", "as": "$m", "at": {"last_child_of": "$go"}, "duration": {"text": "8j"}},
            {"op": "task.add", "name": "Second œuvre", "as": "$so"},
            {"op": "task.add", "name": "Cloisons", "as": "$c", "at": {"last_child_of": "$so"}, "duration": {"text": "4j"}},
            {"op": "task.add", "name": "Électricité", "as": "$e", "at": {"last_child_of": "$so"}, "duration": {"text": "6j"}},
            {"op": "task.add", "name": "Réception", "as": "$r", "duration": {"text": "0j"}},
            {"op": "link.add", "pred": "$t", "succ": "$f"},
            {"op": "link.add", "pred": "$f", "succ": "$m"},
            {"op": "link.add", "pred": "$m", "succ": "$c"},
            {"op": "link.add", "pred": "$c", "succ": "$e", "kind": "ss", "lag": {"text": "+2j"}},
            {"op": "link.add", "pred": "$e", "succ": "$r"},
        ]
    });
    s.apply(&Batch::from_json(batch).unwrap(), ApplyMode::Commit)
        .unwrap();
    // Colonnes d'impression N°, Nom, Durée, Début, Fin (H-UI-68).
    let mut page = template(TemplateId::Classique).page;
    page.columns = Some(TableColumn::PRINT_DEFAULT.to_vec());
    let style = serde_json::json!({
        "format_version": 1,
        "ops": [{"op": "style.apply", "patch": {"page": serde_json::to_value(page).unwrap()}}]
    });
    s.apply(&Batch::from_json(style).unwrap(), ApplyMode::Commit)
        .unwrap();
    s
}

fn export(s: &Session, lang: Language) -> Vec<u8> {
    let params: ExportPdfParams = serde_json::from_value(serde_json::json!({ "path": "essai.pdf" })).unwrap();
    plannercraft_automation::pdf::export(s, &params, lang)
        .unwrap()
        .bytes
}

#[test]
fn plannercraft_export_reimports_with_same_engine_dates() {
    let original = sample(Language::Fr);
    let bytes = export(&original, Language::Fr);
    if let Ok(dir) = std::env::var("PDFIMPORT_KEEP") {
        std::fs::write(std::path::Path::new(&dir).join("roundtrip-fr.pdf"), &bytes).unwrap();
    }
    let plan = import(&bytes, &ImportOptions::default()).unwrap();
    let names: Vec<&str> = plan.tasks.iter().map(|t| t.name.as_str()).collect();
    assert_eq!(
        names,
        [
            "Gros œuvre",
            "Terrassement",
            "Fondations",
            // Coupé par l'export (colonne étroite) : signalé.
            "Élévation des murs du rez-de-chaus…",
            "Second œuvre",
            "Cloisons",
            "Électricité",
            "Réception",
        ]
    );
    assert!(
        plan.warnings
            .iter()
            .any(|w| w.code == WarningCode::NameTruncated && w.id == Some(4))
    );
    let levels: Vec<u16> = plan.tasks.iter().map(|t| t.level).collect();
    assert_eq!(levels, [1, 2, 2, 2, 1, 2, 2, 1]);
    assert!(plan.tasks[0].summary && plan.tasks[4].summary);
    assert!(plan.tasks[7].milestone());
    let converted = convert(
        &plan,
        &ConvertOptions {
            name: "essai".to_owned(),
            now: moment("2026-03-01T08:00"),
            lang: Language::Fr,
            pin_read_starts: false,
            non_working: Vec::new(),
            drawn_links: false,
        },
    )
    .unwrap();
    assert!(converted.gaps.is_empty(), "{:?}", converted.gaps);
    assert_eq!(converted.dates_equal, converted.dates_compared);
    // Dates du moteur identiques à celles du projet d'origine, tâche par
    // tâche (UID dans l'ordre de création des deux côtés) ; l'heure d'un
    // jalon sans lien n'est pas imprimée (le jour seulement) : comparée au
    // jour.
    for (k, uid) in converted.uids.iter().enumerate() {
        let original_uid = TaskUid::new(u32::try_from(k).unwrap() + 1);
        let a = original.schedule().scheduled(original_uid).unwrap();
        let b = converted.session.schedule().scheduled(*uid).unwrap();
        if plan.tasks[k].milestone() {
            assert_eq!(
                (a.start.day(), a.finish.day()),
                (b.start.day(), b.finish.day()),
                "tâche {k}"
            );
        } else {
            assert_eq!((a.start, a.finish), (b.start, b.finish), "tâche {k}");
        }
    }
}

/// Grand projet en anglais : plusieurs pages, tableau continué, dates
/// MM/JJ ; deux lectures donnent le même résultat (déterminisme).
#[test]
fn long_english_export_spans_pages_and_is_deterministic() {
    let mut s = Session::new_project(
        NewProject {
            name: "Long".to_owned(),
            anchor: AnchorSpec::Start(moment("2026-01-05T08:00")),
            lang: Some(Language::En),
        },
        moment("2026-01-01T08:00"),
    )
    .unwrap();
    let mut ops = Vec::new();
    for phase in 0..6 {
        ops.push(serde_json::json!({"op": "task.add", "name": format!("Phase {phase}"), "as": format!("$p{phase}")}));
        for k in 0..20 {
            let label = format!("$t{phase}_{k}");
            ops.push(serde_json::json!({
                "op": "task.add", "name": format!("Task {phase}.{k}"), "as": label,
                "at": {"last_child_of": format!("$p{phase}")},
                "duration": {"text": format!("{}d", 1 + (k % 4))}
            }));
            if k > 0 {
                ops.push(serde_json::json!({"op": "link.add", "pred": format!("$t{phase}_{}", k - 1), "succ": label}));
            } else if phase > 0 {
                ops.push(serde_json::json!({"op": "link.add", "pred": format!("$t{}_19", phase - 1), "succ": label}));
            }
        }
    }
    let batch = serde_json::json!({"format_version": 1, "lang": "en", "ops": ops});
    s.apply(&Batch::from_json(batch).unwrap(), ApplyMode::Commit)
        .unwrap();
    let mut page = template(TemplateId::Classique).page;
    page.columns = Some(TableColumn::PRINT_DEFAULT.to_vec());
    let style = serde_json::json!({
        "format_version": 1,
        "ops": [{"op": "style.apply", "patch": {"page": serde_json::to_value(page).unwrap()}}]
    });
    s.apply(&Batch::from_json(style).unwrap(), ApplyMode::Commit)
        .unwrap();
    let bytes = export(&s, Language::En);
    if let Ok(dir) = std::env::var("PDFIMPORT_KEEP") {
        std::fs::write(std::path::Path::new(&dir).join("roundtrip-en.pdf"), &bytes).unwrap();
    }
    let plan = import(&bytes, &ImportOptions::default()).unwrap();
    assert!(plan.source.table_pages.len() > 1, "{:?}", plan.source.table_pages);
    assert_eq!(plan.tasks.len(), 126);
    assert_eq!(plan.stats.max_id, 126);
    assert_eq!(plan.stats.durations_read, 126);
    assert_eq!(plan.stats.starts_read, 126);
    assert_eq!(plan.stats.finishes_read, 126);
    assert_eq!(plan.tasks.iter().filter(|t| t.summary).count(), 6);
    assert_eq!(plan.tasks.iter().filter(|t| t.level == 2).count(), 120);
    assert_eq!(plan.stats.summaries_consistent, 6);
    let again = import(&bytes, &ImportOptions::default()).unwrap();
    assert_eq!(plan, again);
    let converted = convert(
        &plan,
        &ConvertOptions {
            name: "long".to_owned(),
            now: moment("2026-01-01T08:00"),
            lang: Language::En,
            pin_read_starts: false,
            non_working: Vec::new(),
            drawn_links: false,
        },
    )
    .unwrap();
    assert!(converted.gaps.is_empty(), "{:?}", converted.gaps.first());
}

/// Lot P1c : un planning exporté avec des liens des quatre types, sans
/// colonne Prédécesseurs imprimée, rend ses liens à l'import (relus dans le
/// dessin) ; posés par le moteur, ils redonnent les dates d'origine.
#[test]
fn drawn_links_of_the_four_types_come_back_from_the_export() {
    let mut s = Session::new_project(
        NewProject {
            name: "Liens".to_owned(),
            anchor: AnchorSpec::Start(moment("2026-03-02T08:00")),
            lang: Some(Language::Fr),
        },
        moment("2026-03-01T08:00"),
    )
    .unwrap();
    let batch = serde_json::json!({
        "format_version": 1,
        "lang": "fr",
        "ops": [
            {"op": "task.add", "name": "Terrassement", "as": "$a", "duration": {"text": "5j"}},
            {"op": "task.add", "name": "Fondations", "as": "$b", "duration": {"text": "4j"}},
            {"op": "task.add", "name": "Réseaux", "as": "$c", "duration": {"text": "3j"}},
            {"op": "task.add", "name": "Dallage", "as": "$d", "duration": {"text": "6j"}},
            {"op": "task.add", "name": "Murs", "as": "$e", "duration": {"text": "8j"}},
            {"op": "task.add", "name": "Étanchéité", "as": "$f", "duration": {"text": "2j"}},
            {"op": "task.add", "name": "Réception", "as": "$g", "duration": {"text": "0j"}},
            {"op": "link.add", "pred": "$a", "succ": "$b"},
            {"op": "link.add", "pred": "$b", "succ": "$c", "kind": "ss"},
            {"op": "link.add", "pred": "$b", "succ": "$d"},
            {"op": "link.add", "pred": "$d", "succ": "$e", "kind": "ff"},
            {"op": "link.add", "pred": "$a", "succ": "$f", "kind": "sf"},
            {"op": "link.add", "pred": "$f", "succ": "$g"},
            {"op": "link.add", "pred": "$e", "succ": "$g"},
        ]
    });
    s.apply(&Batch::from_json(batch).unwrap(), ApplyMode::Commit)
        .unwrap();
    let mut page = template(TemplateId::Classique).page;
    page.columns = Some(TableColumn::PRINT_DEFAULT.to_vec());
    let style = serde_json::json!({
        "format_version": 1,
        "ops": [{"op": "style.apply", "patch": {"page": serde_json::to_value(page).unwrap()}}]
    });
    s.apply(&Batch::from_json(style).unwrap(), ApplyMode::Commit)
        .unwrap();
    let bytes = export(&s, Language::Fr);
    if let Ok(dir) = std::env::var("PDFIMPORT_KEEP") {
        std::fs::write(std::path::Path::new(&dir).join("roundtrip-links.pdf"), &bytes).unwrap();
    }
    let plan = import(&bytes, &ImportOptions::default()).unwrap();
    assert!(!plannercraft_pdfimport::convert::has_predecessor_column(&plan));
    let read: Vec<(u32, u32, LinkType)> = plan
        .drawn
        .links
        .iter()
        .map(|l| (l.pred, l.succ, l.kind))
        .collect();
    assert_eq!(
        read,
        [
            (1, 2, LinkType::FinishToStart),
            (2, 3, LinkType::StartToStart),
            (2, 4, LinkType::FinishToStart),
            (4, 5, LinkType::FinishToFinish),
            (1, 6, LinkType::StartToFinish),
            (5, 7, LinkType::FinishToStart),
            (6, 7, LinkType::FinishToStart),
        ],
        "{:?} {:?}",
        plan.drawn.dropped,
        plan.drawn.dropped_at
    );
    assert!(
        plan.drawn.links.iter().all(|l| !l.doubtful()),
        "{:#?}",
        plan.drawn.links
    );
    let converted = convert(
        &plan,
        &ConvertOptions {
            name: "liens".to_owned(),
            now: moment("2026-03-01T08:00"),
            lang: Language::Fr,
            pin_read_starts: false,
            non_working: Vec::new(),
            drawn_links: true,
        },
    )
    .unwrap();
    assert_eq!(converted.drawn.as_ref().unwrap().placed, 7);
    assert!(converted.gaps.is_empty(), "{:?}", converted.gaps);
    for (k, uid) in converted.uids.iter().enumerate() {
        let original_uid = TaskUid::new(u32::try_from(k).unwrap() + 1);
        let a = s.schedule().scheduled(original_uid).unwrap();
        let b = converted.session.schedule().scheduled(*uid).unwrap();
        assert_eq!(
            (a.start.day(), a.finish.day()),
            (b.start.day(), b.finish.day()),
            "tâche {k}"
        );
    }
}
