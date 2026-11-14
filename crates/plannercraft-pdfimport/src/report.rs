//! Rapport d'import (étape 8) : texte lisible et vue JSON stable (clés
//! triées par serde, aucune horloge). Le texte cite les dates **lues** et
//! les dates **calculées** sous ces noms : aucune date lue n'est
//! présentée comme un résultat (règle 3).

use std::collections::BTreeMap;
use std::fmt::Write as _;

use plannercraft_automation::text::Language;
use serde::Serialize;

use crate::convert::{Converted, DateGap, DrawnUse};
use crate::header::{Column, HeaderLanguage};
use crate::links::{DrawnLinks, LinkDoubt};
use crate::plan::{BarColor, DateOrderSource, ImportStats, ImportedPlan, Warning};
use crate::values::NumericOrder;

/// Écarts de dates listés au plus dans le texte.
pub const MAX_GAPS_SHOWN: usize = 20;

/// Tâche lue, vue JSON.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TaskView {
    /// N° lu.
    pub id: u32,
    /// Nom.
    pub name: String,
    /// Niveau.
    pub level: u16,
    /// Récapitulative.
    pub summary: bool,
    /// Jalon.
    pub milestone: bool,
    /// Durée lue (texte).
    pub duration: String,
    /// Durée lue comprise.
    pub duration_read: bool,
    /// Début lu (`AAAA-MM-JJTHH:MM`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_read: Option<String>,
    /// Fin lue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_read: Option<String>,
    /// Prédécesseurs (texte lu).
    pub predecessors: String,
    /// Liens gardés.
    pub links: usize,
    /// Ressources (texte lu).
    #[serde(skip_serializing_if = "String::is_empty")]
    pub resources: String,
    /// % achevé lu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_complete: Option<u8>,
    /// Page (1 = première).
    pub page: usize,
    /// Confiance (0 à 100).
    pub confidence: u8,
}

/// Conversion, vue JSON.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ConversionView {
    /// Début du projet (plus petit début lu, R-270).
    pub project_start: String,
    /// Contraintes « Début au plus tôt le » posées (R-271).
    pub constraints: u32,
    /// Liens posés.
    pub links: u32,
    /// Dates lues comparées (R-272).
    pub dates_compared: u32,
    /// Dates calculées égales aux dates lues, au jour.
    pub dates_equal: u32,
    /// Écarts.
    pub gaps: Vec<DateGap>,
    /// Avertissements de la conversion.
    pub warnings: Vec<Warning>,
    /// Liens dessinés posés (lot P1c), s'ils l'ont été.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drawn: Option<DrawnUse>,
}

/// Lien relu dans le dessin, vue JSON.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DrawnLinkView {
    /// N° du prédécesseur.
    pub pred: u32,
    /// N° du successeur.
    pub succ: u32,
    /// Type (`fs`, `ss`, `ff`, `sf`).
    pub kind: &'static str,
    /// Confiance (0 à 100).
    pub confidence: u8,
    /// Motifs de doute.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub doubts: Vec<LinkDoubt>,
    /// Page (1 = première).
    pub page: usize,
}

/// Liens relus dans le dessin, vue JSON (lot P1c).
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DrawnView {
    /// Pointes de flèche trouvées.
    pub arrows: u32,
    /// Liens reconstitués.
    pub links: Vec<DrawnLinkView>,
    /// Liens douteux (confiance sous le seuil, jamais posés).
    pub doubtful: usize,
    /// Écartés, par motif (code stable).
    pub dropped: BTreeMap<&'static str, u32>,
    /// Bornes atteintes.
    pub truncated: bool,
}

/// Code court d'un type de lien.
#[must_use]
pub const fn kind_code(kind: plannercraft_core::LinkType) -> &'static str {
    use plannercraft_core::LinkType;
    match kind {
        LinkType::FinishToStart => "fs",
        LinkType::StartToStart => "ss",
        LinkType::FinishToFinish => "ff",
        LinkType::StartToFinish => "sf",
    }
}

fn drawn_view(d: &DrawnLinks) -> DrawnView {
    DrawnView {
        arrows: d.arrows,
        links: d
            .links
            .iter()
            .map(|l| DrawnLinkView {
                pred: l.pred,
                succ: l.succ,
                kind: kind_code(l.kind),
                confidence: l.confidence,
                doubts: l.doubts.clone(),
                page: l.page.saturating_add(1),
            })
            .collect(),
        doubtful: d.links.iter().filter(|l| l.doubtful()).count(),
        dropped: d.dropped.iter().map(|(k, n)| (k.code(), *n)).collect(),
        truncated: d.truncated,
    }
}

/// Rapport, vue JSON.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReportView {
    /// Langue des intitulés.
    pub language: Option<HeaderLanguage>,
    /// Ordre jour/mois retenu.
    pub date_order: Option<NumericOrder>,
    /// Origine du choix.
    pub date_order_source: DateOrderSource,
    /// Mesures.
    pub stats: ImportStats,
    /// Tâches lues.
    pub tasks_imported: usize,
    /// Pages.
    pub pages: usize,
    /// Pages du tableau.
    pub table_pages: Vec<usize>,
    /// Producteur.
    pub producer: Option<String>,
    /// Format (centièmes de point).
    pub page_size: (i32, i32),
    /// Colonnes.
    pub columns: Vec<Column>,
    /// Couleurs des barres.
    pub bar_colors: Vec<BarColor>,
    /// Avertissements de la lecture.
    pub warnings: Vec<Warning>,
    /// Tâches.
    pub tasks: Vec<TaskView>,
    /// Liens relus dans le dessin (lot P1c).
    pub drawn: DrawnView,
    /// Conversion, si faite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion: Option<ConversionView>,
}

/// Vue JSON du rapport.
#[must_use]
pub fn view(plan: &ImportedPlan, converted: Option<&Converted>) -> ReportView {
    ReportView {
        language: plan.language,
        date_order: plan.date_order,
        date_order_source: plan.date_order_source,
        stats: plan.stats,
        tasks_imported: plan.tasks.len(),
        pages: plan.source.pages,
        table_pages: plan.source.table_pages.clone(),
        producer: plan.source.producer.clone(),
        page_size: plan.source.page_size,
        columns: plan.source.columns.clone(),
        bar_colors: plan.source.bar_colors.clone(),
        warnings: plan.warnings.clone(),
        tasks: plan
            .tasks
            .iter()
            .map(|t| TaskView {
                id: t.id,
                name: t.name.clone(),
                level: t.level,
                summary: t.summary,
                milestone: t.milestone(),
                duration: t.duration_text.clone(),
                duration_read: t.duration.is_some(),
                start_read: t.start.map(|m| m.to_string()),
                finish_read: t.finish.map(|m| m.to_string()),
                predecessors: t.predecessors_text.clone(),
                links: t.predecessors.len(),
                resources: t.resources.clone(),
                percent_complete: t.percent_complete,
                page: t.page.saturating_add(1),
                confidence: t.confidence,
            })
            .collect(),
        drawn: drawn_view(&plan.drawn),
        conversion: converted.map(|c| ConversionView {
            project_start: c.project_start.to_string(),
            constraints: c.constraints,
            links: c.links,
            dates_compared: c.dates_compared,
            dates_equal: c.dates_equal,
            gaps: c.gaps.clone(),
            warnings: c.warnings.clone(),
            drawn: c.drawn.clone(),
        }),
    }
}

/// Points → millimètres (centièmes de point, arrondi).
fn mm(cpt: i32) -> i64 {
    // 1 pt = 25,4 / 72 mm ; centièmes de point : × 254 / 72 000.
    (i64::from(cpt).saturating_mul(254).saturating_add(36_000)) / 72_000
}

/// Rapport lisible.
#[must_use]
#[allow(
    clippy::too_many_lines,
    clippy::many_single_char_names,
    reason = "une ligne de rapport par bloc, raccourcis de traduction"
)]
pub fn text(plan: &ImportedPlan, converted: Option<&Converted>, lang: Language) -> String {
    let fr = lang == Language::Fr;
    let t = |f: &'static str, e: &'static str| if fr { f } else { e };
    let s = &plan.stats;
    let src = &plan.source;
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{} : {} {} ({} {}), {} {}",
        t("Source", "Source"),
        src.pages,
        t("page(s)", "page(s)"),
        src.pages_read,
        t("lue(s)", "read"),
        t("tableau en page(s)", "table on page(s)"),
        src.table_pages
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", "),
    );
    if let Some(p) = &src.producer {
        let _ = writeln!(out, "{} : {p}", t("Producteur", "Producer"));
    }
    let (w, h) = src.page_size;
    let _ = writeln!(out, "{} : {} × {} mm", t("Format", "Page size"), mm(w), mm(h));
    let columns: Vec<String> = src
        .columns
        .iter()
        .map(|c| format!("{} [{}]", c.header, c.kind.code()))
        .collect();
    let _ = writeln!(out, "{} : {}", t("Colonnes", "Columns"), columns.join(" | "));
    let order = match (plan.date_order, plan.date_order_source) {
        (None, _) => t("aucune date numérique", "no numeric date").to_owned(),
        (Some(o), source) => format!(
            "{} ({})",
            match o {
                NumericOrder::DayMonth => t("jour/mois", "day/month"),
                NumericOrder::MonthDay => t("mois/jour", "month/day"),
            },
            match source {
                DateOrderSource::Forced => t("imposé", "forced"),
                DateOrderSource::Values => t("lu sur les dates", "read from the dates"),
                DateOrderSource::Language => t("supposé d'après la langue", "assumed from the language"),
                DateOrderSource::None => "-",
            }
        ),
    };
    let _ = writeln!(
        out,
        "{} : {} ; {} : {order}",
        t("Langue de l'en-tête", "Header language"),
        match plan.language {
            Some(HeaderLanguage::Fr) => "fr",
            Some(HeaderLanguage::En) => "en",
            None => "-",
        },
        t("dates", "dates"),
    );
    let _ = writeln!(
        out,
        "{} : {} ({} {}, {} {}, {} {}, {} {})",
        t("Tâches lues", "Tasks read"),
        plan.tasks.len(),
        t("N° max", "max ID"),
        s.max_id,
        s.blank_rows,
        t("ligne(s) vide(s)", "blank row(s)"),
        s.duplicate_rows,
        t("doublon(s)", "duplicate(s)"),
        s.levels,
        if s.levels_from_wbs {
            t("niveau(x) (WBS)", "level(s) (WBS)")
        } else {
            t("niveau(x) (retrait)", "level(s) (indent)")
        },
    );
    let _ = writeln!(
        out,
        "{} {}/{n} ; {} {}/{n} ; {} {}/{n} ; {} {} ({} {}) ; {} {}/{} ; {} {}",
        t("Durées", "Durations"),
        s.durations_read,
        t("débuts lus", "starts read"),
        s.starts_read,
        t("fins lues", "finishes read"),
        s.finishes_read,
        t("liens", "links"),
        s.links_read,
        s.links_dropped,
        t("ignoré(s)", "ignored"),
        t("récapitulatives cohérentes", "consistent summaries"),
        s.summaries_consistent,
        s.summaries_checked,
        t("lignes douteuses", "doubtful rows"),
        s.doubtful_rows,
        n = plan.tasks.len(),
    );
    write_warnings(&mut out, &plan.warnings, lang);
    write_drawn(&mut out, &plan.drawn, lang);
    if let Some(c) = converted {
        let _ = writeln!(
            out,
            "{} : {} {} ; {} {} ; {} {}",
            t("Projet", "Project"),
            t("début", "start"),
            c.project_start,
            c.links,
            t("lien(s)", "link(s)"),
            c.constraints,
            t(
                "contrainte(s) « Début au plus tôt le »",
                "\"Start No Earlier Than\" constraint(s)"
            ),
        );
        let _ = writeln!(
            out,
            "{} : {} {}, {} {}, {} {}",
            t(
                "Dates calculées par le moteur et dates lues",
                "Engine dates vs. dates read"
            ),
            c.dates_compared,
            t("comparée(s)", "compared"),
            c.dates_equal,
            t("égale(s) au jour", "equal to the day"),
            c.gaps.len(),
            t("écart(s)", "gap(s)"),
        );
        for g in c.gaps.iter().take(MAX_GAPS_SHOWN) {
            let _ = writeln!(
                out,
                "  {} {} {} : {} {}, {} {} ({:+} j)",
                t("N°", "ID"),
                g.id,
                if g.field == "start" {
                    t("début", "start")
                } else {
                    t("fin", "finish")
                },
                t("lu", "read"),
                g.read,
                t("calculé", "computed"),
                g.computed,
                g.days,
            );
        }
        if c.gaps.len() > MAX_GAPS_SHOWN {
            let _ = writeln!(
                out,
                "  … {} {}",
                c.gaps.len().saturating_sub(MAX_GAPS_SHOWN),
                t("autre(s)", "more")
            );
        }
        if let Some(d) = &c.drawn {
            let _ = writeln!(
                out,
                "{} : {} {}, {} {}, {} {}, {} {}",
                t("Liens dessinés posés", "Drawn links placed"),
                d.placed,
                t("posé(s)", "placed"),
                d.refused_total(),
                t("refusé(s) par le moteur", "refused by the engine"),
                d.negative_lags,
                t("décalage(s) négatif(s)", "negative lag(s)"),
                d.constraints,
                t(
                    "contrainte(s) sur des tâches liées",
                    "constraint(s) on linked tasks"
                ),
            );
        }
        write_warnings(&mut out, &c.warnings, lang);
    }
    out
}

/// Ligne des liens relus dans le dessin : pointes, liens, douteux, écartés
/// par motif.
fn write_drawn(out: &mut String, d: &DrawnLinks, lang: Language) {
    if d.arrows == 0 && d.links.is_empty() {
        return;
    }
    let fr = lang == Language::Fr;
    let t = |f: &'static str, e: &'static str| if fr { f } else { e };
    let dropped: Vec<String> = d
        .dropped
        .iter()
        .map(|(k, n)| format!("{} {n}", k.label(lang)))
        .collect();
    let _ = writeln!(
        out,
        "{} : {} {}, {} {} ({} {}), {} {}{}{}",
        t("Liens dessinés", "Drawn links"),
        d.arrows,
        t("pointe(s)", "arrowhead(s)"),
        d.links.len(),
        t("lien(s) reconstitué(s)", "link(s) rebuilt"),
        d.links.iter().filter(|l| l.doubtful()).count(),
        t("douteux", "doubtful"),
        d.dropped_total(),
        t("écarté(s)", "dropped"),
        if dropped.is_empty() {
            String::new()
        } else {
            format!(" ({})", dropped.join(", "))
        },
        if d.truncated {
            t(", relevé partiel (bornes)", ", partial (limits)")
        } else {
            ""
        },
    );
}

/// Avertissements regroupés par code : nombre et trois premiers N°.
fn write_warnings(out: &mut String, warnings: &[Warning], lang: Language) {
    let mut by_code: BTreeMap<&'static str, (usize, Vec<String>, &'static str)> = BTreeMap::new();
    for w in warnings {
        let entry = by_code
            .entry(w.code.code())
            .or_insert_with(|| (0, Vec::new(), w.code.message(lang)));
        entry.0 = entry.0.saturating_add(1);
        if entry.1.len() < 3 {
            let mut where_ = String::new();
            if let Some(id) = w.id {
                let _ = write!(where_, "{id}");
            }
            if let Some(v) = &w.value {
                if !where_.is_empty() {
                    where_.push(' ');
                }
                let _ = write!(where_, "« {v} »");
            }
            if !where_.is_empty() {
                entry.1.push(where_);
            }
        }
    }
    for (code, (count, examples, message)) in by_code {
        let _ = writeln!(
            out,
            "  {code} ×{count} : {message}{}",
            if examples.is_empty() {
                String::new()
            } else {
                format!(" ({})", examples.join(", "))
            }
        );
    }
}
