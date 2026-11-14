//! Planning lu (étape 6) : tâches dédoublonnées, niveaux, valeurs lues,
//! ordre jour/mois choisi sur le document, avertissements et confiance.
//! Structure neutre : rien n'y est calculé, les dates y sont **lues**
//! (règle 3).

use std::collections::{BTreeMap, BTreeSet};

use plannercraft_automation::text::{Language, TextContext};
use plannercraft_core::{Day, Duration, Moment, ProjectOptions};
use serde::Serialize;

use crate::extract::{Cpt, Extracted, Rgb};
use crate::header::{Column, ColumnKind, HeaderLanguage, TableLayout, classify, find_table};
use crate::lines::{fold, group_lines};
use crate::links::{DrawnLinks, PageFrame, RowRef};
use crate::table::{RawRow, parse_id, read_rows};
use crate::values::{self, DateToken, NumericOrder, PredError, PredRef};
use crate::{ImportError, ImportOptions};

/// Tâche lue.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportedTask {
    /// N° lu (ou rang dans le tableau s'il n'y a pas de colonne N°).
    pub id: u32,
    /// Nom.
    pub name: String,
    /// Niveau hiérarchique (1 = haut).
    pub level: u16,
    /// Récapitulative (la tâche suivante est plus profonde).
    pub summary: bool,
    /// Durée lue.
    pub duration: Option<Duration>,
    /// Texte de la durée.
    pub duration_text: String,
    /// Début lu (jour lu, heure lue ou heure de début par défaut).
    pub start: Option<Moment>,
    /// Texte du début.
    pub start_text: String,
    /// Fin lue.
    pub finish: Option<Moment>,
    /// Texte de la fin.
    pub finish_text: String,
    /// Prédécesseurs lus (N° présents dans le tableau seulement).
    pub predecessors: Vec<PredRef>,
    /// Texte des prédécesseurs.
    pub predecessors_text: String,
    /// Texte de la colonne Successeurs, tel quel (non importé : sert à
    /// vérifier les liens relus dans le dessin, lot P1c).
    pub successors_text: String,
    /// Noms des ressources, tels quels.
    pub resources: String,
    /// % achevé lu.
    pub percent_complete: Option<u8>,
    /// Nom en gras.
    pub bold: bool,
    /// Page de la ligne (0 = première).
    pub page: usize,
    /// Confiance de la ligne, de 0 à 100.
    pub confidence: u8,
}

impl ImportedTask {
    /// Jalon : feuille de durée nulle.
    #[must_use]
    pub fn milestone(&self) -> bool {
        !self.summary && self.duration.is_some_and(|d| d.is_zero())
    }

    /// Ligne douteuse (confiance sous 70).
    #[must_use]
    pub const fn doubtful(&self) -> bool {
        self.confidence < DOUBTFUL_BELOW
    }
}

/// Seuil de confiance sous lequel une ligne est douteuse.
pub const DOUBTFUL_BELOW: u8 = 70;

/// Avertissement de l'import, localisable par son code.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Warning {
    /// Code stable.
    pub code: WarningCode,
    /// N° de la tâche concernée.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    /// Page (1 = première, comme à l'écran).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<usize>,
    /// Valeur en cause (texte lu, borné à 120 caractères).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Codes des avertissements.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WarningCode {
    /// N° absents de la suite (récapitulatives repliées, filtre).
    IdGap,
    /// Même N°, contenu différent : la première ligne est gardée.
    IdConflict,
    /// Ligne vide (N° sans nom) : ignorée.
    BlankRow,
    /// Nom coupé à l'impression (« … » final) : à compléter.
    NameTruncated,
    /// Durée illisible.
    DurationUnreadable,
    /// Durée absente d'une feuille.
    DurationMissing,
    /// Date illisible ou inexistante.
    DateUnreadable,
    /// Ordre jour/mois choisi par la langue de l'en-tête, faute d'indice.
    DateOrderAssumed,
    /// Dates incompatibles entre elles quel que soit l'ordre.
    DateOrderConflict,
    /// Fin lue avant le début lu.
    FinishBeforeStart,
    /// Prédécesseur illisible.
    PredecessorUnreadable,
    /// Prédécesseur d'un autre projet.
    PredecessorExternal,
    /// Prédécesseur absent du tableau.
    PredecessorUnknown,
    /// Tâche prédécesseur d'elle-même.
    PredecessorSelf,
    /// Saut de plus d'un niveau.
    LevelJump,
    /// Lignes de la zone du tableau rattachées à aucune tâche.
    StrayLines,
    /// Bornes de l'extraction atteintes : contenu partiel.
    Truncated,
    /// Pages lues limitées.
    PagesSkipped,
    /// Lien refusé par le moteur (boucle).
    LinkRefused,
    /// Contrainte refusée par le moteur.
    ConstraintRefused,
    /// Hiérarchie non lisible, tâches mises au niveau 1 à la demande.
    Flattened,
}

impl WarningCode {
    /// Code stable (`pdfimport.…`).
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::IdGap => "pdfimport.id_gap",
            Self::IdConflict => "pdfimport.id_conflict",
            Self::BlankRow => "pdfimport.blank_row",
            Self::NameTruncated => "pdfimport.name_truncated",
            Self::DurationUnreadable => "pdfimport.duration_unreadable",
            Self::DurationMissing => "pdfimport.duration_missing",
            Self::DateUnreadable => "pdfimport.date_unreadable",
            Self::DateOrderAssumed => "pdfimport.date_order_assumed",
            Self::DateOrderConflict => "pdfimport.date_order_conflict",
            Self::FinishBeforeStart => "pdfimport.finish_before_start",
            Self::PredecessorUnreadable => "pdfimport.predecessor_unreadable",
            Self::PredecessorExternal => "pdfimport.predecessor_external",
            Self::PredecessorUnknown => "pdfimport.predecessor_unknown",
            Self::PredecessorSelf => "pdfimport.predecessor_self",
            Self::LevelJump => "pdfimport.level_jump",
            Self::StrayLines => "pdfimport.stray_lines",
            Self::Truncated => "pdfimport.truncated",
            Self::PagesSkipped => "pdfimport.pages_skipped",
            Self::LinkRefused => "pdfimport.link_refused",
            Self::ConstraintRefused => "pdfimport.constraint_refused",
            Self::Flattened => "pdfimport.flattened",
        }
    }

    /// Message dans une langue.
    #[must_use]
    pub const fn message(self, lang: Language) -> &'static str {
        let (fr, en) = match self {
            Self::IdGap => (
                "N° absents du tableau (récapitulatives repliées ou filtre à l'impression)",
                "IDs missing from the table (collapsed summaries or a print filter)",
            ),
            Self::IdConflict => (
                "N° en double avec un contenu différent : première ligne gardée",
                "duplicate ID with different content: first row kept",
            ),
            Self::BlankRow => ("ligne vide ignorée", "blank row ignored"),
            Self::NameTruncated => (
                "nom coupé à l'impression (« … ») : à compléter",
                "name cut at print time (\"…\"): to be completed",
            ),
            Self::DurationUnreadable => (
                "durée illisible : durée par défaut",
                "unreadable duration: default duration used",
            ),
            Self::DurationMissing => (
                "durée absente : durée par défaut",
                "missing duration: default used",
            ),
            Self::DateUnreadable => ("date illisible", "unreadable date"),
            Self::DateOrderAssumed => (
                "ordre jour/mois déduit de la langue de l'en-tête (aucune date ne le lève)",
                "day/month order taken from the header language (no date settles it)",
            ),
            Self::DateOrderConflict => (
                "dates incompatibles entre elles : certaines sont ignorées",
                "inconsistent dates: some are ignored",
            ),
            Self::FinishBeforeStart => ("fin lue avant le début lu", "finish read before start"),
            Self::PredecessorUnreadable => (
                "prédécesseur illisible, ignoré",
                "unreadable predecessor, ignored",
            ),
            Self::PredecessorExternal => (
                "prédécesseur d'un autre projet, ignoré",
                "predecessor from another project, ignored",
            ),
            Self::PredecessorUnknown => (
                "prédécesseur absent du tableau, ignoré",
                "predecessor not in the table, ignored",
            ),
            Self::PredecessorSelf => (
                "tâche prédécesseur d'elle-même, ignoré",
                "self predecessor, ignored",
            ),
            Self::LevelJump => (
                "retrait de plus d'un niveau : ramené au niveau suivant",
                "indent of more than one level: reduced to the next level",
            ),
            Self::StrayLines => (
                "lignes de texte du tableau rattachées à aucune tâche",
                "table text lines attached to no task",
            ),
            Self::Truncated => (
                "bornes de lecture atteintes : contenu partiel",
                "read limits reached: partial content",
            ),
            Self::PagesSkipped => (
                "pages au-delà de la borne non lues",
                "pages beyond the limit not read",
            ),
            Self::LinkRefused => (
                "lien refusé par le moteur (boucle), ignoré",
                "link refused by the engine (loop), ignored",
            ),
            Self::ConstraintRefused => (
                "contrainte refusée par le moteur",
                "constraint refused by the engine",
            ),
            Self::Flattened => (
                "hiérarchie non lisible : toutes les tâches au niveau 1 (option « aplatir »)",
                "unreadable outline: all tasks at level 1 (\"flatten\" option)",
            ),
        };
        match lang {
            Language::Fr => fr,
            Language::En => en,
        }
    }
}

/// Origine du choix de l'ordre jour/mois.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DateOrderSource {
    /// Imposé par l'appelant.
    Forced,
    /// Les dates lues ne se lisent que dans cet ordre (jour > 12, jour de
    /// la semaine).
    Values,
    /// Aucune date ne tranche : langue de l'en-tête.
    Language,
    /// Aucune date numérique.
    None,
}

/// Couleur d'aplat de la zone graphique et nombre de barres.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct BarColor {
    /// Couleur.
    #[serde(serialize_with = "serialize_rgb")]
    pub color: Rgb,
    /// Aplats de cette couleur.
    pub count: u32,
}

#[allow(clippy::trivially_copy_pass_by_ref, reason = "signature imposée par serde")]
fn serialize_rgb<S: serde::Serializer>(c: &Rgb, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str(&c.hex())
}

/// Ce que la page donne à voir, gardé pour la copie du style (lot P2).
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct SourceInfo {
    /// Pages du document.
    pub pages: usize,
    /// Pages lues.
    pub pages_read: usize,
    /// Pages qui portent le tableau (1 = première).
    pub table_pages: Vec<usize>,
    /// Producteur déclaré.
    pub producer: Option<String>,
    /// Créateur déclaré.
    pub creator: Option<String>,
    /// Format de la première page du tableau, en centièmes de point.
    pub page_size: (Cpt, Cpt),
    /// Colonnes de la première page du tableau.
    pub columns: Vec<Column>,
    /// Couleurs des aplats de la zone graphique (barres), les plus
    /// fréquentes d'abord, 8 au plus.
    pub bar_colors: Vec<BarColor>,
}

/// Planning lu dans le PDF.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportedPlan {
    /// Langue des intitulés.
    pub language: Option<HeaderLanguage>,
    /// Ordre jour/mois retenu pour les dates numériques.
    pub date_order: Option<NumericOrder>,
    /// Origine de ce choix.
    pub date_order_source: DateOrderSource,
    /// Tâches, dans l'ordre du tableau (lignes vides et récapitulative du
    /// projet exclues).
    pub tasks: Vec<ImportedTask>,
    /// Avertissements, dans l'ordre des tâches.
    pub warnings: Vec<Warning>,
    /// Source.
    pub source: SourceInfo,
    /// Mesures.
    pub stats: ImportStats,
    /// Liens relus dans le dessin du Gantt (lot P1c), qu'il y ait ou non
    /// une colonne Prédécesseurs.
    pub drawn: DrawnLinks,
}

/// Mesures de la lecture.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
pub struct ImportStats {
    /// Plus grand N° lu (tâches attendues si rien n'est replié).
    pub max_id: u32,
    /// Lignes lues (avant dédoublonnage).
    pub rows_read: u32,
    /// Lignes en double (tableau répété sur plusieurs pages).
    pub duplicate_rows: u32,
    /// Lignes vides.
    pub blank_rows: u32,
    /// Niveaux distincts.
    pub levels: u16,
    /// Niveaux tirés de la colonne WBS / EDT (sinon du retrait).
    pub levels_from_wbs: bool,
    /// Récapitulatives dont les dates lues contiennent celles de leurs
    /// enfants, sur celles qui ont des dates lues.
    pub summaries_consistent: u32,
    /// Récapitulatives contrôlées.
    pub summaries_checked: u32,
    /// Durées lues.
    pub durations_read: u32,
    /// Dates de début lues.
    pub starts_read: u32,
    /// Dates de fin lues.
    pub finishes_read: u32,
    /// Liens lus et gardés.
    pub links_read: u32,
    /// Éléments de prédécesseurs refusés.
    pub links_dropped: u32,
    /// Lignes douteuses.
    pub doubtful_rows: u32,
    /// Lignes de texte non rattachées.
    pub stray_lines: u32,
    /// Lignes dont une cellule débordait et a été ramenée à sa valeur
    /// (texte en trop rendu au nom, ou écarté).
    pub cells_trimmed: u32,
}

/// Lignes et mise en page par page de tableau.
struct TablePage {
    page: usize,
    layout: TableLayout,
    rows: Vec<RawRow>,
    stray: u32,
}

/// Borne la valeur citée dans un avertissement.
fn cite(s: &str) -> String {
    s.chars().take(120).collect()
}

/// Construit le planning lu.
#[allow(
    clippy::too_many_lines,
    reason = "pipeline linéaire, étapes commentées dans l'ordre"
)]
///
/// # Errors
///
/// Refus du document entier ([`ImportError`]) : texte absent, numérisé,
/// pas de tableau, pas de dates, fausse hiérarchie, trop de tâches.
pub fn build(extracted: &Extracted, options: &ImportOptions) -> Result<ImportedPlan, ImportError> {
    let glyphs: u64 = extracted.pages.iter().map(|p| p.glyphs).sum();
    let visible: u64 = extracted
        .pages
        .iter()
        .map(|p| {
            p.glyphs
                .saturating_sub(p.invisible_glyphs)
                .saturating_sub(p.unmapped_glyphs)
        })
        .sum();
    let imaged = extracted.pages.iter().filter(|p| p.image_permille >= 500).count();
    if visible < 20 {
        if imaged > 0 || glyphs > 0 && extracted.pages.iter().all(|p| p.invisible_glyphs >= p.glyphs / 2) {
            return Err(ImportError::Scanned);
        }
        return Err(ImportError::NoText);
    }
    let mut warnings: Vec<Warning> = Vec::new();
    if extracted.page_count > extracted.pages.len() {
        warnings.push(Warning {
            code: WarningCode::PagesSkipped,
            id: None,
            page: None,
            value: Some(format!("{}/{}", extracted.pages.len(), extracted.page_count)),
        });
    }
    if extracted.pages.iter().any(|p| p.truncated) {
        warnings.push(Warning {
            code: WarningCode::Truncated,
            id: None,
            page: extracted
                .pages
                .iter()
                .find(|p| p.truncated)
                .map(|p| p.index.saturating_add(1)),
            value: None,
        });
    }
    let mut tables: Vec<TablePage> = Vec::new();
    let mut partial_header = false;
    for page in &extracted.pages {
        let lines = group_lines(&page.words);
        let Some(layout) = find_table(page, &lines) else {
            partial_header |= has_partial_header(&lines);
            continue;
        };
        let rows = read_rows(page.index, &lines, &layout);
        tables.push(TablePage {
            page: page.index,
            layout,
            rows: rows.rows,
            stray: rows.stray_lines,
        });
    }
    let Some(first) = tables.first() else {
        if partial_header {
            return Err(ImportError::NoDates);
        }
        if imaged > 0 && imaged.saturating_mul(2) >= extracted.pages.len() {
            return Err(ImportError::Scanned);
        }
        return Err(ImportError::NoTable);
    };
    let has_dates = tables.iter().any(|t| {
        t.layout.column(ColumnKind::Start).is_some() || t.layout.column(ColumnKind::Finish).is_some()
    });
    if !has_dates {
        return Err(ImportError::NoDates);
    }
    let language = first.layout.language;
    let has_id = first.layout.column(ColumnKind::Id).is_some();
    let lang = match language {
        Some(HeaderLanguage::En) => Language::En,
        _ => Language::Fr,
    };
    let options_model = ProjectOptions::default();
    let ctx = TextContext::for_options(&options_model, lang);
    let mut stats = ImportStats {
        stray_lines: tables.iter().map(|t| t.stray).sum(),
        ..ImportStats::default()
    };

    // Dédoublonnage et lignes vides.
    let mut rows: Vec<RawRow> = Vec::new();
    let mut seen: BTreeMap<u32, usize> = BTreeMap::new();
    for t in &tables {
        for row in &t.rows {
            stats.rows_read = stats.rows_read.saturating_add(1);
            if has_id {
                let Some(id) = row.cell(ColumnKind::Id).and_then(parse_id) else {
                    continue;
                };
                if let Some(&k) = seen.get(&id) {
                    let same = rows.get(k).is_some_and(|r| same_content(r, row));
                    if same {
                        stats.duplicate_rows = stats.duplicate_rows.saturating_add(1);
                    } else {
                        warnings.push(Warning {
                            code: WarningCode::IdConflict,
                            id: Some(id),
                            page: Some(row.page.saturating_add(1)),
                            value: row.cell(ColumnKind::Name).map(cite),
                        });
                    }
                    continue;
                }
                seen.insert(id, rows.len());
            } else if rows.iter().any(|r| r.page != row.page && same_content(r, row)) {
                stats.duplicate_rows = stats.duplicate_rows.saturating_add(1);
                continue;
            }
            rows.push(row.clone());
        }
    }
    // Ordre des N° (pages lues dans un autre ordre que le tableau).
    if has_id {
        rows.sort_by_key(|r| r.cell(ColumnKind::Id).and_then(parse_id).unwrap_or(u32::MAX));
    }
    let mut kept: Vec<(u32, RawRow)> = Vec::new();
    let mut sequence: u32 = 0;
    for row in rows {
        sequence = sequence.saturating_add(1);
        let id = if has_id {
            row.cell(ColumnKind::Id).and_then(parse_id).unwrap_or(sequence)
        } else {
            sequence
        };
        stats.max_id = stats.max_id.max(id);
        if id == 0 {
            // Récapitulative du projet (N° 0) : ses dates bornent le
            // projet, elle n'est pas une tâche.
            continue;
        }
        let blank = row.cell(ColumnKind::Name).is_none()
            && [ColumnKind::Duration, ColumnKind::Start, ColumnKind::Finish]
                .iter()
                .all(|k| row.cell(*k).is_none());
        if blank {
            stats.blank_rows = stats.blank_rows.saturating_add(1);
            continue;
        }
        kept.push((id, row));
    }
    if kept.is_empty() {
        return Err(ImportError::NoTasks);
    }
    if kept.len() > options.limits.max_tasks {
        return Err(ImportError::TooManyTasks(kept.len()));
    }
    if has_id {
        let ids: BTreeSet<u32> = kept.iter().map(|(id, _)| *id).collect();
        let missing = (1..=stats.max_id)
            .filter(|i| !ids.contains(i))
            .count()
            .saturating_sub(usize::try_from(stats.blank_rows).unwrap_or(0));
        if missing > 0 {
            warnings.push(Warning {
                code: WarningCode::IdGap,
                id: None,
                page: None,
                value: Some(missing.to_string()),
            });
        }
    }

    // Cellules débordantes ramenées à leur valeur typée.
    let mut trimmed: BTreeSet<u32> = BTreeSet::new();
    for (id, row) in &mut kept {
        if refine(row, &ctx) {
            trimmed.insert(*id);
        }
    }
    stats.cells_trimmed = u32::try_from(trimmed.len()).unwrap_or(u32::MAX);

    // Niveaux.
    let (levels, from_wbs, distinct) = levels_of(&kept);
    stats.levels = distinct;
    stats.levels_from_wbs = from_wbs;
    let flat_titles = distinct == 1 && !from_wbs && {
        let bold = kept.iter().filter(|(_, r)| r.bold).count();
        bold > 0 && bold < kept.len()
    };
    if flat_titles && !options.allow_flat_hierarchy {
        return Err(ImportError::FlatHierarchy);
    }

    // Dates : ordre jour/mois sur tout le document.
    let tokens: Vec<(Option<DateToken>, Option<DateToken>)> = kept
        .iter()
        .map(|(_, r)| {
            (
                r.cell(ColumnKind::Start).and_then(values::date),
                r.cell(ColumnKind::Finish).and_then(values::date),
            )
        })
        .collect();
    let (date_order, date_order_source) = date_order(&tokens, language, options.date_order, &mut warnings);

    // Tâches.
    let ids: BTreeSet<u32> = kept.iter().map(|(id, _)| *id).collect();
    let mut tasks: Vec<ImportedTask> = Vec::with_capacity(kept.len());
    let mut previous_level: u16 = 0;
    for (k, ((id, row), (start_tok, finish_tok))) in kept.iter().zip(&tokens).enumerate() {
        let id = *id;
        let page = Some(row.page.saturating_add(1));
        let mut penalty: u32 = 0;
        let mut warn =
            |code: WarningCode, value: Option<&str>, penalty_points: u32, warnings: &mut Vec<Warning>| {
                penalty = penalty.saturating_add(penalty_points);
                warnings.push(Warning {
                    code,
                    id: Some(id),
                    page,
                    value: value.map(cite),
                });
            };
        let mut level = levels.get(k).copied().unwrap_or(1);
        if level > previous_level.saturating_add(1) {
            warn(WarningCode::LevelJump, None, 15, &mut warnings);
            level = previous_level.saturating_add(1);
        }
        previous_level = level;
        let summary = levels.get(k.saturating_add(1)).is_some_and(|next| *next > level);
        let duration_text = row.cell(ColumnKind::Duration).unwrap_or("").to_owned();
        let duration = if duration_text.is_empty() {
            if !summary && first.layout.column(ColumnKind::Duration).is_some() {
                warn(WarningCode::DurationMissing, None, 20, &mut warnings);
            }
            None
        } else {
            let d = values::duration(&duration_text, &ctx);
            if d.is_none() {
                warn(
                    WarningCode::DurationUnreadable,
                    Some(&duration_text),
                    25,
                    &mut warnings,
                );
            }
            d
        };
        let mut moment =
            |tok: &Option<DateToken>, text: Option<&str>, finish: bool, warnings: &mut Vec<Warning>| {
                let text = text.unwrap_or("");
                if text.trim().is_empty() || is_na(text) {
                    return None;
                }
                let m = tok.and_then(|t| {
                    to_moment(
                        &t,
                        date_order.unwrap_or(NumericOrder::DayMonth),
                        finish,
                        &options_model,
                    )
                });
                if m.is_none() {
                    penalty = penalty.saturating_add(15);
                    warnings.push(Warning {
                        code: WarningCode::DateUnreadable,
                        id: Some(id),
                        page,
                        value: Some(cite(text)),
                    });
                }
                m
            };
        let start = moment(start_tok, row.cell(ColumnKind::Start), false, &mut warnings);
        let finish = moment(finish_tok, row.cell(ColumnKind::Finish), true, &mut warnings);
        if let (Some(s), Some(f)) = (start, finish)
            && f < s
        {
            penalty = penalty.saturating_add(15);
            warnings.push(Warning {
                code: WarningCode::FinishBeforeStart,
                id: Some(id),
                page,
                value: None,
            });
        }
        let predecessors_text = row.cell(ColumnKind::Predecessors).unwrap_or("").to_owned();
        let mut predecessors = Vec::new();
        for parsed in values::predecessors(&predecessors_text, &ctx) {
            let (code, value) = match parsed {
                Ok(p) if p.id == id => (WarningCode::PredecessorSelf, p.id.to_string()),
                Ok(p) if !ids.contains(&p.id) => (WarningCode::PredecessorUnknown, p.id.to_string()),
                Ok(p) if predecessors.iter().any(|q: &PredRef| q.id == p.id) => continue,
                Ok(p) => {
                    predecessors.push(p);
                    continue;
                }
                Err((text, PredError::External)) => (WarningCode::PredecessorExternal, text),
                Err((text, PredError::Unreadable)) => (WarningCode::PredecessorUnreadable, text),
            };
            stats.links_dropped = stats.links_dropped.saturating_add(1);
            penalty = penalty.saturating_add(if code == WarningCode::PredecessorUnknown {
                5
            } else {
                15
            });
            warnings.push(Warning {
                code,
                id: Some(id),
                page,
                value: Some(cite(&value)),
            });
        }
        if row.text_lines > 2 {
            penalty = penalty.saturating_add(10);
        }
        if trimmed.contains(&id) {
            penalty = penalty.saturating_add(5);
        }
        let name = row.cell(ColumnKind::Name).unwrap_or("").to_owned();
        if name.is_empty() {
            penalty = penalty.saturating_add(30);
        } else if name.ends_with('…') || name.ends_with("...") {
            penalty = penalty.saturating_add(5);
            warnings.push(Warning {
                code: WarningCode::NameTruncated,
                id: Some(id),
                page,
                value: Some(cite(&name)),
            });
        }
        stats.durations_read = stats.durations_read.saturating_add(u32::from(duration.is_some()));
        stats.starts_read = stats.starts_read.saturating_add(u32::from(start.is_some()));
        stats.finishes_read = stats.finishes_read.saturating_add(u32::from(finish.is_some()));
        stats.links_read = stats
            .links_read
            .saturating_add(u32::try_from(predecessors.len()).unwrap_or(u32::MAX));
        let confidence = u8::try_from(100_u32.saturating_sub(penalty)).unwrap_or(0);
        tasks.push(ImportedTask {
            id,
            name,
            level,
            summary,
            duration,
            duration_text,
            start,
            start_text: row.cell(ColumnKind::Start).unwrap_or("").to_owned(),
            finish,
            finish_text: row.cell(ColumnKind::Finish).unwrap_or("").to_owned(),
            predecessors,
            predecessors_text,
            successors_text: row.cell(ColumnKind::Successors).unwrap_or("").to_owned(),
            resources: row.cell(ColumnKind::ResourceNames).unwrap_or("").to_owned(),
            percent_complete: row.cell(ColumnKind::PercentComplete).and_then(values::percent),
            bold: row.bold,
            page: row.page,
            confidence,
        });
    }
    if stats.starts_read == 0 && stats.finishes_read == 0 {
        return Err(ImportError::NoDates);
    }
    if stats.stray_lines > 0 {
        warnings.push(Warning {
            code: WarningCode::StrayLines,
            id: None,
            page: None,
            value: Some(stats.stray_lines.to_string()),
        });
    }
    stats.doubtful_rows = u32::try_from(tasks.iter().filter(|t| t.doubtful()).count()).unwrap_or(u32::MAX);
    let (consistent, checked) = summary_consistency(&tasks);
    stats.summaries_consistent = consistent;
    stats.summaries_checked = checked;
    // Fausse hiérarchie : la plupart des récapitulatives lues ne
    // contiennent pas les dates de leurs enfants (retraits trompeurs).
    if checked >= 3 && consistent.saturating_mul(2) < checked && !from_wbs {
        if !options.allow_flat_hierarchy {
            return Err(ImportError::InconsistentHierarchy);
        }
        flatten(&mut tasks, &mut warnings);
        stats.levels = 1;
    } else if flat_titles && options.allow_flat_hierarchy {
        flatten(&mut tasks, &mut warnings);
    }
    let source = source_info(extracted, &tables);
    let drawn = drawn_links(extracted, &tables, &kept, has_id, &tasks, &options.limits.links);
    Ok(ImportedPlan {
        language,
        date_order,
        date_order_source,
        tasks,
        warnings,
        source,
        stats,
        drawn,
    })
}

/// Plus longue suite de mots `words[i..j]` (la plus à gauche à longueur
/// égale) acceptée par `accept`, au plus `max` mots.
fn best_window(words: &[&str], max: usize, accept: impl Fn(&str) -> bool) -> Option<(usize, usize)> {
    let n = words.len();
    for len in (1..=n.min(max)).rev() {
        for i in 0..=n.saturating_sub(len) {
            let j = i.saturating_add(len);
            if let Some(w) = words.get(i..j)
                && accept(&w.join(" "))
            {
                return Some((i, j));
            }
        }
    }
    None
}

/// Ramène les cellules qui débordent à leur valeur typée (bornes de
/// colonnes approchées, texte du graphique voisin) : durée = plus longue
/// fin lisible (le début revient au nom) ; dates = plus longue suite
/// lisible (le reste d'un début va à la fin vide) ; prédécesseurs = plus
/// long début lisible. Rend vrai si une cellule a changé.
fn refine(row: &mut RawRow, ctx: &TextContext) -> bool {
    let mut changed = false;
    // Deux cellules collées (« Pip12 wks », « 12/19/19W ») : espace
    // ajoutée entre lettre et chiffre pour la recherche seulement.
    let spaced = |t: &str| -> String {
        let mut out = String::with_capacity(t.len());
        let mut prev: Option<char> = None;
        for c in t.chars() {
            if let Some(p) = prev
                && ((p.is_alphabetic() && c.is_ascii_digit()) || (p.is_ascii_digit() && c.is_alphabetic()))
            {
                out.push(' ');
            }
            out.push(c);
            prev = Some(c);
        }
        out
    };
    let text = |row: &RawRow, k: ColumnKind| row.cell(k).map(str::to_owned);
    if let Some(d) = text(row, ColumnKind::Duration)
        && values::duration(&d, ctx).is_none()
    {
        let d = spaced(&d);
        let words: Vec<&str> = d.split_whitespace().collect();
        let found = (1..words.len()).find(|i| {
            words
                .get(*i..)
                .is_some_and(|w| values::duration(&w.join(" "), ctx).is_some())
        });
        if let Some(i) = found {
            let head = words.get(..i).unwrap_or_default().join(" ");
            let tail = words.get(i..).unwrap_or_default().join(" ");
            let name = row.cells.entry(ColumnKind::Name).or_default();
            if !name.is_empty() {
                name.push(' ');
            }
            name.push_str(&head);
            row.cells.insert(ColumnKind::Duration, tail);
            changed = true;
        }
    }
    for kind in [ColumnKind::Start, ColumnKind::Finish] {
        let Some(d) = text(row, kind) else { continue };
        if values::date(&d).is_some() || is_na(&d) {
            continue;
        }
        let d = spaced(&d);
        let words: Vec<&str> = d.split_whitespace().collect();
        let Some((i, j)) = best_window(&words, 6, |t| values::date(t).is_some()) else {
            continue;
        };
        let date = words.get(i..j).unwrap_or_default().join(" ");
        let rest = words.get(j..).unwrap_or_default().join(" ");
        row.cells.insert(kind, date);
        changed = true;
        if kind == ColumnKind::Start && row.cell(ColumnKind::Finish).is_none() {
            let rest_words: Vec<&str> = rest.split_whitespace().collect();
            if let Some((a, b)) = best_window(&rest_words, 6, |t| values::date(t).is_some()) {
                row.cells.insert(
                    ColumnKind::Finish,
                    rest_words.get(a..b).unwrap_or_default().join(" "),
                );
            }
        }
    }
    if let Some(p) = text(row, ColumnKind::Predecessors) {
        let ok = |t: &str| values::predecessors(t, ctx).iter().all(Result::is_ok);
        if !ok(&p) {
            let words: Vec<&str> = p.split_whitespace().collect();
            let found = (1..words.len())
                .rev()
                .find(|k| words.get(..*k).is_some_and(|w| ok(&w.join(" "))));
            if let Some(k) = found {
                row.cells.insert(
                    ColumnKind::Predecessors,
                    words.get(..k).unwrap_or_default().join(" "),
                );
                changed = true;
            }
        }
    }
    changed
}

/// Tout au niveau 1 (option « aplatir ») : plus de récapitulative.
fn flatten(tasks: &mut [ImportedTask], warnings: &mut Vec<Warning>) {
    for t in tasks.iter_mut() {
        t.level = 1;
        t.summary = false;
    }
    warnings.retain(|w| w.code != WarningCode::LevelJump);
    warnings.push(Warning {
        code: WarningCode::Flattened,
        id: None,
        page: None,
        value: None,
    });
}

/// Valeur « NA » / « ND » / « tba » : pas de date, sans avertissement.
fn is_na(text: &str) -> bool {
    matches!(
        fold(text).as_str(),
        "na" | "n/a" | "nd" | "n/d" | "tba" | "tbd" | "-"
    )
}

/// Mêmes cellules (tableau répété sur une autre page) ; le rappel d'une
/// récapitulative en haut de page, « Phase 1 (suite) » / « (continued) »,
/// est la même ligne.
fn same_content(a: &RawRow, b: &RawRow) -> bool {
    let strip = |r: &RawRow| -> BTreeMap<ColumnKind, String> {
        r.cells
            .iter()
            .map(|(k, v)| {
                let v = if *k == ColumnKind::Name {
                    let t = v.trim_end();
                    t.strip_suffix("(suite)")
                        .or_else(|| t.strip_suffix("(continued)"))
                        .unwrap_or(t)
                        .trim_end()
                        .to_owned()
                } else {
                    v.clone()
                };
                (*k, v)
            })
            .collect()
    };
    strip(a) == strip(b)
}

/// En-tête incomplet : un nom de tâche et un N° reconnus sur une même
/// ligne, sans durée ni dates (refus « pas de dates »).
fn has_partial_header(lines: &[crate::lines::TextLine]) -> bool {
    lines.iter().any(|l| {
        let kinds: Vec<ColumnKind> = l
            .words
            .iter()
            .filter_map(|w| classify(&fold(&w.text)).map(|c| c.0))
            .collect();
        let joined = fold(&crate::lines::join_words(&l.words));
        let name = kinds.contains(&ColumnKind::Name)
            || joined.contains("nom de la tache")
            || joined.contains("task name");
        // Des intitulés de dates présents mais non reconnus comme un tableau
        // de tâches (autre outil : « Activity Name ») : pas ce cas-là.
        let dates = kinds
            .iter()
            .any(|k| matches!(k, ColumnKind::Start | ColumnKind::Finish | ColumnKind::Duration));
        name && kinds.contains(&ColumnKind::Id) && !dates
    })
}

/// Niveaux des tâches gardées : colonne WBS / EDT si au moins 80 % des
/// lignes y portent un numéro hiérarchique (`1.2.3`), sinon rang du
/// retrait du nom parmi les retraits distincts (à 2,5 pt près). Rend les
/// niveaux, leur origine et le nombre de niveaux distincts.
fn levels_of(kept: &[(u32, RawRow)]) -> (Vec<u16>, bool, u16) {
    let wbs: Vec<Option<u16>> = kept
        .iter()
        .map(|(_, r)| {
            let t = r.cell(ColumnKind::Wbs)?.trim().trim_end_matches('.');
            let parts: Vec<&str> = t.split('.').collect();
            if parts
                .iter()
                .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
            {
                u16::try_from(parts.len()).ok()
            } else {
                None
            }
        })
        .collect();
    let with_wbs = wbs.iter().filter(|w| w.is_some()).count();
    if !kept.is_empty() && with_wbs.saturating_mul(5) >= kept.len().saturating_mul(4) {
        let levels: Vec<u16> = wbs.iter().map(|w| w.unwrap_or(1)).collect();
        let distinct = u16::try_from(levels.iter().collect::<BTreeSet<_>>().len()).unwrap_or(u16::MAX);
        return (levels, true, distinct);
    }
    let mut indents: Vec<Cpt> = kept.iter().filter_map(|(_, r)| r.name_indent).collect();
    indents.sort_unstable();
    indents.dedup();
    let tolerance: Cpt = 250;
    // Débuts de groupes : un retrait à plus de 2,5 pt du précédent.
    let mut starts: Vec<Cpt> = Vec::new();
    let mut last: Option<Cpt> = None;
    for i in indents {
        if last.is_none_or(|l| i.saturating_sub(l) > tolerance) {
            starts.push(i);
        }
        last = Some(i);
    }
    let level_of = |indent: Cpt| -> u16 {
        let rank = starts.iter().filter(|s| **s <= indent).count();
        u16::try_from(rank.max(1)).unwrap_or(u16::MAX)
    };
    let levels = kept
        .iter()
        .map(|(_, r)| r.name_indent.map_or(1, level_of))
        .collect();
    (
        levels,
        false,
        u16::try_from(starts.len().max(1)).unwrap_or(u16::MAX),
    )
}

/// Choisit l'ordre jour/mois des dates numériques du document.
fn date_order(
    tokens: &[(Option<DateToken>, Option<DateToken>)],
    language: Option<HeaderLanguage>,
    forced: Option<NumericOrder>,
    warnings: &mut Vec<Warning>,
) -> (Option<NumericOrder>, DateOrderSource) {
    let all: Vec<&DateToken> = tokens
        .iter()
        .flat_map(|(a, b)| [a.as_ref(), b.as_ref()])
        .flatten()
        .collect();
    let numeric = all
        .iter()
        .filter(|t| matches!(t.form, values::DateForm::Numeric { .. }))
        .count();
    if let Some(order) = forced {
        return (Some(order), DateOrderSource::Forced);
    }
    if numeric == 0 {
        return (None, DateOrderSource::None);
    }
    let total = all.len();
    let dm = all.iter().filter(|t| t.fits(NumericOrder::DayMonth)).count();
    let md = all.iter().filter(|t| t.fits(NumericOrder::MonthDay)).count();
    let default = match language {
        Some(HeaderLanguage::En) => NumericOrder::MonthDay,
        _ => NumericOrder::DayMonth,
    };
    match (dm == total, md == total) {
        (true, false) => (Some(NumericOrder::DayMonth), DateOrderSource::Values),
        (false, true) => (Some(NumericOrder::MonthDay), DateOrderSource::Values),
        (true, true) => {
            if all.iter().any(|t| t.is_ambiguous()) {
                warnings.push(Warning {
                    code: WarningCode::DateOrderAssumed,
                    id: None,
                    page: None,
                    value: Some(
                        match default {
                            NumericOrder::DayMonth => "jj/mm",
                            NumericOrder::MonthDay => "mm/dd",
                        }
                        .to_owned(),
                    ),
                });
                (Some(default), DateOrderSource::Language)
            } else {
                (Some(default), DateOrderSource::Values)
            }
        }
        (false, false) => {
            warnings.push(Warning {
                code: WarningCode::DateOrderConflict,
                id: None,
                page: None,
                value: Some(format!("{dm}/{md}/{total}")),
            });
            let order = match dm.cmp(&md) {
                std::cmp::Ordering::Greater => NumericOrder::DayMonth,
                std::cmp::Ordering::Less => NumericOrder::MonthDay,
                std::cmp::Ordering::Equal => default,
            };
            (Some(order), DateOrderSource::Values)
        }
    }
}

/// Instant d'une date lue : jour dans l'ordre retenu (et cohérent avec le
/// jour de la semaine lu), heure lue sinon heure par défaut du rôle
/// (R-033 : 08:00 au début, 17:00 à la fin).
fn to_moment(tok: &DateToken, order: NumericOrder, finish: bool, options: &ProjectOptions) -> Option<Moment> {
    if !tok.fits(order) {
        return None;
    }
    let day = Day::from_naive(tok.day(order)?).ok()?;
    let time = tok.time.unwrap_or(if finish {
        options.default_finish_time
    } else {
        options.default_start_time
    });
    Moment::at(day, time).ok()
}

/// Récapitulatives dont les dates lues (au jour) contiennent celles de
/// leurs descendants lus : (cohérentes, contrôlées).
fn summary_consistency(tasks: &[ImportedTask]) -> (u32, u32) {
    let mut consistent = 0_u32;
    let mut checked = 0_u32;
    for (i, t) in tasks.iter().enumerate() {
        if !t.summary {
            continue;
        }
        let (Some(s), Some(f)) = (t.start, t.finish) else {
            continue;
        };
        let children = tasks
            .iter()
            .skip(i.saturating_add(1))
            .take_while(|c| c.level > t.level);
        let mut ok = true;
        let mut any = false;
        for c in children {
            if let Some(cs) = c.start {
                any = true;
                ok &= cs.day() >= s.day();
            }
            if let Some(cf) = c.finish {
                any = true;
                ok &= cf.day() <= f.day();
            }
        }
        if any {
            checked = checked.saturating_add(1);
            consistent = consistent.saturating_add(u32::from(ok));
        }
    }
    (consistent, checked)
}

/// Liens dessinés (lot P1c) : lignes du tableau de chaque page (N° lus,
/// lignes répétées comprises ; sans colonne N°, lignes gardées), zone
/// graphique à droite du tableau.
fn drawn_links(
    extracted: &Extracted,
    tables: &[TablePage],
    kept: &[(u32, RawRow)],
    has_id: bool,
    tasks: &[ImportedTask],
    limits: &crate::links::LinkLimits,
) -> DrawnLinks {
    let known: BTreeSet<u32> = tasks.iter().map(|t| t.id).collect();
    let frames: Vec<PageFrame<'_>> = tables
        .iter()
        .filter_map(|t| {
            let page = extracted.pages.iter().find(|p| p.index == t.page)?;
            let rows: Vec<RowRef> = if has_id {
                t.rows
                    .iter()
                    .filter_map(|r| {
                        Some(RowRef {
                            baseline: r.baseline,
                            size: r.name_size,
                            id: r.cell(ColumnKind::Id).and_then(parse_id)?,
                        })
                    })
                    .collect()
            } else {
                kept.iter()
                    .filter(|(_, r)| r.page == t.page)
                    .map(|(id, r)| RowRef {
                        baseline: r.baseline,
                        size: r.name_size,
                        id: *id,
                    })
                    .collect()
            };
            // Bord de la zone graphique : bord droit de la dernière colonne
            // si le quadrillage le donne ; sinon (bornes tirées des vides,
            // la dernière colonne court jusqu'au bord de la page) son bord
            // gauche.
            let last = t.layout.columns.iter().max_by_key(|c| c.x1);
            let chart_left = last.map_or(0, |c| if c.ruled { c.x1 } else { c.x0 });
            Some(PageFrame {
                page,
                chart_left,
                top: t.layout.header_bottom,
                rows,
            })
        })
        .collect();
    crate::links::read(&frames, &known, limits)
}

/// Format, colonnes et couleurs des barres (zone graphique des pages du
/// tableau, aplats rectangulaires de hauteur 3 à 20 pt, blanc et noir
/// exclus).
fn source_info(extracted: &Extracted, tables: &[TablePage]) -> SourceInfo {
    let first = tables.first();
    let page = first.and_then(|t| extracted.pages.iter().find(|p| p.index == t.page));
    let mut colors: BTreeMap<Rgb, u32> = BTreeMap::new();
    for t in tables {
        let Some(p) = extracted.pages.iter().find(|p| p.index == t.page) else {
            continue;
        };
        let right = t.layout.right();
        for r in &p.rects {
            let h = r.y1.saturating_sub(r.y0);
            let [red, green, blue] = r.color.0;
            let white = red > 240 && green > 240 && blue > 240;
            let black = red < 16 && green < 16 && blue < 16;
            if r.is_rect && r.x0 >= right && (300..=2000).contains(&h) && !white && !black {
                let c = colors.entry(r.color).or_insert(0);
                *c = c.saturating_add(1);
            }
        }
    }
    let mut bar_colors: Vec<BarColor> = colors
        .into_iter()
        .map(|(color, count)| BarColor { color, count })
        .collect();
    bar_colors.sort_by(|a, b| b.count.cmp(&a.count).then(a.color.cmp(&b.color)));
    bar_colors.truncate(8);
    SourceInfo {
        pages: extracted.page_count,
        pages_read: extracted.pages.len(),
        table_pages: tables.iter().map(|t| t.page.saturating_add(1)).collect(),
        producer: extracted.producer.clone(),
        creator: extracted.creator.clone(),
        page_size: page.map_or((0, 0), |p| (p.width, p.height)),
        columns: first.map(|t| t.layout.columns.clone()).unwrap_or_default(),
        bar_colors,
    }
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
    use crate::extract::{PageContent, Word};

    /// Bords gauches des colonnes N°, Nom, Durée, Début, Fin, Préd.
    const X: [Cpt; 6] = [400, 2000, 20000, 25000, 32000, 39000];

    fn word(text: &str, x0: Cpt, baseline: Cpt, bold: bool) -> Word {
        let n = Cpt::try_from(text.chars().count()).unwrap();
        Word {
            text: text.to_owned(),
            x0,
            x1: x0.saturating_add(n.saturating_mul(420)),
            baseline,
            size: 800,
            bold,
            glyphs: Vec::new(),
        }
    }

    /// Ligne : cellules (texte, retrait du nom en centièmes de point).
    struct Row<'a> {
        cells: [&'a str; 6],
        indent: Cpt,
        bold: bool,
    }

    fn page(header: [&str; 6], rows: &[Row<'_>]) -> Extracted {
        let mut words = Vec::new();
        let mut y: Cpt = 1000;
        let put = |cells: [&str; 6], indent: Cpt, bold: bool, y: Cpt, words: &mut Vec<Word>| {
            for (k, cell) in cells.iter().enumerate() {
                let mut x = X[k] + if k == 1 { indent } else { 0 };
                for w in cell.split_whitespace() {
                    let wd = word(w, x, y, bold && k == 1);
                    x = wd.x1 + 300;
                    words.push(wd);
                }
            }
        };
        put(header, 0, false, y, &mut words);
        for r in rows {
            y += 1200;
            put(r.cells, r.indent, r.bold, y, &mut words);
        }
        let content = PageContent {
            words,
            width: 84_000,
            height: 59_500,
            glyphs: 5_000,
            ..PageContent::default()
        };
        Extracted {
            pages: vec![content],
            page_count: 1,
            producer: None,
            creator: None,
        }
    }

    const FR: [&str; 6] = ["N°", "Nom de la tâche", "Durée", "Début", "Fin", "Prédécesseurs"];
    const EN: [&str; 6] = ["ID", "Task Name", "Duration", "Start", "Finish", "Predecessors"];

    fn row(cells: [&str; 6], indent: Cpt, bold: bool) -> Row<'_> {
        Row { cells, indent, bold }
    }

    #[test]
    fn levels_links_and_milestones_from_a_word_table() {
        let ex = page(
            FR,
            &[
                row(
                    ["1", "Lot 1", "10 jours", "Lun 05/01/26", "Ven 16/01/26", ""],
                    0,
                    true,
                ),
                row(
                    ["2", "Fouilles", "3 jrs", "Lun 05/01/26", "Mer 07/01/26", ""],
                    970,
                    false,
                ),
                row(
                    [
                        "3",
                        "Béton",
                        "1 sem",
                        "Jeu 08/01/26",
                        "Mer 14/01/26",
                        "2FD+0 jour",
                    ],
                    970,
                    false,
                ),
                row(
                    [
                        "4",
                        "Fin du lot",
                        "0 jour",
                        "Ven 16/01/26",
                        "Ven 16/01/26",
                        "3;2DD+1 j",
                    ],
                    970,
                    false,
                ),
                row(
                    ["5", "Réception", "0 jour", "Ven 16/01/26", "Ven 16/01/26", "4"],
                    0,
                    false,
                ),
            ],
        );
        let plan = build(&ex, &ImportOptions::default()).unwrap();
        let levels: Vec<u16> = plan.tasks.iter().map(|t| t.level).collect();
        assert_eq!(levels, [1, 2, 2, 2, 1]);
        assert!(plan.tasks[0].summary);
        assert!(plan.tasks[3].milestone() && plan.tasks[4].milestone());
        assert_eq!(plan.tasks[3].predecessors.len(), 2);
        assert_eq!(
            plan.tasks[3].predecessors[1].kind,
            plannercraft_core::LinkType::StartToStart
        );
        assert_eq!(plan.date_order, Some(NumericOrder::DayMonth));
        assert_eq!(plan.date_order_source, DateOrderSource::Values);
        assert_eq!(plan.stats.summaries_consistent, 1);
        assert_eq!(plan.stats.links_read, 4);
        assert!(plan.warnings.is_empty(), "{:?}", plan.warnings);
    }

    #[test]
    fn unknown_and_unreadable_predecessors_are_reported_not_imported() {
        let ex = page(
            EN,
            &[
                row(["1", "A", "1 day", "1/5/26", "1/5/26", ""], 0, false),
                row(["2", "B", "1 day", "1/6/26", "1/6/26", "1,9,2,zz"], 0, false),
            ],
        );
        let plan = build(&ex, &ImportOptions::default()).unwrap();
        assert_eq!(plan.tasks[1].predecessors.len(), 1);
        let codes: Vec<WarningCode> = plan.warnings.iter().map(|w| w.code).collect();
        assert!(codes.contains(&WarningCode::PredecessorUnknown));
        assert!(codes.contains(&WarningCode::PredecessorSelf));
        assert!(codes.contains(&WarningCode::PredecessorUnreadable));
    }

    #[test]
    fn ambiguous_dates_follow_the_header_language_with_a_warning() {
        let ex = page(
            EN,
            &[
                row(["1", "A", "1 day", "03/04/26", "03/04/26", ""], 0, false),
                row(["2", "B", "1 day", "05/06/26", "05/06/26", ""], 0, false),
            ],
        );
        let plan = build(&ex, &ImportOptions::default()).unwrap();
        assert_eq!(plan.date_order, Some(NumericOrder::MonthDay));
        assert_eq!(plan.date_order_source, DateOrderSource::Language);
        assert!(
            plan.warnings
                .iter()
                .any(|w| w.code == WarningCode::DateOrderAssumed)
        );
        assert_eq!(plan.tasks[0].start.unwrap().to_string(), "2026-03-04T08:00");
        // Imposé par l'appelant : pas d'avertissement.
        let forced = ImportOptions {
            date_order: Some(NumericOrder::DayMonth),
            ..ImportOptions::default()
        };
        let plan = build(&ex, &forced).unwrap();
        assert_eq!(plan.tasks[0].start.unwrap().to_string(), "2026-04-03T08:00");
        assert!(
            plan.warnings
                .iter()
                .all(|w| w.code != WarningCode::DateOrderAssumed)
        );
    }

    #[test]
    fn weekday_settles_the_order_of_uk_dates() {
        // 05/01/26 est un lundi en JJ/MM (5 janvier 2026), un vendredi en
        // MM/JJ (1er mai 2026) : « Mon » tranche malgré l'en-tête anglais.
        let ex = page(
            EN,
            &[row(
                ["1", "A", "1 day", "Mon 05/01/26", "Mon 05/01/26", ""],
                0,
                false,
            )],
        );
        let plan = build(&ex, &ImportOptions::default()).unwrap();
        assert_eq!(plan.date_order, Some(NumericOrder::DayMonth));
        assert_eq!(plan.date_order_source, DateOrderSource::Values);
    }

    #[test]
    fn flat_list_with_bold_titles_is_refused_unless_flattened() {
        let rows = [
            row(
                ["1", "LOT GROS OEUVRE", "10 jours", "05/01/26", "16/01/26", ""],
                0,
                true,
            ),
            row(["2", "Fouilles", "3 jours", "05/01/26", "07/01/26", ""], 0, false),
            row(
                ["3", "LOT CHARPENTE", "5 jours", "19/01/26", "23/01/26", ""],
                0,
                true,
            ),
            row(["4", "Pose", "5 jours", "19/01/26", "23/01/26", ""], 0, false),
        ];
        let ex = page(FR, &rows);
        assert_eq!(
            build(&ex, &ImportOptions::default()),
            Err(ImportError::FlatHierarchy)
        );
        let flat = ImportOptions {
            allow_flat_hierarchy: true,
            ..ImportOptions::default()
        };
        let plan = build(&ex, &flat).unwrap();
        assert!(plan.tasks.iter().all(|t| t.level == 1 && !t.summary));
        assert!(plan.warnings.iter().any(|w| w.code == WarningCode::Flattened));
    }

    #[test]
    fn indents_contradicting_summary_dates_are_refused() {
        // Feuilles au bord, récapitulatives en retrait (vu dans le corpus) :
        // les « récapitulatives » ne contiennent pas les dates des lignes
        // suivantes.
        let rows = [
            row(
                ["1", "Phase A", "5 jours", "05/01/26", "09/01/26", ""],
                970,
                false,
            ),
            row(["2", "a1", "5 jours", "12/01/26", "16/01/26", ""], 1940, false),
            row(
                ["3", "Phase B", "5 jours", "19/01/26", "23/01/26", ""],
                970,
                false,
            ),
            row(["4", "b1", "5 jours", "26/01/26", "30/01/26", ""], 1940, false),
            row(
                ["5", "Phase C", "5 jours", "02/02/26", "06/02/26", ""],
                970,
                false,
            ),
            row(["6", "c1", "5 jours", "09/02/26", "13/02/26", ""], 1940, false),
        ];
        let ex = page(FR, &rows);
        assert_eq!(
            build(&ex, &ImportOptions::default()),
            Err(ImportError::InconsistentHierarchy)
        );
    }

    #[test]
    fn table_without_dates_is_refused() {
        let ex = page(
            ["N°", "Nom de la tâche", "Durée", "", "", ""],
            &[row(["1", "A", "3 jours", "", "", ""], 0, false)],
        );
        assert_eq!(build(&ex, &ImportOptions::default()), Err(ImportError::NoDates));
    }

    #[test]
    fn scanned_and_textless_pages_are_refused() {
        let mut ex = page(FR, &[]);
        ex.pages[0].words.clear();
        ex.pages[0].glyphs = 0;
        assert_eq!(build(&ex, &ImportOptions::default()), Err(ImportError::NoText));
        ex.pages[0].image_permille = 1000;
        assert_eq!(build(&ex, &ImportOptions::default()), Err(ImportError::Scanned));
        // Texte reconnu (OCR) invisible sur une image pleine page.
        ex.pages[0].glyphs = 3000;
        ex.pages[0].invisible_glyphs = 3000;
        assert_eq!(build(&ex, &ImportOptions::default()), Err(ImportError::Scanned));
    }

    #[test]
    fn page_without_table_is_refused() {
        let mut ex = page(["Compte", "rendu", "de", "réunion", "du", "lundi"], &[]);
        ex.pages[0].glyphs = 5000;
        assert_eq!(build(&ex, &ImportOptions::default()), Err(ImportError::NoTable));
    }
}
