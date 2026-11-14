//! Règles de l'import PDF (`docs/scheduling-rules.md`, R-270 à R-277) :
//! les dates lues ne sont jamais recopiées comme dates calculées (règle 3) ;
//! jours fériés et périodes chômées déduites des écarts (lot P1b).
#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    reason = "tests : les interdits de la règle 10 ne s'appliquent pas, un échec doit paniquer"
)]

use plannercraft_automation::text::{Language, TextContext};
use plannercraft_core::calendar::{DaySetting, resolve_day};
use plannercraft_core::{Day, Moment, ProjectOptions};
use plannercraft_pdfimport::extract::{Extracted, PageContent, Word};
use plannercraft_pdfimport::header::HeaderLanguage;
use plannercraft_pdfimport::plan::{DateOrderSource, ImportStats, SourceInfo, build};
use plannercraft_pdfimport::{
    ConvertOptions, DeduceLimits, Deduction, ImportOptions, ImportedPlan, ImportedTask, NonWorkingDays,
    NumericOrder, WarningCode, calendar, convert, deduce, values,
};

fn ctx() -> TextContext {
    TextContext::for_options(&ProjectOptions::default(), Language::Fr)
}

fn moment(s: &str) -> Moment {
    s.parse().unwrap()
}

/// Tâche lue : dates `AAAA-MM-JJ` (début 08:00, fin 17:00).
fn task(
    id: u32,
    level: u16,
    summary: bool,
    duration: &str,
    start: &str,
    finish: &str,
    preds: &str,
) -> ImportedTask {
    let predecessors = values::predecessors(preds, &ctx())
        .into_iter()
        .map(Result::unwrap)
        .collect();
    ImportedTask {
        id,
        name: format!("T{id}"),
        level,
        summary,
        duration: values::duration(duration, &ctx()),
        duration_text: duration.to_owned(),
        start: (!start.is_empty()).then(|| moment(&format!("{start}T08:00"))),
        start_text: start.to_owned(),
        finish: (!finish.is_empty()).then(|| moment(&format!("{finish}T17:00"))),
        finish_text: finish.to_owned(),
        predecessors,
        predecessors_text: preds.to_owned(),
        successors_text: String::new(),
        resources: String::new(),
        percent_complete: None,
        bold: summary,
        page: 0,
        confidence: 100,
    }
}

fn plan(tasks: Vec<ImportedTask>) -> ImportedPlan {
    ImportedPlan {
        language: None,
        date_order: Some(NumericOrder::DayMonth),
        date_order_source: DateOrderSource::Values,
        tasks,
        warnings: Vec::new(),
        source: SourceInfo::default(),
        stats: ImportStats::default(),
        drawn: plannercraft_pdfimport::DrawnLinks::default(),
    }
}

fn options(pin: bool) -> ConvertOptions {
    ConvertOptions {
        name: "essai".to_owned(),
        now: moment("2026-01-01T08:00"),
        lang: Language::Fr,
        pin_read_starts: pin,
        non_working: Vec::new(),
    }
}

/// R-270 : le projet commence au plus petit début lu, même s'il n'est pas
/// sur la première ligne.
#[test]
fn r270_project_start_is_the_earliest_read_start() {
    let p = plan(vec![
        task(1, 1, false, "2 jours", "2026-01-07", "2026-01-08", ""),
        task(2, 1, false, "1 jour", "2026-01-05", "2026-01-05", ""),
    ]);
    let c = convert(&p, &options(false)).unwrap();
    assert_eq!(c.project_start, moment("2026-01-05T08:00"));
    // Sans début lu : plus petite fin lue, à l'heure de début par défaut.
    let p = plan(vec![task(1, 1, false, "1 jour", "", "2026-01-09", "")]);
    assert_eq!(
        convert(&p, &options(false)).unwrap().project_start,
        moment("2026-01-09T08:00")
    );
    // Aucune date : pas de projet.
    let p = plan(vec![task(1, 1, false, "1 jour", "", "", "")]);
    assert!(convert(&p, &options(false)).is_err());
}

/// R-271 : « Début au plus tôt le » au début lu, sur une feuille sans
/// prédécesseur que le moteur placerait plus tôt ; jamais sur une
/// récapitulative ni sur une feuille liée (par défaut).
#[test]
fn r271_snet_only_where_the_engine_would_start_earlier() {
    let p = plan(vec![
        task(1, 1, true, "10 jours", "2026-01-05", "2026-01-16", ""),
        task(2, 2, false, "3 jours", "2026-01-05", "2026-01-07", ""),
        task(3, 2, false, "2 jours", "2026-01-12", "2026-01-13", ""),
        task(4, 2, false, "2 jours", "2026-01-15", "2026-01-16", "3"),
    ]);
    let c = convert(&p, &options(false)).unwrap();
    // 2 démarre au début du projet : rien ; 3 est posée au 12/01 ; 4 est
    // liée : pas de contrainte, le moteur la place après 3 (14/01) et
    // l'écart avec le 15/01 lu est listé (R-272).
    assert_eq!(c.constraints, 1);
    let s = c.session.schedule();
    assert_eq!(s.scheduled(c.uids[2]).unwrap().start, moment("2026-01-12T08:00"));
    assert_eq!(s.scheduled(c.uids[3]).unwrap().start, moment("2026-01-14T08:00"));
    assert!(
        c.gaps
            .iter()
            .any(|g| g.id == 4 && g.field == "start" && g.days == -1)
    );
}

/// R-271 étendue (`pin_read_starts`) : la feuille liée placée trop tôt est
/// contrainte à son début lu, en commençant par la source de l'écart ; une
/// tâche en aval rattrapée par le décalage n'est pas contrainte.
#[test]
fn r271_pin_read_starts_constrains_the_source_of_the_gap_only() {
    let p = plan(vec![
        task(1, 1, false, "2 jours", "2026-01-05", "2026-01-06", ""),
        // Lue au 08/01 (un jour de battement invisible dans le PDF).
        task(2, 1, false, "2 jours", "2026-01-08", "2026-01-09", "1"),
        task(3, 1, false, "1 jour", "2026-01-12", "2026-01-12", "2"),
    ]);
    let c = convert(&p, &options(true)).unwrap();
    assert_eq!(c.constraints, 1);
    assert!(c.gaps.is_empty(), "{:?}", c.gaps);
    let strict = convert(&p, &options(false)).unwrap();
    assert_eq!(strict.constraints, 0);
    assert_eq!(strict.gaps.len(), 4);
}

/// R-272 : comparaison au jour, écart signé en jours calendaires (calculé
/// moins lu) ; une date lue n'est jamais recopiée : une durée lue qui ne
/// tombe pas sur la fin lue donne un écart.
#[test]
fn r272_gaps_are_signed_calendar_days() {
    let p = plan(vec![
        // 3 jours ouvrés depuis le ven 09/01 : fin le mar 13/01, lue au 12/01.
        task(1, 1, false, "3 jours", "2026-01-09", "2026-01-12", ""),
    ]);
    let c = convert(&p, &options(false)).unwrap();
    assert_eq!(c.dates_compared, 2);
    assert_eq!(c.dates_equal, 1);
    assert_eq!(c.gaps.len(), 1);
    let g = &c.gaps[0];
    assert_eq!(
        (g.field, g.read.as_str(), g.computed.as_str(), g.days),
        ("finish", "2026-01-12", "2026-01-13", 1)
    );
    let s = c.session.schedule().scheduled(c.uids[0]).unwrap();
    assert_eq!(s.finish, moment("2026-01-13T17:00"));
}

fn word(text: &str, x0: i32, baseline: i32) -> Word {
    let n = i32::try_from(text.chars().count()).unwrap();
    Word {
        text: text.to_owned(),
        x0,
        x1: x0 + n * 420,
        baseline,
        size: 800,
        bold: false,
        glyphs: Vec::new(),
    }
}

/// Une page : en-tête puis lignes N°, Nom, Durée, Début, Fin.
fn page(header: [&str; 5], rows: &[[&str; 5]]) -> Extracted {
    let xs = [400, 2000, 20000, 25000, 32000];
    let mut words = Vec::new();
    for (r, cells) in std::iter::once(&header).chain(rows).enumerate() {
        let y = 1000 + 1200 * i32::try_from(r).unwrap();
        for (k, cell) in cells.iter().enumerate() {
            let mut x = xs[k];
            for w in cell.split_whitespace() {
                let wd = word(w, x, y);
                x = wd.x1 + 300;
                words.push(wd);
            }
        }
    }
    Extracted {
        pages: vec![PageContent {
            words,
            glyphs: 2000,
            width: 84_000,
            height: 59_500,
            ..PageContent::default()
        }],
        page_count: 1,
        producer: None,
        creator: None,
    }
}

/// R-273 : l'ordre jour/mois des dates numériques se décide sur tout le
/// document : un jour > 12 tranche pour toutes les dates, même
/// ambiguës ; à défaut, la langue de l'en-tête avec un avertissement.
#[test]
fn r273_date_order_is_decided_on_the_whole_document() {
    let fr = ["N°", "Nom de la tâche", "Durée", "Début", "Fin"];
    let en = ["ID", "Task Name", "Duration", "Start", "Finish"];
    // En-tête anglais, mais « 25/03/26 » n'existe qu'en JJ/MM : « 03/04/26 »
    // est le 3 avril.
    let ex = page(
        en,
        &[
            ["1", "A", "1 day", "03/04/26", "03/04/26"],
            ["2", "B", "1 day", "25/03/26", "25/03/26"],
        ],
    );
    let p = build(&ex, &ImportOptions::default()).unwrap();
    assert_eq!(p.date_order, Some(NumericOrder::DayMonth));
    assert_eq!(p.tasks[0].start.unwrap(), moment("2026-04-03T08:00"));
    // Rien ne tranche : langue de l'en-tête, avertissement.
    let ex = page(fr, &[["1", "A", "1 jour", "03/04/26", "03/04/26"]]);
    let p = build(&ex, &ImportOptions::default()).unwrap();
    assert_eq!(p.date_order_source, DateOrderSource::Language);
    assert_eq!(p.tasks[0].start.unwrap(), moment("2026-04-03T08:00"));
    assert!(p.warnings.iter().any(|w| w.code == WarningCode::DateOrderAssumed));
    // Incompatibles dans les deux ordres : avertissement, dates fautives
    // ignorées.
    let ex = page(
        fr,
        &[
            ["1", "A", "1 jour", "25/03/26", "25/03/26"],
            ["2", "B", "1 jour", "03/25/26", "03/25/26"],
        ],
    );
    let p = build(&ex, &ImportOptions::default()).unwrap();
    assert!(
        p.warnings
            .iter()
            .any(|w| w.code == WarningCode::DateOrderConflict)
    );
    assert!(p.tasks[1].start.is_none());
}

fn day(s: &str) -> Day {
    s.parse().unwrap()
}

fn days(p: (Day, Day)) -> (String, String) {
    (p.0.to_string(), p.1.to_string())
}

/// Conversion avec des jours chômés.
fn convert_with(p: &ImportedPlan, non_working: Vec<NonWorkingDays>) -> plannercraft_pdfimport::Converted {
    let mut o = options(false);
    o.non_working = non_working;
    convert(p, &o).unwrap()
}

/// Déduction sur le calendrier Standard (aucun férié).
fn deduce_standard(p: &ImportedPlan) -> Deduction {
    let c = convert_with(p, Vec::new());
    deduce(p, c.session.project(), &DeduceLimits::default())
}

/// R-274 : à l'import, les fériés français sont proposés cochés quand
/// l'en-tête est français, sur toutes les années couvertes par les dates
/// lues (et la suivante), nommés dans la langue de l'interface.
#[test]
fn r274_import_proposes_french_holidays_for_the_covered_years() {
    let mut p = plan(vec![
        task(1, 1, false, "5 jours", "2026-12-21", "2026-12-28", ""),
        task(2, 1, false, "5 jours", "2027-03-01", "2027-03-05", ""),
    ]);
    assert!(!calendar::suggests_french_holidays(&p));
    p.language = Some(HeaderLanguage::Fr);
    assert!(calendar::suggests_french_holidays(&p));
    let h = calendar::french_holidays(&p, Language::Fr);
    // 2026, 2027 et 2028 : 33 fêtes, une par jour.
    assert_eq!(h.len(), 33);
    assert!(h.iter().all(|n| n.from == n.to));
    let christmas = h.iter().find(|n| n.from == day("2026-12-25")).unwrap();
    assert_eq!(christmas.name, "Noël");
    let en = calendar::french_holidays(&p, Language::En);
    assert!(en.iter().any(|n| n.name == "Christmas Day"));
    // Noël posé : « 5 jours » du lun 21/12 finit le lun 28/12, comme lu.
    let c = convert_with(&p, h);
    assert!(c.gaps.is_empty(), "{:?}", c.gaps);
    let standard = convert_with(&p, Vec::new());
    assert_eq!(standard.gaps.len(), 1);
}

/// R-275 : les jours chômés deviennent des exceptions du calendrier du
/// projet (`calendar.set_exception`), posées avant les tâches ; un jour
/// cité deux fois garde le premier nom, aucune exception ne se chevauche.
#[test]
fn r275_non_working_days_become_project_calendar_exceptions() {
    let p = plan(vec![task(1, 1, false, "3 jours", "2026-12-22", "2027-01-05", "")]);
    let nw = vec![
        NonWorkingDays {
            from: day("2026-12-25"),
            to: day("2026-12-25"),
            name: "Noël".to_owned(),
        },
        NonWorkingDays {
            from: day("2026-12-24"),
            to: day("2027-01-04"),
            name: "Congés".to_owned(),
        },
    ];
    let c = convert_with(&p, nw);
    let project = c.session.project();
    let cal = project.calendars.get(project.calendar).unwrap();
    let got: Vec<(String, String, String)> = cal
        .exceptions()
        .iter()
        .map(|e| {
            (
                e.period.first().to_string(),
                e.period.last().to_string(),
                e.name.clone(),
            )
        })
        .collect();
    assert_eq!(
        got,
        [
            (
                "2026-12-24".to_owned(),
                "2026-12-24".to_owned(),
                "Congés".to_owned()
            ),
            (
                "2026-12-25".to_owned(),
                "2026-12-25".to_owned(),
                "Noël".to_owned()
            ),
            (
                "2026-12-26".to_owned(),
                "2027-01-04".to_owned(),
                "Congés".to_owned()
            ),
        ]
    );
    for d in ["2026-12-24", "2026-12-25", "2026-12-31", "2027-01-04"] {
        let r = resolve_day(&project.calendars, project.calendar, day(d)).unwrap();
        assert_eq!(r.hours, DaySetting::NonWorking, "{d}");
    }
    // Mar 22, mer 23, puis mar 05/01 : fin lue retrouvée.
    assert!(c.gaps.is_empty(), "{:?}", c.gaps);
    // Document neuf : rien dans l'historique.
    assert_eq!(c.session.history().entries.len(), 0);
}

/// R-276, R-277 : une période d'août. Les tâches sans écart de part et
/// d'autre prouvent les jours travaillés ; la tâche longue a 15 jours de
/// trop, exactement les 15 jours candidats : période déterminée.
#[test]
fn r276_r277_one_august_period() {
    let p = plan(vec![
        task(1, 1, false, "5 jours", "2026-07-27", "2026-07-31", ""),
        task(2, 1, false, "5 jours", "2026-08-24", "2026-08-28", ""),
        task(3, 1, false, "10 jours", "2026-07-27", "2026-08-28", ""),
    ]);
    let d = deduce_standard(&p);
    assert_eq!((d.usable, d.proven, d.equations), (3, 2, 1));
    assert_eq!(d.periods.len(), 1, "{d:?}");
    let period = &d.periods[0];
    assert!(period.exact());
    assert_eq!(
        days(period.placement(0).unwrap()),
        ("2026-08-03".to_owned(), "2026-08-21".to_owned())
    );
    assert_eq!((period.working_days, period.explained), (15, 1));
    assert_eq!(d.explained, 1);
    // Cochée : la fin lue est retrouvée.
    let chosen = calendar::periods(&calendar::default_placements(&d.periods), "Congés");
    let c = convert_with(&p, chosen);
    assert!(c.gaps.is_empty(), "{:?}", c.gaps);
    assert_eq!(convert_with(&p, Vec::new()).gaps.len(), 1);
}

/// R-277 : deux périodes (fin d'année, été) dans deux suites séparées par
/// des jours prouvés ; chacune déterminée.
#[test]
fn r277_two_periods() {
    let p = plan(vec![
        task(1, 1, false, "5 jours", "2026-12-14", "2026-12-18", ""),
        task(2, 1, false, "5 jours", "2027-01-04", "2027-01-08", ""),
        task(3, 1, false, "10 jours", "2026-12-14", "2027-01-08", ""),
        task(4, 1, false, "5 jours", "2027-07-26", "2027-07-30", ""),
        task(5, 1, false, "5 jours", "2027-08-23", "2027-08-27", ""),
        task(6, 1, false, "10 jours", "2027-07-26", "2027-08-27", ""),
    ]);
    let d = deduce_standard(&p);
    let got: Vec<(String, String)> = d
        .periods
        .iter()
        .map(|p| days(p.placement(p.default_option).unwrap()))
        .collect();
    assert_eq!(
        got,
        [
            ("2026-12-21".to_owned(), "2027-01-01".to_owned()),
            ("2027-08-02".to_owned(), "2027-08-20".to_owned()),
        ]
    );
    assert!(
        d.periods
            .iter()
            .all(plannercraft_pdfimport::ProposedPeriod::exact)
    );
    assert_eq!(d.explained, 2);
}

/// R-277 : deux tâches contradictoires (même intervalle, 5 et 6 jours à
/// retirer) : aucune période ne satisfait la majorité des équations de la
/// suite, rien n'est proposé.
#[test]
fn r277_contradiction_between_tasks_proposes_nothing() {
    let p = plan(vec![
        task(1, 1, false, "5 jours", "2026-03-02", "2026-03-13", ""),
        task(2, 1, false, "4 jours", "2026-03-02", "2026-03-13", ""),
    ]);
    let d = deduce_standard(&p);
    assert_eq!(d.equations, 2);
    assert!(d.periods.is_empty(), "{d:?}");
    assert_eq!(d.ambiguous, 1);
}

/// R-276 : une tâche d'écart nul interdit ses jours : le mardi prouvé
/// travaillé n'est dans aucun placement proposé, alors qu'il l'est sans
/// elle.
#[test]
fn r276_proven_task_forbids_a_day() {
    let tasks = |with_proof: bool| {
        let mut t = vec![
            task(1, 1, false, "3 jours", "2026-03-02", "2026-03-06", ""),
            task(2, 1, false, "2 jours", "2026-03-03", "2026-03-06", ""),
        ];
        if with_proof {
            t.push(task(3, 1, false, "1 jour", "2026-03-03", "2026-03-03", ""));
        }
        plan(t)
    };
    let tuesday = day("2026-03-03");
    let contains_tuesday = |d: &Deduction| {
        d.periods
            .iter()
            .flat_map(|p| p.options.iter())
            .any(|&(a, b)| a <= tuesday && tuesday <= b)
    };
    let without = deduce_standard(&tasks(false));
    assert!(contains_tuesday(&without), "{without:?}");
    let with = deduce_standard(&tasks(true));
    assert_eq!(with.proven, 1);
    assert!(!with.periods.is_empty(), "{with:?}");
    assert!(!contains_tuesday(&with), "{with:?}");
    // Le mardi coupe la suite : deux placements de 2 jours (mer-jeu,
    // jeu-ven), à placer ; défaut : jeu-ven, collé au week-end.
    let period = &with.periods[0];
    assert!(!period.exact());
    assert_eq!(
        days(period.placement(period.default_option).unwrap()),
        ("2026-03-05".to_owned(), "2026-03-06".to_owned())
    );
    assert_eq!(
        days(period.window().unwrap()),
        ("2026-03-04".to_owned(), "2026-03-06".to_owned())
    );
}

/// R-276, R-277 : une « demi-journée » (5 jours lus du mercredi au
/// mercredi suivant) n'est jamais un congé : impossible si les jours sont
/// prouvés, non proposée (un seul jour à placer) sinon ; une heure lue
/// autre que l'heure par défaut écarte la tâche.
#[test]
fn r277_half_day_is_never_a_holiday() {
    let alone = plan(vec![task(1, 1, false, "5 jours", "2026-03-04", "2026-03-11", "")]);
    let d = deduce_standard(&alone);
    assert_eq!((d.equations, d.impossible), (1, 0));
    assert!(d.periods.is_empty(), "{d:?}");
    assert_eq!(d.ambiguous, 1);
    let proven = plan(vec![
        task(1, 1, false, "5 jours", "2026-03-04", "2026-03-11", ""),
        task(2, 1, false, "6 jours", "2026-03-04", "2026-03-11", ""),
    ]);
    let d = deduce_standard(&proven);
    assert_eq!((d.proven, d.impossible), (1, 1));
    assert_eq!(d.periods.len(), 0);
    let mut afternoon = task(1, 1, false, "5 jours", "2026-03-04", "2026-03-11", "");
    afternoon.start = Some(moment("2026-03-04T13:00"));
    let d = deduce_standard(&plan(vec![afternoon]));
    assert_eq!((d.partial_days, d.usable), (1, 0));
}
