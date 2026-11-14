//! `plannercraft import-pdf <planning.pdf> -o out.xml [--json]
//! [--pin-starts] [--flatten] [--date-order jj/mm|mm/jj] [--holidays]
//! [--periods] [--no-drawn-links] [--dry-run]` : planning Gantt lu dans un
//! PDF (export MS Project), projet construit par les commandes du moteur,
//! dates calculées par le moteur et comparées aux dates lues (ADR-0032,
//! R-270 à R-273), puis enregistré en MSPDI (écriture atomique). Lot P1b :
//! jours fériés français et périodes chômées déduites (R-274 à R-277) sur
//! demande ; lot P1c : liens relus dans le dessin posés par défaut quand le
//! PDF n'a pas de colonne Prédécesseurs (R-278 à R-280, ADR-0034). Rapport sur la sortie standard (texte, ou JSON
//! avec `--json`). PDF refusé (numérisé, sans tableau, sans dates,
//! hiérarchie illisible…) : code de sortie 4 ([`exit::PDF_REFUSED`]).

use std::path::Path;
use std::process::ExitCode;

use clap::{Args, ValueEnum};
use plannercraft_automation::files::{read_path, write_atomic};
use plannercraft_pdfimport::{
    ConvertOptions, DeduceLimits, ImportOptions, NumericOrder, calendar, convert, deduce, import, report,
};
use serde_json::json;

use super::{Ctx, Run, err_line, finish, out, out_json, warn};
use crate::exit;

/// Ordre jour/mois imposé.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum DateOrderArg {
    /// JJ/MM/AA.
    #[value(name = "jj/mm", alias = "dd/mm")]
    DayMonth,
    /// MM/JJ/AA.
    #[value(name = "mm/jj", alias = "mm/dd")]
    MonthDay,
}

/// Arguments de `plannercraft import-pdf`.
#[derive(Debug, Args)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "drapeaux indépendants de la ligne de commande"
)]
pub struct ImportPdfArgs {
    /// Fichier PDF (planning Gantt exporté, pas un scan).
    pub input: String,
    /// Fichier MSPDI écrit (obligatoire sauf avec `--dry-run`).
    #[arg(
        short = 'o',
        long = "output",
        value_name = "FICHIER",
        required_unless_present = "dry_run"
    )]
    pub output: Option<String>,
    /// Rapport JSON sur la sortie standard.
    #[arg(long)]
    pub json: bool,
    /// « Début au plus tôt le » aussi sur les tâches liées que le moteur
    /// place avant leur début lu (R-271 étendue).
    #[arg(long)]
    pub pin_starts: bool,
    /// Importe tout au niveau 1 quand la hiérarchie n'est pas lisible
    /// (sinon le PDF est refusé).
    #[arg(long)]
    pub flatten: bool,
    /// Ordre jour/mois des dates numériques (défaut : décidé sur le
    /// document, R-273).
    #[arg(long, value_enum)]
    pub date_order: Option<DateOrderArg>,
    /// Pose les onze jours fériés légaux français dans le calendrier du
    /// projet (R-274, R-275).
    #[arg(long)]
    pub holidays: bool,
    /// Pose toutes les périodes chômées déduites des écarts, à leur
    /// placement par défaut (R-276, R-277), sur le calendrier choisi.
    #[arg(long)]
    pub periods: bool,
    /// Ne pose pas les liens relus dans le dessin du Gantt (posés par
    /// défaut quand le PDF n'a pas de colonne Prédécesseurs, R-278).
    #[arg(long)]
    pub no_drawn_links: bool,
    /// Lit et compare sans écrire de fichier.
    #[arg(long)]
    pub dry_run: bool,
}

/// Taille lue au plus (borne de l'import).
const MAX_PDF_BYTES: u64 = 64 * 1024 * 1024;

/// Exécute la sous-commande.
pub fn run(ctx: &Ctx, args: &ImportPdfArgs) -> ExitCode {
    finish(import_pdf(ctx, args))
}

fn import_pdf(ctx: &Ctx, args: &ImportPdfArgs) -> Run {
    let t = |fr, en| ctx.tr(fr, en);
    let path = Path::new(&args.input);
    let bytes = read_path(path, MAX_PDF_BYTES).map_err(|e| {
        warn(&format!("{} : {}", args.input, e.message(ctx.lang)));
        ExitCode::from(exit::FILE)
    })?;
    let options = ImportOptions {
        date_order: args.date_order.map(|o| match o {
            DateOrderArg::DayMonth => NumericOrder::DayMonth,
            DateOrderArg::MonthDay => NumericOrder::MonthDay,
        }),
        allow_flat_hierarchy: args.flatten,
        ..ImportOptions::default()
    };
    let plan = match import(&bytes, &options) {
        Ok(p) => p,
        Err(e) => {
            warn(&format!("{} : {}", e.code(), e.message(ctx.lang)));
            if args.json {
                out_json(&json!({ "error": { "code": e.code(), "message": e.message(ctx.lang) } }))?;
            }
            return Err(ExitCode::from(exit::PDF_REFUSED));
        }
    };
    let name = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let options = |non_working| ConvertOptions {
        name: name.clone(),
        now: ctx.now,
        lang: ctx.lang,
        pin_read_starts: args.pin_starts,
        non_working,
        drawn_links: !args.no_drawn_links,
    };
    let refused = |e: plannercraft_pdfimport::ConvertError| {
        warn(&e.to_string());
        ExitCode::from(exit::PDF_REFUSED)
    };
    let holidays = if args.holidays {
        calendar::french_holidays(&plan, ctx.lang)
    } else {
        Vec::new()
    };
    let mut converted = convert(&plan, &options(holidays.clone())).map_err(refused)?;
    if args.periods {
        // Périodes déduites sur le calendrier choisi (fériés compris), puis
        // posées à leur placement par défaut (comme cocher toutes les
        // périodes proposées dans la fenêtre d'import).
        let deduction = deduce(&plan, converted.session.project(), &DeduceLimits::default());
        if !deduction.periods.is_empty() {
            let mut all = holidays;
            all.extend(calendar::periods(
                &calendar::default_placements(&deduction.periods),
                t(
                    "Congés (déduits de l'import PDF)",
                    "Time off (deduced from the PDF import)",
                ),
            ));
            converted = convert(&plan, &options(all)).map_err(refused)?;
        }
    }
    let mut saved = None;
    if let (Some(output), false) = (&args.output, args.dry_run) {
        let written = converted
            .session
            .save_bytes(&plannercraft_mspdi::write::WriteOptions::default());
        write_atomic(Path::new(output), &written.xml).map_err(|e| {
            warn(&format!("{output} : {}", e.message(ctx.lang)));
            ExitCode::from(exit::FILE)
        })?;
        saved = Some(output.clone());
    }
    if args.json {
        let mut v = serde_json::to_value(report::view(&plan, Some(&converted))).map_err(|e| {
            warn(&e.to_string());
            ExitCode::from(exit::REFUSED)
        })?;
        if let (Some(obj), Some(p)) = (v.as_object_mut(), &saved) {
            obj.insert("path".to_owned(), json!(p));
        }
        return out_json(&v);
    }
    out(&report::text(&plan, Some(&converted), ctx.lang))?;
    if let Some(p) = saved {
        err_line(&format!("{} : {p}", t("Projet enregistré", "Project saved")));
    }
    Ok(())
}
