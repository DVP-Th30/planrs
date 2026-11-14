//! Fenêtre « Importer un PDF… » (lot P1b, ADR-0033, H-UI-205 à H-UI-212) :
//! aperçu d'un planning Gantt lu dans un PDF, réglages, calendrier (jours
//! fériés français, périodes chômées déduites des écarts), puis ouverture
//! comme un nouveau document non enregistré.
//!
//! Fichier → Importer un PDF… ouvre le sélecteur
//! ([`crate::FileService::pick_bytes`], comme l'import CSV) ; les octets
//! reçus sont lus **localement** par `plannercraft-pdfimport` (rien n'est
//! envoyé nulle part, sur le web non plus). Le calcul (lecture du PDF,
//! conversion par les commandes du moteur, déduction du calendrier) tourne
//! dans un fil dédié sur le bureau (`plannercraft-pdf-import`) et, sur le
//! web, à l'image qui suit l'affichage de « Lecture du PDF en cours… » ;
//! chaque réglage changé relance le calcul sans figer la fenêtre (bureau)
//! et l'aperçu affiché reste celui du dernier calcul fini. Le lecteur PDF
//! n'est relu que si l'ordre des dates ou « sans hiérarchie » changent.
//!
//! Règle 3 : aucune date n'est recopiée ; le tableau montre les dates lues
//! et les fins **calculées par le moteur** sur le calendrier choisi.
//!
//! Lot P1c (ADR-0034, H-UI-213 à H-UI-215) : liens relus dans le dessin du
//! Gantt quand le PDF n'a pas de colonne Prédécesseurs (ligne du résumé,
//! case « Recréer les liens dessinés », cochée d'office).

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use plannercraft_automation::text::Language as TextLanguage;
use plannercraft_core::{Day, Moment, Project, TimeOfDay};
use plannercraft_pdfimport::header::ColumnKind;
use plannercraft_pdfimport::{
    ConvertOptions, DeduceLimits, Deduction, ImportError, ImportLimits, ImportOptions, ImportedPlan,
    LinkDrop, NumericOrder, Warning, WarningCode, calendar, convert, deduce, import,
};

use crate::action::DialogKind;
use crate::app::{App, DocOrigin};
use crate::format::format_date;
use crate::i18n::{Arg, TextId, Texts, error_text_id};
use crate::io::PickRequest;

/// Taille au plus d'un PDF lu sur le web (Mio) : une panique du lecteur
/// n'y est pas interceptable, le risque est borné par la taille.
pub const WEB_MAX_MIB: usize = 16;
/// Pages lues au plus sur le web.
pub const WEB_MAX_PAGES: usize = 50;

/// Avertissements du moteur montrés au plus dans la fenêtre.
const MAX_WARNINGS_SHOWN: usize = 6;

/// Bornes de l'import : celles du moteur sur le bureau, resserrées sur le
/// web.
#[must_use]
pub fn import_limits() -> ImportLimits {
    let mut limits = ImportLimits::default();
    if cfg!(target_arch = "wasm32") {
        limits.max_bytes = WEB_MAX_MIB.saturating_mul(1024 * 1024);
        limits.extract.max_pages = WEB_MAX_PAGES;
    }
    limits
}

/// Valeur partagée comparée par identité (la fenêtre est clonable et
/// comparable ; ses calculs ne sont jamais recopiés).
pub struct Shared<T>(pub Arc<T>);

impl<T> Shared<T> {
    /// Partage `value`.
    pub fn new(value: T) -> Self {
        Self(Arc::new(value))
    }
}

impl<T> Clone for Shared<T> {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

impl<T> PartialEq for Shared<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Eq for Shared<T> {}

impl<T> std::fmt::Debug for Shared<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Shared(..)")
    }
}

/// Réglages qui pilotent un calcul.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PdfSettings {
    /// Ordre jour/mois imposé (`None` : décidé sur le document, R-273).
    pub date_order: Option<NumericOrder>,
    /// Importer sans hiérarchie (tout au niveau 1) un tableau refusé pour
    /// hiérarchie plate ou incohérente.
    pub flatten: bool,
    /// R-271 étendue : « Début au plus tôt le » aussi sur les tâches liées.
    pub pin_starts: bool,
    /// Jours fériés français (`None` : cochés si l'en-tête est français,
    /// R-274).
    pub holidays: Option<bool>,
    /// Périodes chômées cochées (placements choisis), R-277.
    pub periods: Vec<(Day, Day)>,
    /// Recréer les liens relus dans le dessin (`None` : réglage par
    /// défaut, [`DRAWN_LINKS_DEFAULT`] ; R-278).
    pub drawn_links: Option<bool>,
}

/// Case « Recréer les liens dessinés » cochée d'office : la mesure du lot
/// P1c (ADR-0034) donne 99 % de liens justes sur les PDF publics à colonne
/// Prédécesseurs, 37 sur 37 au contrôle à l'œil des plannings du
/// propriétaire, et aucune date identique perdue.
pub const DRAWN_LINKS_DEFAULT: bool = true;

/// Ligne du tableau d'aperçu (dates formatées à l'affichage, pour les
/// seules lignes visibles).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PreviewRow {
    /// N° lu.
    pub id: u32,
    /// Niveau (1 = haut).
    pub level: u16,
    /// Récapitulative.
    pub summary: bool,
    /// Nom.
    pub name: String,
    /// Durée telle que lue.
    pub duration: String,
    /// Début lu.
    pub start_read: Option<Moment>,
    /// Fin lue.
    pub finish_read: Option<Moment>,
    /// Fin calculée par le moteur (jour ; une fin à 00:00 compte pour le
    /// jour d'avant, R-272).
    pub finish_computed: Option<Day>,
    /// Écart du début, en jours calendaires (calculé − lu).
    pub start_gap: i64,
    /// Écart de la fin.
    pub finish_gap: i64,
}

impl PreviewRow {
    /// Une date calculée diffère de la date lue.
    #[must_use]
    pub const fn has_gap(&self) -> bool {
        self.start_gap != 0 || self.finish_gap != 0
    }
}

/// Aperçu calculé.
#[derive(Debug)]
pub struct Preview {
    /// Périodes chômées déduites (sur le calendrier avec les fériés
    /// cochés, sans les périodes).
    pub deduction: Shared<Deduction>,
    /// Lignes du tableau.
    pub rows: Vec<PreviewRow>,
    /// Dates égales au jour près (débuts et fins).
    pub dates_equal: u32,
    /// Dates comparées.
    pub dates_compared: u32,
    /// Fériés posés.
    pub holidays_on: bool,
    /// Années des fériés posés (première, dernière).
    pub holiday_years: Option<(i32, i32)>,
    /// Avertissements du moteur (lecture puis conversion).
    pub warnings: Vec<Warning>,
    /// Liens dessinés posés dans le projet (nombre), si la case est
    /// cochée et que le PDF n'a pas de colonne Prédécesseurs.
    pub drawn_placed: Option<u32>,
    /// Projet construit (ouvert par « Importer »).
    pub project: Shared<Project>,
}

/// Échec du calcul.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreviewError {
    /// PDF refusé par le moteur.
    Refused(ImportError),
    /// Planning lu, projet refusé par le moteur (code ou détail).
    Convert(String),
}

/// Résultat d'un calcul pour des réglages.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Computed {
    /// Réglages demandés.
    pub settings: PdfSettings,
    /// Planning lu (réutilisé tant que l'ordre des dates et « sans
    /// hiérarchie » ne changent pas).
    pub plan: Shared<Result<ImportedPlan, ImportError>>,
    /// Aperçu, ou échec.
    pub result: Result<Shared<Preview>, PreviewError>,
}

impl Computed {
    /// Aperçu calculé.
    #[must_use]
    pub fn preview(&self) -> Option<&Preview> {
        self.result.as_ref().ok().map(|p| p.0.as_ref())
    }
}

/// Entrée d'un calcul (envoyée au fil de calcul).
#[derive(Clone)]
pub struct ComputeInput {
    /// Octets du PDF.
    pub bytes: Arc<[u8]>,
    /// Réglages.
    pub settings: PdfSettings,
    /// Planning déjà lu pour le même ordre des dates et la même hiérarchie.
    pub cached_plan: Option<Shared<Result<ImportedPlan, ImportError>>>,
    /// « Maintenant » (date courante du projet).
    pub now: Moment,
    /// Langue des noms d'exceptions et de la session.
    pub lang: TextLanguage,
    /// Nom du projet.
    pub name: String,
    /// Nom des exceptions des périodes déduites.
    pub period_name: String,
}

/// Lit le PDF (ou reprend la lecture en cache), pose les jours chômés,
/// construit le projet, déduit les périodes et prépare l'aperçu. Pur et
/// déterministe ; aucun accès au système de fichiers.
#[must_use]
pub fn compute(input: &ComputeInput) -> Computed {
    let settings = input.settings.clone();
    let plan = input.cached_plan.clone().unwrap_or_else(|| {
        let options = ImportOptions {
            limits: import_limits(),
            date_order: settings.date_order,
            allow_flat_hierarchy: settings.flatten,
        };
        Shared::new(import(&input.bytes, &options))
    });
    let result = match plan.0.as_ref() {
        Err(error) => Err(PreviewError::Refused(*error)),
        Ok(p) => preview(p, input).map(Shared::new),
    };
    Computed {
        settings,
        plan,
        result,
    }
}

/// Aperçu d'un planning lu (étapes R-274 à R-277 puis R-272).
fn preview(p: &ImportedPlan, input: &ComputeInput) -> Result<Preview, PreviewError> {
    let settings = &input.settings;
    let holidays_on = settings
        .holidays
        .unwrap_or_else(|| calendar::suggests_french_holidays(p));
    let holidays = if holidays_on {
        calendar::french_holidays(p, input.lang)
    } else {
        Vec::new()
    };
    let holiday_years = {
        use chrono::Datelike as _;
        let years = holidays.iter().map(|h| h.from.to_naive().year());
        years.clone().min().zip(years.max())
    };
    let options = |non_working| ConvertOptions {
        name: input.name.clone(),
        now: input.now,
        lang: input.lang,
        pin_read_starts: settings.pin_starts,
        non_working,
        drawn_links: settings.drawn_links.unwrap_or(DRAWN_LINKS_DEFAULT),
    };
    let refused = |e: plannercraft_pdfimport::ConvertError| PreviewError::Convert(e.to_string());
    let base = convert(p, &options(holidays.clone())).map_err(refused)?;
    let deduction = deduce(p, base.session.project(), &DeduceLimits::default());
    let converted = if settings.periods.is_empty() {
        base
    } else {
        let mut all = holidays;
        all.extend(calendar::periods(&settings.periods, &input.period_name));
        convert(p, &options(all)).map_err(refused)?
    };
    let gaps: BTreeMap<(u32, &str), i64> = converted.gaps.iter().map(|g| ((g.id, g.field), g.days)).collect();
    let schedule = converted.session.schedule();
    let rows = p
        .tasks
        .iter()
        .zip(&converted.uids)
        .map(|(t, uid)| {
            let finish_computed = schedule.scheduled(*uid).map(|s| {
                if s.finish.time_of_day().minutes() == 0 {
                    s.finish.day().checked_add_days(-1).unwrap_or(s.finish.day())
                } else {
                    s.finish.day()
                }
            });
            PreviewRow {
                id: t.id,
                level: t.level,
                summary: t.summary,
                name: t.name.clone(),
                duration: t.duration_text.clone(),
                start_read: t.start,
                finish_read: t.finish,
                finish_computed,
                start_gap: gaps.get(&(t.id, "start")).copied().unwrap_or(0),
                finish_gap: gaps.get(&(t.id, "finish")).copied().unwrap_or(0),
            }
        })
        .collect();
    let mut warnings = p.warnings.clone();
    warnings.extend(converted.warnings.iter().cloned());
    let drawn_placed = converted.drawn.as_ref().map(|d| d.placed);
    Ok(Preview {
        deduction: Shared::new(deduction),
        rows,
        dates_equal: converted.dates_equal,
        dates_compared: converted.dates_compared,
        holidays_on,
        holiday_years,
        warnings,
        drawn_placed,
        project: Shared::new(converted.session.project().clone()),
    })
}

/// Calcul en cours.
enum Job {
    /// Fil dédié (bureau).
    #[cfg(not(target_arch = "wasm32"))]
    Thread(std::sync::mpsc::Receiver<Computed>),
    /// Calcul à faire à l'image suivante (web, ou fil indisponible) ;
    /// vrai une fois l'état « en cours » affiché.
    Deferred(Box<ComputeInput>, bool),
}

/// Emplacement du calcul en cours (comparé par identité).
#[derive(Clone, Default)]
struct JobSlot(Option<Arc<Mutex<Job>>>);

impl PartialEq for JobSlot {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (None, None) => true,
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Eq for JobSlot {}

impl std::fmt::Debug for JobSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(if self.0.is_some() {
            "JobSlot(en cours)"
        } else {
            "JobSlot(libre)"
        })
    }
}

/// Octets du fichier choisi ; leur forme de débogage ne donne que la
/// taille.
#[derive(Clone, PartialEq, Eq)]
pub struct PdfBytes(pub Arc<[u8]>);

impl std::fmt::Debug for PdfBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PdfBytes({} octets)", self.0.len())
    }
}

/// Case d'une période proposée.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PeriodChoice {
    /// Cochée.
    pub checked: bool,
    /// Placement choisi (indice dans `ProposedPeriod::options`).
    pub option: usize,
}

/// Brouillon de la fenêtre.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PdfImportState {
    /// Nom du fichier choisi.
    pub name: String,
    /// Octets du fichier.
    pub bytes: PdfBytes,
    /// Réglages voulus.
    pub settings: PdfSettings,
    /// Dernier calcul fini (affiché).
    pub shown: Option<Computed>,
    /// Cases des périodes proposées par `shown`.
    pub choices: Vec<PeriodChoice>,
    /// Filtre « écarts seulement ».
    pub only_gaps: bool,
    /// Réglages du calcul en cours.
    running: Option<PdfSettings>,
    job: JobSlot,
    /// « Maintenant » à l'ouverture.
    now: Moment,
    /// Langue de la session et des noms d'exceptions.
    lang: TextLanguage,
    /// Nom des périodes déduites dans le calendrier.
    period_name: String,
    /// Calcul à l'image suivante même sur le bureau (chemin du web,
    /// [`PdfImportState::use_deferred_compute`]).
    deferred: bool,
}

/// Clé de lecture : réglages qui changent le planning lu.
fn parse_key(s: &PdfSettings) -> (Option<NumericOrder>, bool) {
    (s.date_order, s.flatten)
}

impl PdfImportState {
    /// Fenêtre sur le fichier `name` ; premier calcul demandé à la première
    /// image.
    #[must_use]
    pub fn open(
        name: String,
        bytes: Arc<[u8]>,
        now: Moment,
        lang: TextLanguage,
        period_name: String,
    ) -> Self {
        Self {
            name,
            bytes: PdfBytes(bytes),
            settings: PdfSettings::default(),
            shown: None,
            choices: Vec::new(),
            only_gaps: false,
            running: None,
            job: JobSlot::default(),
            now,
            lang,
            period_name,
            deferred: cfg!(target_arch = "wasm32"),
        }
    }

    /// Calcule à l'image qui suit l'affichage de « en cours » au lieu d'un
    /// fil : chemin du web, exercé aussi par les tests du bureau.
    pub fn use_deferred_compute(&mut self) {
        self.deferred = true;
    }

    /// Un calcul est en cours ou à lancer : l'aperçu affiché n'est pas
    /// celui des réglages voulus.
    #[must_use]
    pub fn busy(&self) -> bool {
        self.running.is_some() || self.shown.as_ref().is_none_or(|c| c.settings != self.settings)
    }

    /// Aperçu affiché.
    #[must_use]
    pub fn preview(&self) -> Option<&Preview> {
        self.shown.as_ref().and_then(Computed::preview)
    }

    /// Entrée du calcul des réglages voulus.
    fn input(&self) -> ComputeInput {
        let cached_plan = self
            .shown
            .as_ref()
            .filter(|c| parse_key(&c.settings) == parse_key(&self.settings))
            .map(|c| c.plan.clone());
        ComputeInput {
            bytes: Arc::clone(&self.bytes.0),
            settings: self.settings.clone(),
            cached_plan,
            now: self.now,
            lang: self.lang,
            name: document_name(&self.name),
            period_name: self.period_name.clone(),
        }
    }

    /// Relève le calcul fini et lance celui des réglages voulus ; un seul
    /// calcul à la fois (un réglage changé pendant un calcul attend sa
    /// fin).
    pub fn refresh(&mut self, ctx: &egui::Context) {
        if let Some(done) = self.poll(ctx) {
            self.running = None;
            self.accept(done);
        }
        if self.running.is_none() && self.shown.as_ref().is_none_or(|c| c.settings != self.settings) {
            self.start(ctx);
        }
    }

    /// Calcul fini, ou `None`.
    fn poll(&mut self, ctx: &egui::Context) -> Option<Computed> {
        let slot = self.job.0.clone()?;
        let mut job = slot.lock().ok()?;
        let done = match &mut *job {
            #[cfg(not(target_arch = "wasm32"))]
            Job::Thread(rx) => match rx.try_recv() {
                Ok(done) => done,
                Err(std::sync::mpsc::TryRecvError::Empty) => return None,
                // Fil arrêté sans réponse (panique) : refus du lecteur.
                Err(std::sync::mpsc::TryRecvError::Disconnected) => Computed {
                    settings: self.running.clone().unwrap_or_default(),
                    plan: Shared::new(Err(ImportError::ReaderFailure)),
                    result: Err(PreviewError::Refused(ImportError::ReaderFailure)),
                },
            },
            Job::Deferred(input, drawn) => {
                if !*drawn {
                    // L'état « en cours » est d'abord affiché, le calcul
                    // (bloquant) vient à l'image suivante.
                    *drawn = true;
                    ctx.request_repaint();
                    return None;
                }
                compute(input)
            }
        };
        drop(job);
        self.job = JobSlot::default();
        Some(done)
    }

    /// Lance le calcul des réglages voulus.
    fn start(&mut self, ctx: &egui::Context) {
        let input = self.input();
        self.running = Some(self.settings.clone());
        #[cfg(not(target_arch = "wasm32"))]
        if !self.deferred {
            let (tx, rx) = std::sync::mpsc::channel();
            let repaint = ctx.clone();
            let thread_input = input.clone();
            let spawned = std::thread::Builder::new()
                .name("plannercraft-pdf-import".to_owned())
                .spawn(move || {
                    let done = compute(&thread_input);
                    if tx.send(done).is_err() {
                        log::info!("import PDF : fenêtre refermée avant la fin du calcul");
                    }
                    repaint.request_repaint();
                });
            if spawned.is_ok() {
                self.job = JobSlot(Some(Arc::new(Mutex::new(Job::Thread(rx)))));
                return;
            }
            log::warn!("import PDF : fil de calcul indisponible, calcul à l'image suivante");
        }
        self.job = JobSlot(Some(Arc::new(Mutex::new(Job::Deferred(Box::new(input), false)))));
        ctx.request_repaint();
    }

    /// Installe un calcul fini ; périodes remises à zéro quand le planning
    /// lu ou le calendrier de la déduction ont changé.
    fn accept(&mut self, done: Computed) {
        let key = |c: &Computed| {
            (
                parse_key(&c.settings),
                c.preview().map(|p| p.holidays_on),
                c.preview().map(|p| p.deduction.0.periods.len()),
            )
        };
        let same_deduction = self.shown.as_ref().is_some_and(|old| key(old) == key(&done));
        if !same_deduction {
            self.choices = done
                .preview()
                .map(|p| {
                    p.deduction
                        .0
                        .periods
                        .iter()
                        .map(|period| PeriodChoice {
                            checked: false,
                            option: period.default_option,
                        })
                        .collect()
                })
                .unwrap_or_default();
            // Les périodes cochées valaient pour l'ancienne déduction.
            self.settings.periods.clear();
            // L'ordre des dates suit le document tant que l'utilisateur ne
            // l'impose pas.
        }
        // Fériés : la case reflète le choix appliqué.
        if self.settings.holidays.is_none()
            && let Some(p) = done.preview()
        {
            self.settings.holidays = Some(p.holidays_on);
            if let Some(shown) = self.shown.as_mut() {
                shown.settings.holidays = Some(p.holidays_on);
            }
        }
        let mut done = done;
        if done.settings.holidays.is_none() {
            done.settings.holidays = done.preview().map(|p| p.holidays_on);
        }
        self.shown = Some(done);
    }

    /// Réglage des périodes depuis les cases.
    fn sync_periods(&mut self) {
        let Some(p) = self.preview() else { return };
        let periods: Vec<(Day, Day)> = p
            .deduction
            .0
            .periods
            .iter()
            .zip(&self.choices)
            .filter(|(_, c)| c.checked)
            .filter_map(|(period, c)| period.placement(c.option))
            .collect();
        self.settings.periods = periods;
    }
}

/// Réponse de la fenêtre pour une image.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfOutcome {
    /// Rien.
    Open,
    /// « Importer ».
    Import,
    /// « Annuler » ou Échap.
    Cancel,
}

/// Identifiant de la fenêtre (couche modale d'egui).
#[must_use]
pub fn modal_id() -> egui::Id {
    egui::Id::unique("plannercraft.pdf-import")
}

/// Nom du document importé : nom du fichier sans son extension.
#[must_use]
pub fn document_name(file: &str) -> String {
    super::csv_import::document_name(file)
}

/// Texte traduit d'un avertissement du moteur (code stable).
#[must_use]
pub const fn warning_text_id(code: WarningCode) -> TextId {
    match code {
        WarningCode::IdGap => TextId::PdfWarnIdGap,
        WarningCode::IdConflict => TextId::PdfWarnIdConflict,
        WarningCode::BlankRow => TextId::PdfWarnBlankRow,
        WarningCode::NameTruncated => TextId::PdfWarnNameTruncated,
        WarningCode::DurationUnreadable => TextId::PdfWarnDurationUnreadable,
        WarningCode::DurationMissing => TextId::PdfWarnDurationMissing,
        WarningCode::DateUnreadable => TextId::PdfWarnDateUnreadable,
        WarningCode::DateOrderAssumed => TextId::PdfWarnDateOrderAssumed,
        WarningCode::DateOrderConflict => TextId::PdfWarnDateOrderConflict,
        WarningCode::FinishBeforeStart => TextId::PdfWarnFinishBeforeStart,
        WarningCode::PredecessorUnreadable => TextId::PdfWarnPredecessorUnreadable,
        WarningCode::PredecessorExternal => TextId::PdfWarnPredecessorExternal,
        WarningCode::PredecessorUnknown => TextId::PdfWarnPredecessorUnknown,
        WarningCode::PredecessorSelf => TextId::PdfWarnPredecessorSelf,
        WarningCode::LevelJump => TextId::PdfWarnLevelJump,
        WarningCode::StrayLines => TextId::PdfWarnStrayLines,
        WarningCode::Truncated => TextId::PdfWarnTruncated,
        WarningCode::PagesSkipped => TextId::PdfWarnPagesSkipped,
        WarningCode::LinkRefused => TextId::PdfWarnLinkRefused,
        WarningCode::ConstraintRefused => TextId::PdfWarnConstraintRefused,
        WarningCode::Flattened => TextId::PdfWarnFlattened,
    }
}

/// Que faire après un refus, quand il y a quelque chose à faire.
#[must_use]
pub const fn refusal_hint(error: ImportError) -> Option<TextId> {
    match error {
        ImportError::Encrypted => Some(TextId::PdfHintEncrypted),
        ImportError::Scanned | ImportError::NoText => Some(TextId::PdfHintScanned),
        ImportError::NoTable => Some(TextId::PdfHintNoTable),
        ImportError::NoDates => Some(TextId::PdfHintNoDates),
        ImportError::FlatHierarchy | ImportError::InconsistentHierarchy => Some(TextId::PdfHintHierarchy),
        ImportError::Invalid | ImportError::ReaderFailure => Some(TextId::PdfHintDamaged),
        ImportError::TooLarge => Some(TextId::PdfHintTooLarge),
        ImportError::NoTasks | ImportError::TooManyTasks(_) => None,
    }
}

/// Ligne d'un avertissement : N° et valeur en cause s'il y en a.
fn warning_line(texts: &Texts, w: &Warning) -> String {
    let mut line = texts.get(warning_text_id(w.code));
    if let Some(value) = &w.value {
        line = texts.get_args(
            TextId::PdfWarningValue,
            &[
                ("detail", Arg::Str(line.into())),
                ("value", Arg::Str(value.as_str().into())),
            ],
        );
    }
    if let Some(id) = w.id {
        line = texts.get_args(
            TextId::PdfWarningTask,
            &[
                ("id", Arg::Str(id.to_string().into())),
                ("detail", Arg::Str(line.into())),
            ],
        );
    }
    line
}

/// Nombre en `Arg::Count`.
fn count(n: usize) -> Arg<'static> {
    Arg::Count(i64::try_from(n).unwrap_or(i64::MAX))
}

/// Date courte d'un jour.
fn day_text(texts: &Texts, day: Day) -> String {
    Moment::at(day, TimeOfDay::MIDNIGHT)
        .map_or_else(|_| day.to_string(), |m| format_date(m, texts.format_labels()))
}

/// Écart signé en jours (« +2 », « −1 »).
fn signed(n: i64) -> String {
    if n > 0 { format!("+{n}") } else { n.to_string() }
}

/// Résumé : tâches, niveaux, liens, colonnes reconnues, avertissements.
fn summary(ui: &mut egui::Ui, texts: &Texts, plan: &ImportedPlan, p: &Preview) {
    let parts = [
        texts.get_args(TextId::PdfSummaryTasks, &[("count", count(plan.tasks.len()))]),
        texts.get_args(
            TextId::PdfSummaryLevels,
            &[("count", Arg::Count(i64::from(plan.stats.levels)))],
        ),
        texts.get_args(
            TextId::PdfSummaryLinks,
            &[("count", Arg::Count(i64::from(plan.stats.links_read)))],
        ),
    ];
    ui.label(parts.join(", "));
    if let Some((read, dropped)) = drawn_counts(plan) {
        ui.label(texts.get_args(
            TextId::PdfSummaryDrawn,
            &[("count", count(read)), ("dropped", count(dropped))],
        ));
    }
    let columns: Vec<&str> = plan
        .source
        .columns
        .iter()
        .filter(|c| c.kind != ColumnKind::Other)
        .map(|c| c.header.as_str())
        .collect();
    if !columns.is_empty() {
        ui.label(texts.get_args(
            TextId::PdfColumns,
            &[("names", Arg::Str(columns.join(", ").into()))],
        ));
    }
    if !p.warnings.is_empty() {
        ui.colored_label(
            ui.visuals().warn_fg_color,
            texts.get_args(TextId::PdfWarnings, &[("count", count(p.warnings.len()))]),
        );
        for w in p.warnings.iter().take(MAX_WARNINGS_SHOWN) {
            ui.add(egui::Label::new(warning_line(texts, w)).truncate());
        }
        let more = p.warnings.len().saturating_sub(MAX_WARNINGS_SHOWN);
        if more > 0 {
            ui.label(texts.get_args(TextId::PdfWarnMore, &[("count", count(more))]));
        }
    }
}

/// Liens relus dans le dessin (H-UI-213) : liens sûrs, et écartés (pointes
/// dont le tracé n'a pas donné de lien, et liens douteux) ; une forme en
/// pointe sans trait (bout d'une barre récapitulative) et un lien relu deux
/// fois ne sont pas comptés ; `None` si le dessin n'a pas de pointe.
#[must_use]
pub fn drawn_counts(plan: &ImportedPlan) -> Option<(usize, usize)> {
    let d = &plan.drawn;
    if d.arrows == 0 && d.links.is_empty() {
        return None;
    }
    let doubtful = d.links.iter().filter(|l| l.doubtful()).count();
    let dropped = d
        .dropped
        .iter()
        .filter(|(why, _)| !matches!(why, LinkDrop::Duplicate | LinkDrop::NoLine))
        .map(|(_, n)| usize::try_from(*n).unwrap_or(usize::MAX))
        .fold(doubtful, usize::saturating_add);
    Some((d.links.len().saturating_sub(doubtful), dropped))
}

/// Réglages montrés (H-UI-207) : seulement ceux qui servent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "une visibilité indépendante par réglage de la fenêtre"
)]
pub struct VisibleSettings {
    /// Ordre jour/mois : ambigu dans le document (`date_order_assumed`,
    /// `date_order_conflict`), ou imposé par l'utilisateur.
    pub date_order: bool,
    /// « Importer sans hiérarchie » : PDF refusé pour hiérarchie plate ou
    /// incohérente, ou case déjà cochée.
    pub flatten: bool,
    /// Contrainte des tâches liées : le PDF a des liens.
    pub pin_starts: bool,
    /// « Recréer les liens dessinés » (H-UI-214) : le dessin donne au
    /// moins un lien sûr et le PDF n'a pas de colonne Prédécesseurs (sinon
    /// la colonne fait foi).
    pub drawn_links: bool,
}

/// Réglages à montrer pour le planning lu `plan` (ou son refus).
#[must_use]
pub fn visible_settings(
    plan: Option<&Result<ImportedPlan, ImportError>>,
    settings: &PdfSettings,
) -> VisibleSettings {
    let read = plan.and_then(|p| p.as_ref().ok());
    let refused_hierarchy = matches!(
        plan,
        Some(Err(
            ImportError::FlatHierarchy | ImportError::InconsistentHierarchy
        ))
    );
    VisibleSettings {
        date_order: settings.date_order.is_some()
            || read.is_some_and(|p| {
                p.warnings.iter().any(|w| {
                    matches!(
                        w.code,
                        WarningCode::DateOrderAssumed | WarningCode::DateOrderConflict
                    )
                })
            }),
        flatten: refused_hierarchy || settings.flatten,
        pin_starts: read.is_some_and(|p| p.stats.links_read > 0),
        drawn_links: read.is_some_and(|p| {
            !plannercraft_pdfimport::convert::has_predecessor_column(p)
                && p.drawn.links.iter().any(|l| !l.doubtful())
        }),
    }
}

/// Réglages : ordre des dates, sans hiérarchie, tâches liées, liens
/// dessinés ([`visible_settings`]).
fn settings_block(ui: &mut egui::Ui, texts: &Texts, state: &mut PdfImportState) {
    let plan = state.shown.as_ref().map(|c| c.plan.clone());
    let visible = visible_settings(plan.as_ref().map(|p| p.0.as_ref()), &state.settings);
    if visible.date_order {
        let choices = [
            (None, TextId::CiAuto),
            (Some(NumericOrder::DayMonth), TextId::PdfOrderDayMonth),
            (Some(NumericOrder::MonthDay), TextId::PdfOrderMonthDay),
        ];
        egui::Grid::new("plannercraft.pdf-import.options").show(ui, |ui| {
            super::csv_import::combo(
                ui,
                texts,
                "plannercraft.pdf-import.date-order",
                TextId::PdfDateOrder,
                &mut state.settings.date_order,
                &choices,
            );
            ui.end_row();
        });
        ui.label(texts.get(TextId::PdfOrderAssumed));
    }
    if visible.flatten {
        ui.checkbox(&mut state.settings.flatten, texts.get(TextId::PdfFlatten));
    }
    if visible.pin_starts {
        ui.checkbox(&mut state.settings.pin_starts, texts.get(TextId::PdfPinStarts));
        ui.label(texts.get(TextId::PdfPinStartsHint));
    }
    if visible.drawn_links {
        let mut on = state.settings.drawn_links.unwrap_or(DRAWN_LINKS_DEFAULT);
        if ui.checkbox(&mut on, texts.get(TextId::PdfDrawnLinks)).changed() {
            state.settings.drawn_links = Some(on);
        }
    }
}

/// Bloc calendrier : fériés, périodes déduites, compteur.
fn calendar_block(ui: &mut egui::Ui, texts: &Texts, state: &mut PdfImportState) {
    let Some(p) = state.preview() else { return };
    let (holidays_on, years, deduction) = (p.holidays_on, p.holiday_years, p.deduction.clone());
    ui.strong(texts.get(TextId::PdfCalendar));
    ui.label(texts.get(TextId::PdfCalendarHint));
    let mut on = state.settings.holidays.unwrap_or(holidays_on);
    if ui.checkbox(&mut on, texts.get(TextId::PdfHolidays)).changed() {
        state.settings.holidays = Some(on);
    }
    if let Some((from, to)) = years {
        ui.label(texts.get_args(
            TextId::PdfHolidaysHint,
            &[
                ("from", Arg::Str(from.to_string().into())),
                ("to", Arg::Str(to.to_string().into())),
            ],
        ));
    } else {
        ui.label(texts.get(TextId::PdfHolidaysScope));
    }
    ui.add_space(4.0);
    ui.label(texts.get(TextId::PdfPeriods));
    let periods = &deduction.0.periods;
    if periods.is_empty() {
        ui.label(texts.get(TextId::PdfPeriodsNone));
    }
    let mut changed = false;
    for (k, period) in periods.iter().enumerate() {
        let Some(choice) = state.choices.get_mut(k) else {
            continue;
        };
        let Some((from, to)) = period.placement(choice.option) else {
            continue;
        };
        ui.horizontal(|ui| {
            let line = texts.get_args(
                TextId::PdfPeriodLine,
                &[
                    ("from", Arg::Str(day_text(texts, from).into())),
                    ("to", Arg::Str(day_text(texts, to).into())),
                    ("count", Arg::Count(i64::from(period.working_days))),
                    ("n", Arg::Count(i64::from(period.explained))),
                ],
            );
            changed |= ui.checkbox(&mut choice.checked, line).changed();
            if !period.exact() {
                let last = period.options.len().saturating_sub(1);
                let earlier = ui
                    .add_enabled(choice.option > 0, egui::Button::new("◀").small())
                    .on_hover_text(texts.get(TextId::PdfPeriodEarlier));
                let earlier_name = texts.get(TextId::PdfPeriodEarlier);
                earlier.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::Role::Button, choice.option > 0, earlier_name.clone())
                });
                if earlier.clicked() {
                    choice.option = choice.option.saturating_sub(1);
                    changed = true;
                }
                let later = ui
                    .add_enabled(choice.option < last, egui::Button::new("▶").small())
                    .on_hover_text(texts.get(TextId::PdfPeriodLater));
                let later_name = texts.get(TextId::PdfPeriodLater);
                later.widget_info(|| {
                    egui::WidgetInfo::labeled(egui::Role::Button, choice.option < last, later_name.clone())
                });
                if later.clicked() {
                    choice.option = choice.option.saturating_add(1).min(last);
                    changed = true;
                }
            }
        });
        if !period.exact()
            && let Some((wfrom, wto)) = period.window()
        {
            ui.indent(("plannercraft.pdf-import.place", k), |ui| {
                ui.label(texts.get_args(
                    TextId::PdfPeriodPlace,
                    &[
                        ("count", Arg::Count(i64::from(period.working_days))),
                        ("from", Arg::Str(day_text(texts, wfrom).into())),
                        ("to", Arg::Str(day_text(texts, wto).into())),
                    ],
                ));
            });
        }
    }
    if changed {
        state.sync_periods();
    }
    let left = deduction.0.impossible;
    if left > 0 {
        ui.label(texts.get_args(TextId::PdfUnexplained, &[("count", Arg::Count(i64::from(left)))]));
    }
}

/// Largeurs minimales des colonnes du tableau (points).
const COLUMN_WIDTHS: [f32; 7] = [40.0, 230.0, 64.0, 88.0, 88.0, 88.0, 120.0];

/// En-têtes des colonnes du tableau.
const HEADERS: [TextId; 7] = [
    TextId::PdfColId,
    TextId::PdfColName,
    TextId::PdfColDuration,
    TextId::PdfColStartRead,
    TextId::PdfColFinishRead,
    TextId::PdfColFinishComputed,
    TextId::PdfColGap,
];

/// Largeurs des colonnes : au moins celle de leur en-tête (un titre de
/// colonne n'est jamais coupé, même en pseudo-langue).
fn column_widths(ctx: &egui::Context, texts: &Texts) -> [f32; 7] {
    let font = egui::TextStyle::Body.resolve(&ctx.global_style());
    let mut widths = COLUMN_WIDTHS;
    for (width, id) in widths.iter_mut().zip(HEADERS) {
        let text = texts.get(id);
        let measured = ctx.fonts_mut(|f| {
            f.layout_no_wrap(text, font.clone(), egui::Color32::PLACEHOLDER)
                .size()
                .x
        });
        *width = width.max(measured + 6.0);
    }
    widths
}

/// Tableau des tâches lues, virtualisé, `height` points au plus.
fn table(ui: &mut egui::Ui, texts: &Texts, state: &PdfImportState, height: f32, widths: [f32; 7]) {
    let Some(p) = state.preview() else { return };
    let rows: Vec<&PreviewRow> = p
        .rows
        .iter()
        .filter(|r| !state.only_gaps || r.has_gap())
        .collect();
    // Cellule de largeur fixe, texte calé à gauche (retrait des noms
    // visible), coupé par « … » s'il déborde.
    let cell = |ui: &mut egui::Ui, width: f32, text: egui::RichText| {
        let size = egui::vec2(width, ui.spacing().interact_size.y);
        ui.allocate_ui_with_layout(size, egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.set_width(width);
            ui.add(egui::Label::new(text).truncate());
        });
    };
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            for (id, width) in HEADERS.iter().zip(widths) {
                cell(ui, width, egui::RichText::new(texts.get(*id)).strong());
            }
        });
        if rows.is_empty() {
            ui.label(texts.get(TextId::PdfNoRows));
            return;
        }
        let row_h = ui.spacing().interact_size.y;
        let warn = ui.visuals().warn_fg_color;
        egui::ScrollArea::vertical()
            .id_salt("plannercraft.pdf-import.rows")
            .max_height(height)
            .auto_shrink([false, true])
            .show_rows(ui, row_h, rows.len(), |ui, range| {
                for row in rows.get(range).unwrap_or_default() {
                    ui.horizontal(|ui| {
                        let date = |m: Option<Moment>| {
                            m.map(|m| format_date(m, texts.format_labels()))
                                .unwrap_or_default()
                        };
                        let indent = "  ".repeat(usize::from(row.level.saturating_sub(1)).min(12));
                        let mut name = egui::RichText::new(format!("{indent}{}", row.name));
                        if row.summary {
                            name = name.strong();
                        }
                        let gap = match (row.start_gap, row.finish_gap) {
                            (0, 0) => String::new(),
                            (0, f) => {
                                texts.get_args(TextId::PdfGapFinish, &[("value", Arg::Str(signed(f).into()))])
                            }
                            (s, 0) => {
                                texts.get_args(TextId::PdfGapStart, &[("value", Arg::Str(signed(s).into()))])
                            }
                            (s, f) => [
                                texts.get_args(TextId::PdfGapStart, &[("value", Arg::Str(signed(s).into()))]),
                                texts
                                    .get_args(TextId::PdfGapFinish, &[("value", Arg::Str(signed(f).into()))]),
                            ]
                            .join(", "),
                        };
                        let mark = |text: String, on: bool| {
                            let t = egui::RichText::new(text);
                            if on { t.color(warn) } else { t }
                        };
                        cell(ui, widths[0], egui::RichText::new(row.id.to_string()));
                        cell(ui, widths[1], name);
                        cell(ui, widths[2], egui::RichText::new(row.duration.as_str()));
                        cell(ui, widths[3], mark(date(row.start_read), row.start_gap != 0));
                        cell(ui, widths[4], egui::RichText::new(date(row.finish_read)));
                        cell(
                            ui,
                            widths[5],
                            mark(
                                row.finish_computed
                                    .map(|d| day_text(texts, d))
                                    .unwrap_or_default(),
                                row.finish_gap != 0,
                            ),
                        );
                        cell(ui, widths[6], mark(gap, true));
                    });
                }
            });
    });
}

/// Refus affiché en clair, et quoi faire quand c'est utile.
fn refusal(ui: &mut egui::Ui, texts: &Texts, error: &PreviewError) {
    match error {
        PreviewError::Refused(e) => {
            let reason = texts.get(error_text_id(e.code()));
            super::error_line(
                ui,
                Some(&texts.get_args(TextId::PdfRefused, &[("detail", Arg::Str(reason.into()))])),
            );
            if let Some(hint) = refusal_hint(*e) {
                ui.label(texts.get(hint));
            }
        }
        PreviewError::Convert(detail) => super::error_line(
            ui,
            Some(&texts.get_args(
                TextId::PdfConvertFailed,
                &[("detail", Arg::Str(detail.as_str().into()))],
            )),
        ),
    }
}

/// Dessine la fenêtre (une image) ; rend la réponse de l'utilisateur.
pub fn show(ctx: &egui::Context, texts: &Texts, state: &mut PdfImportState) -> PdfOutcome {
    state.refresh(ctx);
    let mut outcome = PdfOutcome::Open;
    let armed = super::enter_armed(ctx, None);
    let widths = column_widths(ctx, texts);
    let table_width: f32 = widths.iter().sum::<f32>() + 6.0 * ctx.global_style().spacing.item_spacing.x + 4.0;
    let response = egui::Modal::new(modal_id()).show(ctx, |ui| {
        ui.set_width(table_width.max(780.0));
        ui.heading(texts.get(TextId::PdfTitle));
        ui.label(texts.get_args(TextId::CiFile, &[("name", Arg::Str(state.name.as_str().into()))]));
        ui.label(texts.get(TextId::PdfLocal));
        if cfg!(target_arch = "wasm32") {
            ui.label(texts.get_args(
                TextId::PdfWebRisk,
                &[("size", count(WEB_MAX_MIB)), ("max", count(WEB_MAX_PAGES))],
            ));
        }
        ui.separator();
        if state.busy() {
            let id = if state.shown.is_none() {
                TextId::PdfReading
            } else {
                TextId::PdfComputing
            };
            ui.label(egui::RichText::new(texts.get(id)).italics());
        }
        // Hauteurs bornées par l'écran : résumé, réglages et calendrier
        // défilent ; le tableau a son propre défilement, en dessous (pas de
        // défilements imbriqués).
        let screen = ctx.content_rect().height();
        let table_height = (screen * 0.28).clamp(110.0, 260.0);
        let body_height = (screen - table_height - 240.0).max(120.0);
        let shown = state.shown.clone();
        egui::ScrollArea::vertical()
            .id_salt("plannercraft.pdf-import.body")
            .max_height(body_height)
            .show(ui, |ui| {
                if let Some(shown) = &shown {
                    match (&shown.result, shown.plan.0.as_ref()) {
                        (Ok(p), Ok(plan)) => summary(ui, texts, plan, &p.0),
                        (Err(error), _) => refusal(ui, texts, error),
                        (Ok(_), Err(_)) => {}
                    }
                    settings_block(ui, texts, state);
                    if shown.result.is_ok() {
                        ui.separator();
                        calendar_block(ui, texts, state);
                    }
                }
            });
        if shown.as_ref().is_some_and(|s| s.result.is_ok()) {
            ui.separator();
            // Compteur hors de la zone défilante : il reste visible quand
            // la liste des périodes dépasse (relecture du lot P1b).
            let same = state.preview().map(|p| (p.dates_equal, p.dates_compared));
            ui.horizontal(|ui| {
                ui.checkbox(&mut state.only_gaps, texts.get(TextId::PdfOnlyGaps));
                if let Some((equal, compared)) = same {
                    ui.separator();
                    ui.strong(texts.get_args(
                        TextId::PdfSame,
                        &[
                            ("n", Arg::Count(i64::from(equal))),
                            ("count", Arg::Count(i64::from(compared))),
                        ],
                    ));
                }
            });
            table(ui, texts, state, table_height, widths);
        }
        ui.separator();
        ui.horizontal(|ui| {
            let ready = !state.busy() && state.preview().is_some();
            if ui
                .add_enabled(ready, egui::Button::new(texts.get(TextId::CiImport)))
                .clicked()
            {
                outcome = PdfOutcome::Import;
            }
            if ui.button(texts.get(TextId::TaskInfoCancel)).clicked() {
                outcome = PdfOutcome::Cancel;
            }
        });
    });
    let escape = response.is_top_modal
        && !response.any_popup_open
        && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
    if escape {
        outcome = PdfOutcome::Cancel;
    }
    if outcome == PdfOutcome::Open
        && !state.busy()
        && state.preview().is_some()
        && super::enter_pressed(ctx, armed, response.is_top_modal, response.any_popup_open)
    {
        outcome = PdfOutcome::Import;
    }
    outcome
}

impl App {
    /// Fichier → Importer un PDF… : sélecteur de fichiers ; la réponse
    /// arrive par [`App::poll_picked`].
    pub(crate) fn start_pdf_import(&mut self, ctx: &egui::Context) {
        let request = PickRequest {
            title: self.texts.get(TextId::PdfPickTitle),
            filter: self.texts.get(TextId::PdfPickFilter),
            extensions: vec!["pdf".to_owned()],
        };
        self.pick_purpose = crate::app::PickPurpose::Pdf;
        let tx = self.pick_tx.clone();
        self.env.files.pick_bytes(ctx, request, tx);
    }

    /// Ouvre l'aperçu de l'import PDF sur `bytes` (une fenêtre déjà ouverte
    /// est remplacée ; une saisie de cellule est abandonnée).
    pub fn open_pdf_import(&mut self, name: String, bytes: Vec<u8>) {
        let now = (self.env.now)();
        let lang = self.text_language();
        let period_name = self.texts.get(TextId::PdfPeriodName);
        let state = PdfImportState::open(name, Arc::from(bytes), now, lang, period_name);
        self.view.edit.editing = None;
        self.view.drag = None;
        self.view.dialog = Some(super::DialogState {
            kind: DialogKind::PdfImport,
            body: super::DialogBody::PdfImport(Box::new(state)),
        });
    }

    /// Dessine l'aperçu de l'import PDF et applique sa réponse.
    pub(crate) fn show_pdf_import(&mut self, ctx: &egui::Context) {
        let Some(super::DialogState {
            body: super::DialogBody::PdfImport(state),
            ..
        }) = &mut self.view.dialog
        else {
            return;
        };
        match show(ctx, &self.texts, state) {
            PdfOutcome::Open => {}
            PdfOutcome::Cancel => self.close_dialog(),
            PdfOutcome::Import => self.import_pdf(ctx),
        }
    }

    /// « Importer » : le projet de l'aperçu affiché est ouvert comme un
    /// nouveau document (après confirmation si le courant est modifié).
    fn import_pdf(&mut self, ctx: &egui::Context) {
        let now = (self.env.now)();
        let Some(super::DialogState {
            body: super::DialogBody::PdfImport(state),
            ..
        }) = &self.view.dialog
        else {
            return;
        };
        let Some(project) = state.preview().map(|p| p.project.0.as_ref().clone()) else {
            return;
        };
        let origin = DocOrigin {
            name: document_name(&state.name),
            path: None,
            demo: false,
        };
        let loaded = crate::document::load_project(project, now);
        self.close_dialog();
        self.request_install_pdf(loaded, origin, ctx);
    }
}
