//! Fichier → Importer un PDF… (lot P1b, ADR-0033, H-UI-205 à H-UI-212) :
//! sélecteur scripté (`FileService::pick_bytes`), aperçu calculé hors du
//! fil d'interface, réglages, calendrier (fériés français, périodes
//! déduites), puis « Importer » = nouveau document non enregistré. Lot P1c
//! (ADR-0034, H-UI-213 à H-UI-215) : liens relus dans le dessin.
//!
//! Le PDF des tests est **produit par PlannerCraft** (export PDF d'un
//! planning construit par les commandes, avec un férié et trois semaines
//! de congés dans son calendrier) : aucun fichier réel.

use std::collections::VecDeque;
use std::sync::mpsc::Sender;

use egui_kittest::Harness;
use egui_kittest::kittest::{NodeT as _, Queryable as _};
use plannercraft_automation::command::params::ExportPdfParams;
use plannercraft_automation::text::Language as TextLanguage;
use plannercraft_automation::{AnchorSpec, ApplyMode, Batch, NewProject, Session};
use plannercraft_core::calendar::DaySetting;
use plannercraft_core::{Day, Moment};
use plannercraft_pdfimport::header::ColumnKind;
use plannercraft_pdfimport::{ImportError, ImportedPlan, NumericOrder, Warning, WarningCode};
use plannercraft_render::style::{TableColumn, TemplateId, template};
use plannercraft_ui_egui::action::DialogKind;
use plannercraft_ui_egui::dialog::DialogBody;
use plannercraft_ui_egui::dialog::pdf_import::{
    ComputeInput, DRAWN_LINKS_DEFAULT, PdfImportState, PdfSettings, PreviewError, VisibleSettings, compute,
    drawn_counts, import_limits, refusal_hint, visible_settings, warning_text_id,
};
use plannercraft_ui_egui::i18n::{Arg, Language, TextId, Texts};
use plannercraft_ui_egui::io::{OpenEvent, PickRequest, PickedFile};
use plannercraft_ui_egui::{Action, App, Env, FileService, LanguagePref, ThemePref};

use crate::{fr, fr_light, harness_for, now, open_demo};

/// Sélecteur de test : rend les fichiers du script, dans l'ordre ; vérifie
/// le filtre des PDF.
struct PickedFiles(VecDeque<PickedFile>);

impl FileService for PickedFiles {
    fn pick_and_open(&mut self, ctx: &egui::Context, _: Moment, _: String, _: String, tx: Sender<OpenEvent>) {
        tx.send(OpenEvent::Cancelled).unwrap();
        ctx.request_repaint();
    }

    fn pick_bytes(&mut self, ctx: &egui::Context, request: PickRequest, tx: Sender<PickedFile>) {
        assert_eq!(request.extensions, ["pdf"]);
        tx.send(self.0.pop_front().unwrap_or(PickedFile::Cancelled))
            .unwrap();
        ctx.request_repaint();
    }
}

fn picked(name: &str, bytes: &[u8]) -> PickedFile {
    PickedFile::Picked {
        name: name.to_owned(),
        result: Ok(bytes.to_vec()),
    }
}

fn moment(s: &str) -> Moment {
    s.parse().unwrap()
}

fn day(s: &str) -> Day {
    s.parse().unwrap()
}

/// Planning de chantier exporté en PDF par PlannerCraft (colonnes N°, Nom,
/// Durée, Début, Fin) : calendrier avec le 14 juillet chômé et des congés
/// du 3 au 21 août 2026 ; l'élévation enjambe les congés, les réseaux et la
/// charpente les encadrent (jours prouvés travaillés).
pub fn sample_pdf() -> Vec<u8> {
    let mut s = Session::new_project(
        NewProject {
            name: "Chantier".to_owned(),
            anchor: AnchorSpec::Start(moment("2026-07-06T08:00")),
            lang: Some(TextLanguage::Fr),
        },
        moment("2026-07-01T08:00"),
    )
    .unwrap();
    let batch = serde_json::json!({
        "format_version": 1,
        "lang": "fr",
        "ops": [
            {"op": "calendar.set_exception", "cal": "project", "from": "2026-07-14", "to": "2026-07-14", "ranges": [], "name": "Fête nationale"},
            {"op": "calendar.set_exception", "cal": "project", "from": "2026-08-03", "to": "2026-08-21", "ranges": [], "name": "Congés"},
            {"op": "task.add", "name": "Gros œuvre", "as": "$go"},
            {"op": "task.add", "name": "Terrassement", "as": "$t", "at": {"last_child_of": "$go"}, "duration": {"text": "5j"}},
            {"op": "task.add", "name": "Fondations", "as": "$f", "at": {"last_child_of": "$go"}, "duration": {"text": "10j"}},
            {"op": "task.add", "name": "Élévation", "as": "$e", "at": {"last_child_of": "$go"}, "duration": {"text": "24j"}},
            {"op": "task.add", "name": "Réseaux", "as": "$r", "at": {"last_child_of": "$go"}, "duration": {"text": "4j"}},
            {"op": "task.add", "name": "Charpente", "as": "$c", "at": {"last_child_of": "$go"}, "duration": {"text": "20j"}},
            {"op": "task.add", "name": "Second œuvre", "as": "$so"},
            {"op": "task.add", "name": "Cloisons", "as": "$cl", "at": {"last_child_of": "$so"}, "duration": {"text": "10j"}},
            {"op": "task.add", "name": "Électricité", "as": "$el", "at": {"last_child_of": "$so"}, "duration": {"text": "5j"}},
            {"op": "task.add", "name": "Réception", "as": "$rc", "duration": {"text": "0j"}},
            {"op": "link.add", "pred": "$t", "succ": "$f"},
            {"op": "link.add", "pred": "$f", "succ": "$e"},
            {"op": "link.add", "pred": "$e", "succ": "$r", "kind": "ss"},
            {"op": "link.add", "pred": "$r", "succ": "$c"},
            {"op": "link.add", "pred": "$e", "succ": "$cl"},
            {"op": "link.add", "pred": "$cl", "succ": "$el"},
            {"op": "link.add", "pred": "$el", "succ": "$rc"},
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
    let params: ExportPdfParams =
        serde_json::from_value(serde_json::json!({ "path": "chantier.pdf" })).unwrap();
    plannercraft_automation::pdf::export(&s, &params, TextLanguage::Fr)
        .unwrap()
        .bytes
}

/// Harnais dont le sélecteur rend `files`.
fn harness_with(files: Vec<PickedFile>) -> Harness<'static, App> {
    let env = Env {
        files: Box::new(PickedFiles(files.into())),
        ..Env::fixed(now())
    };
    harness_for(App::new(fr_light(), env))
}

/// Fenêtre ouverte.
fn state(app: &App) -> &PdfImportState {
    match app.view().dialog.as_ref().map(|d| (&d.kind, &d.body)) {
        Some((DialogKind::PdfImport, DialogBody::PdfImport(state))) => state,
        other => panic!("aperçu de l'import PDF non ouvert : {other:?}"),
    }
}

/// Attend la fin du calcul (fil dédié), 20 s au plus.
pub fn settle(harness: &mut Harness<'static, App>) {
    for _ in 0..1_000 {
        harness.run();
        if !state(harness.state()).busy() {
            // La fenêtre prend sa taille en quelques images (zone centrée).
            for _ in 0..6 {
                harness.step();
            }
            harness.run();
            return;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    panic!("calcul de l'import PDF non terminé");
}

/// Clique l'entrée du menu Fichier dont le libellé commence par `item`.
fn file_menu(harness: &mut Harness<'static, App>, item: TextId) {
    crate::click(harness, &fr(TextId::MenuFile));
    let label = fr(item);
    harness
        .get_by(|node| node.label().is_some_and(|text| text.starts_with(&label)))
        .click();
    harness.run();
}

/// Clique le nœud dont le libellé commence par `prefix` (case, bouton).
fn click_prefix(harness: &mut Harness<'static, App>, prefix: &str) {
    harness
        .get_by(|node| node.label().is_some_and(|text| text.starts_with(prefix)))
        .click();
    harness.run();
}

fn fr_args(id: TextId, args: &[(&str, Arg<'_>)]) -> String {
    Texts::new(Language::Fr).get_args(id, args)
}

/// Libellé du compteur « Dates identiques au PDF : n sur m ».
fn same(n: i64, of: i64) -> String {
    fr_args(
        TextId::PdfSame,
        &[("n", Arg::Count(n)), ("count", Arg::Count(of))],
    )
}

/// H-UI-205 : Fichier → Importer un PDF…, sans document ; le sélecteur ne
/// propose que les PDF ; l'entrée est aussi dans la palette ; un fichier
/// refermé n'ouvre rien.
#[test]
fn h_ui_205_menu_opens_picker_and_preview() {
    let mut harness = harness_with(vec![
        PickedFile::Cancelled,
        picked("planning-chantier.pdf", &sample_pdf()),
    ]);
    harness.state_mut().dispatch(Action::ImportPdf);
    harness.run();
    assert!(harness.state().view().dialog.is_none());
    file_menu(&mut harness, TextId::MenuImportPdf);
    assert_eq!(state(harness.state()).name, "planning-chantier.pdf");
    settle(&mut harness);
    assert!(harness.query_by_label(&fr(TextId::PdfTitle)).is_some());
    assert!(harness.query_by_label(&fr(TextId::PdfLocal)).is_some());
    harness.key_press(egui::Key::Escape);
    harness.run();
    assert!(harness.state().view().dialog.is_none());
    assert!(harness.state().document.is_none());
}

/// H-UI-206 : résumé (tâches, niveaux, liens, colonnes reconnues), tableau
/// des tâches lues (retrait par niveau, fin calculée, écart) et filtre
/// « écarts seulement ».
#[test]
fn h_ui_206_summary_table_and_gap_filter() {
    let mut harness = harness_with(vec![picked("planning-chantier.pdf", &sample_pdf())]);
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    let s = state(harness.state());
    let p = s.preview().expect("aperçu");
    assert_eq!(p.rows.len(), 10);
    let levels: Vec<u16> = p.rows.iter().map(|r| r.level).collect();
    assert_eq!(levels, [1, 2, 2, 2, 2, 2, 1, 2, 2, 1]);
    let summary = [
        fr_args(TextId::PdfSummaryTasks, &[("count", Arg::Count(10))]),
        fr_args(TextId::PdfSummaryLevels, &[("count", Arg::Count(2))]),
        fr_args(TextId::PdfSummaryLinks, &[("count", Arg::Count(0))]),
    ]
    .join(", ");
    assert!(
        harness.query_by_label(&summary).is_some(),
        "résumé absent : {summary}"
    );
    let plan = s.shown.as_ref().unwrap().plan.0.as_ref().as_ref().unwrap();
    let kinds: Vec<ColumnKind> = plan.source.columns.iter().map(|c| c.kind).collect();
    assert_eq!(
        kinds,
        [
            ColumnKind::Id,
            ColumnKind::Name,
            ColumnKind::Duration,
            ColumnKind::Start,
            ColumnKind::Finish
        ]
    );
    let headers: Vec<&str> = plan.source.columns.iter().map(|c| c.header.as_str()).collect();
    let columns = fr_args(
        TextId::PdfColumns,
        &[("names", Arg::Str(headers.join(", ").into()))],
    );
    assert!(
        harness.query_by_label(&columns).is_some(),
        "colonnes reconnues absentes"
    );
    // Fériés cochés d'office (en-tête français) ; l'élévation enjambe les
    // congés inconnus : sa fin calculée tombe trois semaines plus tôt.
    assert!(p.holidays_on);
    let elevation = p.rows.iter().find(|r| r.name == "Élévation").unwrap();
    assert_eq!(elevation.finish_gap, -21);
    assert_eq!(elevation.finish_computed, Some(day("2026-08-28")));
    // Retrait du nom par niveau.
    assert!(harness.query_by_label("  Élévation").is_some());
    assert!(harness.query_by_label("Second œuvre").is_some());
    // Filtre : la seule ligne à écart reste.
    click_prefix(&mut harness, &fr(TextId::PdfOnlyGaps));
    assert!(state(harness.state()).only_gaps);
    assert!(harness.query_by_label("  Terrassement").is_none());
    assert!(harness.query_by_label("  Élévation").is_some());
    let gap = fr_args(TextId::PdfGapFinish, &[("value", Arg::Str("-21".into()))]);
    assert!(harness.query_by_label(&gap).is_some(), "écart absent");
}

/// H-UI-207 : réglages montrés seulement quand ils servent ; ordre
/// jour/mois si aucune date ne le tranche, « sans hiérarchie » après un
/// refus de hiérarchie, contrainte des tâches liées si le PDF a des liens.
#[test]
fn h_ui_207_settings_shown_only_when_useful() {
    let mut plan = ImportedPlan {
        language: None,
        date_order: Some(NumericOrder::DayMonth),
        date_order_source: plannercraft_pdfimport::plan::DateOrderSource::Values,
        tasks: Vec::new(),
        warnings: Vec::new(),
        source: plannercraft_pdfimport::plan::SourceInfo::default(),
        stats: plannercraft_pdfimport::plan::ImportStats::default(),
        drawn: plannercraft_pdfimport::DrawnLinks::default(),
    };
    let none = PdfSettings::default();
    assert_eq!(
        visible_settings(Some(&Ok(plan.clone())), &none),
        VisibleSettings::default()
    );
    plan.warnings.push(Warning {
        code: WarningCode::DateOrderAssumed,
        id: None,
        page: None,
        value: None,
    });
    plan.stats.links_read = 3;
    let shown = visible_settings(Some(&Ok(plan)), &none);
    assert!(shown.date_order && shown.pin_starts && !shown.flatten);
    for refused in [ImportError::FlatHierarchy, ImportError::InconsistentHierarchy] {
        let shown = visible_settings(Some(&Err(refused)), &none);
        assert!(shown.flatten && !shown.date_order && !shown.pin_starts);
        assert_eq!(refusal_hint(refused), Some(TextId::PdfHintHierarchy));
    }
    assert!(!visible_settings(Some(&Err(ImportError::Scanned)), &none).flatten);
    // Un réglage imposé reste visible pour être défait.
    let forced = PdfSettings {
        date_order: Some(NumericOrder::MonthDay),
        flatten: true,
        ..PdfSettings::default()
    };
    let shown = visible_settings(None, &forced);
    assert!(shown.date_order && shown.flatten);
    // Le PDF exporté n'a ni liens imprimés ni ordre ambigu : rien à régler.
    let mut harness = harness_with(vec![picked("planning-chantier.pdf", &sample_pdf())]);
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    assert!(harness.query_by_label(&fr(TextId::PdfPinStarts)).is_none());
    assert!(harness.query_by_label(&fr(TextId::PdfFlatten)).is_none());
    assert!(harness.query_by_label(&fr(TextId::PdfDateOrder)).is_none());
}

/// H-UI-208 : fériés français cochés d'office pour un PDF français,
/// décochables ; le compteur et le tableau suivent.
#[test]
fn h_ui_208_french_holidays_default_and_toggle() {
    let mut harness = harness_with(vec![picked("planning-chantier.pdf", &sample_pdf())]);
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    let with = state(harness.state()).preview().unwrap().dates_equal;
    assert_eq!(state(harness.state()).settings.holidays, Some(true));
    assert!(harness.query_by_label(&same(i64::from(with), 20)).is_some());
    click_prefix(&mut harness, &fr(TextId::PdfHolidays));
    settle(&mut harness);
    let s = state(harness.state());
    assert_eq!(s.settings.holidays, Some(false));
    let p = s.preview().unwrap();
    assert!(!p.holidays_on);
    assert!(p.dates_equal < with, "{} ≥ {with}", p.dates_equal);
    // Sans le 14 juillet, les fondations finissent un jour ouvré plus tôt.
    let fondations = p.rows.iter().find(|r| r.name == "Fondations").unwrap();
    assert_eq!(fondations.finish_gap, -3);
    assert!(
        harness
            .query_by_label(&same(i64::from(p.dates_equal), 20))
            .is_some()
    );
}

/// H-UI-209 : la période déduite est proposée décochée, avec ses jours et
/// le nombre de tâches qu'elle explique ; cochée, toutes les dates
/// retombent sur le PDF.
#[test]
fn h_ui_209_deduced_period_checked_by_the_user() {
    let mut harness = harness_with(vec![picked("planning-chantier.pdf", &sample_pdf())]);
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    let s = state(harness.state());
    let p = s.preview().unwrap();
    let periods = &p.deduction.0.periods;
    assert_eq!(periods.len(), 1, "{:?}", p.deduction.0);
    assert!(periods[0].exact());
    assert_eq!(
        periods[0].placement(0),
        Some((day("2026-08-03"), day("2026-08-21")))
    );
    assert_eq!(s.choices.len(), 1);
    assert!(!s.choices[0].checked, "période cochée d'office");
    assert!(p.dates_equal < p.dates_compared);
    let line = fr_args(
        TextId::PdfPeriodLine,
        &[
            ("from", Arg::Str("lun 03/08/26".into())),
            ("to", Arg::Str("ven 21/08/26".into())),
            ("count", Arg::Count(15)),
            ("n", Arg::Count(1)),
        ],
    );
    assert!(
        harness.query_by_label(&line).is_some(),
        "période absente : {line}"
    );
    harness.get_by_label(&line).click();
    harness.run();
    settle(&mut harness);
    let s = state(harness.state());
    assert!(s.choices[0].checked);
    assert_eq!(s.settings.periods, [(day("2026-08-03"), day("2026-08-21"))]);
    let p = s.preview().unwrap();
    assert_eq!(p.dates_equal, p.dates_compared);
    assert!(harness.query_by_label(&same(20, 20)).is_some());
    // La période n'a pas bougé (même déduction) et reste cochée.
    assert_eq!(p.deduction.0.periods.len(), 1);
}

/// H-UI-210 : « Importer » ouvre un nouveau document non enregistré, nommé
/// d'après le fichier, dont le calendrier porte les jours chômés retenus ;
/// « Annuler » ne touche à rien ; document modifié : confirmation.
#[test]
fn h_ui_210_import_opens_new_document_cancel_keeps() {
    let bytes = sample_pdf();
    let mut harness = harness_with(vec![
        picked("planning-chantier.pdf", &bytes),
        picked("planning-chantier.pdf", &bytes),
        picked("planning-chantier.pdf", &bytes),
    ]);
    open_demo(&mut harness);
    // Annuler : l'exemple reste.
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    crate::click(&mut harness, &fr(TextId::TaskInfoCancel));
    assert!(harness.state().view().dialog.is_none());
    assert!(harness.state().document.as_ref().unwrap().origin.demo);
    // Document modifié : confirmation, puis « Ne pas enregistrer ».
    harness.state_mut().dispatch(Action::Structure(
        plannercraft_ui_egui::structure::StructureOp::InsertBefore,
    ));
    harness.run();
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    crate::click(&mut harness, &fr(TextId::CiImport));
    assert!(harness.state().confirmation_pending());
    assert!(harness.state().document.as_ref().unwrap().origin.demo);
    crate::click(&mut harness, &fr(TextId::ConfirmCancel));
    assert!(harness.state().document.as_ref().unwrap().origin.demo);
    // Troisième import : période cochée, « Importer », « Ne pas
    // enregistrer » : document remplacé.
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    let line = fr_args(
        TextId::PdfPeriodLine,
        &[
            ("from", Arg::Str("lun 03/08/26".into())),
            ("to", Arg::Str("ven 21/08/26".into())),
            ("count", Arg::Count(15)),
            ("n", Arg::Count(1)),
        ],
    );
    harness.get_by_label(&line).click();
    harness.run();
    settle(&mut harness);
    crate::click(&mut harness, &fr(TextId::CiImport));
    assert!(harness.state().confirmation_pending());
    crate::click(&mut harness, &fr(TextId::ConfirmDiscard));
    let app = harness.state();
    assert!(app.view().dialog.is_none());
    let document = app.document.as_ref().expect("document importé");
    assert_eq!(document.origin.name, "planning-chantier");
    assert_eq!(document.origin.path, None);
    assert!(!document.origin.demo);
    assert!(!document.is_dirty());
    let project = document.project();
    assert_eq!(project.tasks.len(), 10);
    let cal = project.calendars.get(project.calendar).unwrap();
    let names: Vec<&str> = cal.exceptions().iter().map(|e| e.name.as_str()).collect();
    assert!(names.contains(&"Fête nationale"), "{names:?}");
    assert!(names.contains(&fr(TextId::PdfPeriodName).as_str()), "{names:?}");
    let resolved =
        plannercraft_core::calendar::resolve_day(&project.calendars, project.calendar, day("2026-08-10"));
    assert_eq!(resolved.unwrap().hours, DaySetting::NonWorking);
}

/// H-UI-211 : un PDF refusé affiche le motif en clair et quoi faire ;
/// « Importer » reste grisé ; chaque refus et chaque avertissement du
/// moteur a son texte traduit.
#[test]
fn h_ui_211_refusal_shown_with_hint() {
    let mut harness = harness_with(vec![picked("faux.pdf", b"ceci n'est pas un PDF")]);
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    let s = state(harness.state());
    assert_eq!(
        s.shown.as_ref().unwrap().result.as_ref().err(),
        Some(&PreviewError::Refused(ImportError::Invalid))
    );
    let reason = fr_args(
        TextId::PdfRefused,
        &[("detail", Arg::Str(fr(TextId::ErrorPdfimportInvalid).into()))],
    );
    assert!(
        harness.query_by_label(&reason).is_some(),
        "motif absent : {reason}"
    );
    assert!(harness.query_by_label(&fr(TextId::PdfHintDamaged)).is_some());
    assert!(
        harness
            .get_by_label(&fr(TextId::CiImport))
            .accesskit_node()
            .is_disabled()
    );
    let texts = Texts::new(Language::Fr);
    for e in [
        ImportError::TooLarge,
        ImportError::Encrypted,
        ImportError::Invalid,
        ImportError::ReaderFailure,
        ImportError::NoText,
        ImportError::Scanned,
        ImportError::NoTable,
        ImportError::NoDates,
        ImportError::FlatHierarchy,
        ImportError::InconsistentHierarchy,
        ImportError::NoTasks,
        ImportError::TooManyTasks(20_001),
    ] {
        let id = plannercraft_ui_egui::i18n::error_text_id(e.code());
        assert_ne!(id, TextId::ErrorUnknown, "{}", e.code());
        assert_ne!(texts.get(id), "");
    }
    for code in [
        WarningCode::IdGap,
        WarningCode::Flattened,
        WarningCode::DateOrderConflict,
        WarningCode::ConstraintRefused,
    ] {
        assert_ne!(texts.get(warning_text_id(code)), "");
    }
}

/// H-UI-212 : le calcul ne bloque pas l'interface. Bureau : fil dédié, un
/// réglage changé pendant un calcul attend sa fin (un seul calcul à la
/// fois) ; web (chemin différé, exercé ici) : « Lecture du PDF en cours… »
/// est affiché une image avant le calcul. Bornes du web.
#[test]
fn h_ui_212_background_and_deferred_compute() {
    let bytes = std::sync::Arc::<[u8]>::from(sample_pdf());
    let ctx = egui::Context::default();
    let mut s = PdfImportState::open(
        "a.pdf".to_owned(),
        std::sync::Arc::clone(&bytes),
        now(),
        TextLanguage::Fr,
        "Congés".to_owned(),
    );
    assert!(s.busy());
    s.refresh(&ctx);
    assert!(s.busy());
    // Réglage changé pendant le calcul : pas de second calcul.
    s.settings.pin_starts = true;
    s.refresh(&ctx);
    for _ in 0..1_000 {
        s.refresh(&ctx);
        if !s.busy() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert!(!s.busy());
    assert!(s.shown.as_ref().unwrap().settings.pin_starts);
    // Chemin différé (web) : la première image affiche l'attente.
    let mut d = PdfImportState::open(
        "a.pdf".to_owned(),
        bytes,
        now(),
        TextLanguage::Fr,
        "Congés".to_owned(),
    );
    d.use_deferred_compute();
    d.refresh(&ctx);
    assert!(d.busy() && d.shown.is_none());
    d.refresh(&ctx);
    assert!(
        d.busy() && d.shown.is_none(),
        "calcul fait avant d'afficher l'attente"
    );
    d.refresh(&ctx);
    assert!(!d.busy() && d.preview().is_some());
    // Même résultat que le fil (déterministe).
    assert_eq!(
        d.preview().unwrap().rows,
        compute(&ComputeInput {
            bytes: std::sync::Arc::from(sample_pdf()),
            settings: d.settings.clone(),
            cached_plan: None,
            now: now(),
            lang: TextLanguage::Fr,
            name: "a".to_owned(),
            period_name: "Congés".to_owned(),
        })
        .preview()
        .unwrap()
        .rows
    );
    // Bornes : celles du moteur sur le bureau (64 Mio, 500 pages).
    let limits = import_limits();
    assert_eq!(limits, plannercraft_pdfimport::ImportLimits::default());
    // Le libellé d'attente existe.
    assert_ne!(fr(TextId::PdfReading), "");
}

/// H-UI-206 : capture de la fenêtre (français, clair), sur le PDF produit
/// par le projet ; fériés cochés d'office, période proposée décochée.
#[test]
fn h_ui_206_snapshot_pdf_import() {
    let ui_state = plannercraft_ui_egui::UiState {
        language: LanguagePref::Fr,
        theme: ThemePref::Light,
        ..plannercraft_ui_egui::UiState::default()
    };
    let env = Env {
        files: Box::new(PickedFiles(
            vec![picked("planning-chantier.pdf", &sample_pdf())].into(),
        )),
        ..Env::fixed(now())
    };
    let mut harness = crate::common::harness_with(App::new(ui_state, env), ThemePref::Light);
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    crate::common::snapshot(&mut harness, "pdf_import_fr_light");
}

/// Liens du projet construit par l'aperçu (successeur, nombre de liens).
fn project_links(state: &PdfImportState) -> usize {
    state
        .preview()
        .unwrap()
        .project
        .0
        .tasks
        .iter()
        .map(|t| t.predecessors.len())
        .sum()
}

/// H-UI-213 : le résumé donne les liens relus dans le dessin et les
/// écartés ; rien quand le dessin n'a pas de pointe.
#[test]
fn h_ui_213_drawn_links_summary_line() {
    let mut harness = harness_with(vec![picked("planning-chantier.pdf", &sample_pdf())]);
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    let s = state(harness.state());
    let plan = s.shown.as_ref().unwrap().plan.0.as_ref().as_ref().unwrap();
    // Les sept liens du planning exporté (FD, DD), aucun écarté (les bouts
    // des barres récapitulatives, en pointe sans trait, ne comptent pas).
    assert_eq!(drawn_counts(plan), Some((7, 0)));
    let line = fr_args(
        TextId::PdfSummaryDrawn,
        &[("count", Arg::Count(7)), ("dropped", Arg::Count(0))],
    );
    assert!(harness.query_by_label(&line).is_some(), "ligne absente : {line}");
    // Planning sans pointe : pas de ligne.
    let mut bare = plan.clone();
    bare.drawn = plannercraft_pdfimport::DrawnLinks::default();
    assert_eq!(drawn_counts(&bare), None);
}

/// H-UI-214 : « Recréer les liens dessinés » montrée seulement quand le
/// dessin donne des liens et que le PDF n'a pas de colonne Prédécesseurs ;
/// cochée d'office.
#[test]
fn h_ui_214_drawn_links_checkbox_shown_and_checked() {
    let mut harness = harness_with(vec![picked("planning-chantier.pdf", &sample_pdf())]);
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    let s = state(harness.state());
    assert_eq!(s.settings.drawn_links, None);
    assert_eq!(s.preview().unwrap().drawn_placed.is_some(), DRAWN_LINKS_DEFAULT);
    let label = fr(TextId::PdfDrawnLinks);
    let checkbox = harness.get_by_label(&label);
    assert_eq!(
        checkbox.accesskit_node().toggled(),
        Some(egui::accesskit::Toggled::True)
    );
    let plan = s
        .shown
        .as_ref()
        .unwrap()
        .plan
        .0
        .as_ref()
        .as_ref()
        .unwrap()
        .clone();
    let none = PdfSettings::default();
    assert!(visible_settings(Some(&Ok(plan.clone())), &none).drawn_links);
    // Colonne Prédécesseurs : elle fait foi, la case n'est pas montrée.
    let mut column = plan.clone();
    column.stats.links_read = 2;
    assert!(!visible_settings(Some(&Ok(column)), &none).drawn_links);
    // Aucun lien sûr dans le dessin : pas de case.
    let mut bare = plan;
    bare.drawn.links.clear();
    assert!(!visible_settings(Some(&Ok(bare)), &none).drawn_links);
}

/// H-UI-215 : le tableau, le compteur et le projet importé suivent la
/// case.
#[test]
fn h_ui_215_table_and_counter_follow_the_drawn_links() {
    let mut harness = harness_with(vec![picked("planning-chantier.pdf", &sample_pdf())]);
    harness.state_mut().dispatch(Action::ImportPdf);
    settle(&mut harness);
    let s = state(harness.state());
    let with = s.preview().unwrap();
    assert_eq!(with.drawn_placed, Some(7));
    assert_eq!(project_links(s), 7);
    let (equal, compared) = (with.dates_equal, with.dates_compared);
    assert!(
        harness
            .query_by_label(&same(i64::from(equal), i64::from(compared)))
            .is_some()
    );
    click_prefix(&mut harness, &fr(TextId::PdfDrawnLinks));
    settle(&mut harness);
    let s = state(harness.state());
    assert_eq!(s.settings.drawn_links, Some(false));
    let without = s.preview().unwrap();
    assert_eq!(without.drawn_placed, None);
    assert_eq!(project_links(s), 0);
    assert!(
        harness
            .query_by_label(&same(
                i64::from(without.dates_equal),
                i64::from(without.dates_compared)
            ))
            .is_some()
    );
    // Recochée : les liens reviennent, le compteur aussi.
    click_prefix(&mut harness, &fr(TextId::PdfDrawnLinks));
    settle(&mut harness);
    let s = state(harness.state());
    assert_eq!(project_links(s), 7);
    assert_eq!(s.preview().unwrap().dates_equal, equal);
}
