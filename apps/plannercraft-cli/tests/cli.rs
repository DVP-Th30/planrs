//! Tests du binaire `plannercraft` (lot L9) : lancé par `CARGO_BIN_EXE_plannercraft`,
//! environnement maîtrisé (`PLANNERCRAFT_NOW`, `LANG`), fichiers dans un dossier
//! temporaire propre à chaque test. Codes de sortie : C-MCP-08.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing,
    reason = "tests"
)]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU32, Ordering};

use plannercraft_automation::Workspace;
use plannercraft_automation::text::Language;
use plannercraft_core::Moment;
use serde_json::{Value, json};

const NOW: &str = "2026-10-19T08:00";

/// Dossier temporaire propre à un test (retiré à la fin du test).
struct TempDir(PathBuf);

impl TempDir {
    fn new(tag: &str) -> Self {
        static N: AtomicU32 = AtomicU32::new(0);
        let n = N.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("plannercraft-cli-{tag}-{}-{n}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn join(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn house_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../crates/plannercraft-ui-egui/assets/demo/house.xml")
}

/// Copie de `house.xml` dans `dir`.
fn house_copy(dir: &TempDir) -> PathBuf {
    let p = dir.join("maison.xml");
    std::fs::copy(house_path(), &p).unwrap();
    p
}

/// Commande `plannercraft` à environnement maîtrisé (français, sans
/// `PLANNERCRAFT_NOW`).
fn plannercraft() -> Command {
    let mut c = Command::new(env!("CARGO_BIN_EXE_plannercraft"));
    c.env_remove("PLANNERCRAFT_NOW").env("LANG", "fr_FR.UTF-8");
    c
}

fn run(args: &[&str]) -> Output {
    plannercraft().args(args).output().unwrap()
}

fn code(o: &Output) -> i32 {
    o.status.code().expect("code de sortie")
}

fn stdout_json(o: &Output) -> Value {
    serde_json::from_slice(&o.stdout)
        .unwrap_or_else(|e| panic!("JSON attendu ({e}) : {}", String::from_utf8_lossy(&o.stdout)))
}

fn s(p: &Path) -> &str {
    p.to_str().unwrap()
}

fn now() -> Moment {
    NOW.parse().unwrap()
}

/// Espace en mémoire (ce que sert le MCP) avec `path` ouvert.
fn workspace_with(path: &Path, lang: Language) -> Workspace {
    let mut ws = Workspace::new(now(), lang);
    ws.call("project.open", json!({ "path": s(path) })).unwrap();
    ws
}

const ADD_TASK: &str = r#"{"format_version":1,"lang":"fr","ops":[{"op":"task.add","name":"Peinture des volets","duration":{"text":"3j"}}]}"#;

#[test]
fn cli_summary_house_json_equals_mcp_view() {
    let house = house_path();
    let o = run(&["--now", NOW, "summary", s(&house), "--json"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let cli = stdout_json(&o);
    let mut ws = workspace_with(&house, Language::Fr);
    let view = ws.call("project.summary", json!({})).unwrap();
    assert_eq!(cli, view);
    assert_eq!(cli["tasks"], json!(49));
    // Sortie texte : mêmes dates, issues du moteur.
    let o = run(&["--now", NOW, "summary", s(&house)]);
    assert_eq!(code(&o), 0);
    let text = String::from_utf8(o.stdout).unwrap();
    assert!(text.contains("Construction d'une maison"), "{text}");
    assert!(text.contains(view["finish"].as_str().unwrap()), "{text}");
    // `tasks --json` : la vue `ListView` (toutes les pages réunies).
    let o = run(&["--now", NOW, "tasks", s(&house), "--json", "--critical"]);
    assert_eq!(code(&o), 0);
    let list = stdout_json(&o);
    let mcp = ws
        .call("task.list", json!({ "critical": true, "limit": 500 }))
        .unwrap();
    assert_eq!(list["tasks"], mcp["tasks"]);
    assert_eq!(list["total"], mcp["total"]);
    // `--limit` : une page, telle que le MCP la rend.
    let o = run(&["--now", NOW, "tasks", s(&house), "--json", "--limit", "5"]);
    assert_eq!(code(&o), 0);
    let page = ws.call("task.list", json!({ "limit": 5 })).unwrap();
    assert_eq!(stdout_json(&o), page);
    // Texte : une ligne par tâche, plus en-tête et pied.
    let o = run(&["--now", NOW, "tasks", s(&house)]);
    assert_eq!(code(&o), 0);
    let text = String::from_utf8(o.stdout).unwrap();
    assert_eq!(text.lines().count(), 49 + 2, "{text}");
}

#[test]
fn cli_apply_dry_run_leaves_file() {
    let dir = TempDir::new("dry");
    let plan = house_copy(&dir);
    let ops = dir.join("ops.json");
    std::fs::write(&ops, ADD_TASK).unwrap();
    let before = std::fs::read(&plan).unwrap();
    let out = dir.join("sortie.xml");
    let o = run(&[
        "--now",
        NOW,
        "apply",
        s(&plan),
        s(&ops),
        "--dry-run",
        "-o",
        s(&out),
        "--json",
    ]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let v = stdout_json(&o);
    assert_eq!(v["committed"], json!(false));
    assert_eq!(v["created_uids"].as_array().unwrap().len(), 1);
    assert_eq!(std::fs::read(&plan).unwrap(), before);
    assert!(!out.exists(), "--dry-run n'écrit rien, même avec -o");
}

#[test]
fn cli_apply_writes_atomic() {
    let dir = TempDir::new("write");
    let plan = house_copy(&dir);
    let ops = dir.join("ops.json");
    std::fs::write(&ops, ADD_TASK).unwrap();
    let original = std::fs::read(&plan).unwrap();

    // `-o` : le plan reste intact, la sortie porte la tâche.
    let out = dir.join("sortie.xml");
    let o = run(&["--now", NOW, "apply", s(&plan), s(&ops), "-o", s(&out), "--json"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let outcome = stdout_json(&o);
    assert_eq!(outcome["committed"], json!(true));
    assert_eq!(std::fs::read(&plan).unwrap(), original);
    let mut ws = workspace_with(&out, Language::Fr);
    let summary = ws.call("project.summary", json!({})).unwrap();
    assert_eq!(summary["tasks"], json!(50));

    // Sans `-o` : le plan est remplacé, rien d'autre ne reste dans le
    // dossier (pas de temporaire).
    let o = run(&["--now", NOW, "apply", s(&plan), s(&ops)]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let text = String::from_utf8(o.stdout).unwrap();
    assert!(text.contains("Lot appliqué"), "{text}");
    assert!(text.contains(s(&plan)), "{text}");
    assert_ne!(std::fs::read(&plan).unwrap(), original);
    let mut ws = workspace_with(&plan, Language::Fr);
    let list = ws.call("task.list", json!({ "limit": 500 })).unwrap();
    let names: Vec<&str> = list["tasks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["name"].as_str().unwrap())
        .collect();
    assert!(names.contains(&"Peinture des volets"), "{names:?}");
    let mut entries: Vec<String> = std::fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    entries.sort();
    assert_eq!(entries, ["maison.xml", "ops.json", "sortie.xml"]);
}

/// J7 (lot L5) : `plannercraft apply` sert les écritures de calendrier et
/// `baseline.clear` (R-247 à R-250) ; le MSPDI écrit les garde.
#[test]
fn cli_apply_calendars_j7() {
    let dir = TempDir::new("calendars");
    let plan = house_copy(&dir);
    let ops = dir.join("ops.json");
    std::fs::write(
        &ops,
        r#"{"format_version":1,"ops":[
            {"op":"calendar.add","name":"Équipe samedi","copy_of":1},
            {"op":"calendar.set_week","calendar":"$NEW","days":[{"day":"saturday","setting":{"ranges":[{"from":"08:00","to":"12:00"}]}}]},
            {"op":"calendar.set_exception","cal":"$NEW","from":"2026-11-11","to":"2026-11-11","name":"Armistice"},
            {"op":"baseline.clear","number":5}
        ]}"#
        .replace("\"$NEW\"", "999"),
    )
    .unwrap();
    // Calendrier 999 inconnu : refus, code de sortie 1.
    let o = run(&["--now", NOW, "apply", s(&plan), s(&ops), "--json"]);
    assert_eq!(code(&o), 1, "{}", String::from_utf8_lossy(&o.stderr));
    // UID du calendrier créé : le suivant du plus grand du fichier.
    let mut ws = workspace_with(&plan, Language::Fr);
    let next = ws
        .call("calendar.add", json!({ "name": "Sonde", "dry_run": true }))
        .unwrap()["side_effects"][0]["calendar_created"]
        .as_u64()
        .unwrap();
    let text = std::fs::read_to_string(&ops)
        .unwrap()
        .replace("999", &next.to_string());
    std::fs::write(&ops, text).unwrap();
    let out = dir.join("sortie.xml");
    let o = run(&["--now", NOW, "apply", s(&plan), s(&ops), "-o", s(&out), "--json"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let v = stdout_json(&o);
    assert_eq!(v["side_effects"][0], json!({ "calendar_created": next }));
    assert!(
        v["warnings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|w| w["code"] == json!("baseline.nothing_to_clear"))
    );
    let xml = std::fs::read_to_string(&out).unwrap();
    assert!(xml.contains("<Name>Équipe samedi</Name>"));
    assert!(xml.contains("<Name>Armistice</Name>"));
}

#[test]
fn cli_apply_refused_exit_1() {
    let dir = TempDir::new("refused");
    let plan = house_copy(&dir);
    let before = std::fs::read(&plan).unwrap();
    let ops = dir.join("ops.json");
    std::fs::write(
        &ops,
        r#"{"format_version":1,"ops":[{"op":"task.update","uid":99999,"name":"X"}]}"#,
    )
    .unwrap();
    let o = run(&["--now", NOW, "apply", s(&plan), s(&ops), "--json"]);
    assert_eq!(code(&o), 1);
    let v = stdout_json(&o);
    assert_eq!(v["error"]["code"], json!("task.not_found"));
    assert_eq!(v["error"]["op_index"], json!(0));
    assert!(String::from_utf8_lossy(&o.stderr).contains("task.not_found"));
    assert_eq!(std::fs::read(&plan).unwrap(), before);
    // Lot JSON mal formé : refus métier (`param.invalid`), même code.
    std::fs::write(&ops, "{ pas du JSON").unwrap();
    let o = run(&["--now", NOW, "apply", s(&plan), s(&ops)]);
    assert_eq!(code(&o), 1);
    assert!(String::from_utf8_lossy(&o.stderr).contains("param.invalid"));
    assert_eq!(std::fs::read(&plan).unwrap(), before);
}

#[test]
fn cli_missing_file_exit_3() {
    let dir = TempDir::new("missing");
    let absent = dir.join("absent.xml");
    for args in [
        vec!["summary", s(&absent)],
        vec!["tasks", s(&absent), "--json"],
        vec!["render", s(&absent), "-o", "x.svg"],
    ] {
        let o = plannercraft().arg("--now").arg(NOW).args(&args).output().unwrap();
        assert_eq!(code(&o), 3, "{args:?}");
        assert!(
            String::from_utf8_lossy(&o.stderr).contains("file.read"),
            "{args:?}"
        );
    }
    // MSPDI illisible.
    let bad = dir.join("mauvais.xml");
    std::fs::write(&bad, "<Project><Tasks>").unwrap();
    let o = run(&["--now", NOW, "summary", s(&bad)]);
    assert_eq!(code(&o), 3);
    assert!(String::from_utf8_lossy(&o.stderr).contains("file.invalid"));
    // Lot absent.
    let plan = house_copy(&dir);
    let o = run(&["--now", NOW, "apply", s(&plan), s(&dir.join("ops.json"))]);
    assert_eq!(code(&o), 3);
    // Écriture impossible (dossier inexistant).
    let o = run(&[
        "--now",
        NOW,
        "render",
        s(&plan),
        "-o",
        s(&dir.join("nul/part/x.svg")),
    ]);
    assert_eq!(code(&o), 3);
    assert!(String::from_utf8_lossy(&o.stderr).contains("file.write"));
}

#[test]
#[allow(clippy::many_single_char_names, reason = "tests : a.svg, b.svg, c.svg")]
fn cli_render_svg_bytes_stable() {
    let dir = TempDir::new("render");
    let house = house_path();
    let a = dir.join("a.svg");
    let b = dir.join("b.svg");
    for out in [&a, &b] {
        let o = run(&["--now", NOW, "render", s(&house), "-o", s(out), "--zoom", "week"]);
        assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    }
    let bytes = std::fs::read(&a).unwrap();
    assert_eq!(bytes, std::fs::read(&b).unwrap());
    assert!(
        bytes.starts_with(b"<svg"),
        "{}",
        String::from_utf8_lossy(&bytes[..64])
    );
    // Même rendu que `render.gantt` du MCP.
    let mut ws = workspace_with(&house, Language::Fr);
    let v = ws.call("render.gantt", json!({ "zoom": "week" })).unwrap();
    assert_eq!(v["svg"].as_str().unwrap().as_bytes(), bytes.as_slice());
    // Filtre et plage.
    let c = dir.join("c.svg");
    let o = run(&[
        "--now",
        NOW,
        "render",
        s(&house),
        "-o",
        s(&c),
        "--critical",
        "--from",
        "2026-11-02",
        "--to",
        "2027-01-29",
    ]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let v = ws
        .call(
            "render.gantt",
            json!({ "critical_only": true, "from": "2026-11-02", "to": "2027-01-29" }),
        )
        .unwrap();
    assert_eq!(v["svg"].as_str().unwrap().as_bytes(), std::fs::read(&c).unwrap());
    // Jour mal formé : usage.
    let o = run(&["render", s(&house), "-o", s(&c), "--from", "02/11/2026"]);
    assert_eq!(code(&o), 2);
}

/// Début du projet écrit par `import-csv` avec cet environnement.
fn imported_start(dir: &TempDir, env_now: Option<&str>, flag: Option<&str>) -> (i32, Option<String>) {
    let csv = dir.join("taches.csv");
    std::fs::write(&csv, "Nom;Durée\nFondations;5j\n").unwrap();
    let out = dir.join("taches.xml");
    let _ = std::fs::remove_file(&out);
    let mut c = plannercraft();
    if let Some(e) = env_now {
        c.env("PLANNERCRAFT_NOW", e);
    }
    if let Some(f) = flag {
        c.args(["--now", f]);
    }
    let o = c.args(["import-csv", s(&csv), "-o", s(&out)]).output().unwrap();
    if code(&o) != 0 {
        return (code(&o), None);
    }
    let mut ws = workspace_with(&out, Language::Fr);
    let v = ws.call("project.summary", json!({})).unwrap();
    (0, Some(v["start"].as_str().unwrap().to_owned()))
}

#[test]
fn cli_now_env_and_flag_precedence() {
    let dir = TempDir::new("now");
    // Variable seule.
    let got = imported_start(&dir, Some("2026-11-02T08:00"), None);
    assert_eq!(got, (0, Some("2026-11-02T08:00".to_owned())));
    // L'option l'emporte sur la variable.
    let got = imported_start(&dir, Some("2026-11-02T08:00"), Some("2026-12-01T08:00"));
    assert_eq!(got, (0, Some("2026-12-01T08:00".to_owned())));
    // Option seule.
    let got = imported_start(&dir, None, Some("2027-02-01T08:00"));
    assert_eq!(got, (0, Some("2027-02-01T08:00".to_owned())));
    // Horloge locale à défaut : une date est écrite.
    let (c, start) = imported_start(&dir, None, None);
    assert_eq!(c, 0);
    assert!(start.is_some());
    // Valeurs invalides : usage (2).
    assert_eq!(imported_start(&dir, Some("demain"), None).0, 2);
    assert_eq!(imported_start(&dir, None, Some("2026-02-30T08:00")).0, 2);
    // Une variable invalide est ignorée si l'option est donnée.
    let got = imported_start(&dir, Some("demain"), Some("2026-11-02T08:00"));
    assert_eq!(got.0, 0);
}

/// UID de la tâche d'ID `id` du plan ouvert dans `ws`.
fn task_uid_of_id(ws: &Workspace, id: u32) -> u32 {
    let p = ws.session().unwrap().project();
    p.task_by_id(plannercraft_core::TaskId::new(id))
        .unwrap()
        .uid()
        .get()
}

#[test]
fn cli_explain_json() {
    // `--json` : exactement la vue du MCP, ID converti en UID.
    let house = house_path();
    let o = run(&["--now", NOW, "explain", s(&house), "--task", "3", "--json"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let cli = stdout_json(&o);
    let mut ws = workspace_with(&house, Language::Fr);
    let uid = task_uid_of_id(&ws, 3);
    let view = ws.call("analysis.explain_date", json!({ "task": uid })).unwrap();
    assert_eq!(cli, view);
    assert_eq!(cli["task"]["id"], json!(3));
    assert_eq!(cli["side"], json!("start"));
    // `--finish`, `--depth`.
    let o = run(&[
        "--now",
        NOW,
        "explain",
        s(&house),
        "--task",
        "3",
        "--finish",
        "--depth",
        "1",
        "--json",
    ]);
    assert_eq!(code(&o), 0);
    let view = ws
        .call(
            "analysis.explain_date",
            json!({ "task": uid, "side": "finish", "depth": 1 }),
        )
        .unwrap();
    assert_eq!(stdout_json(&o), view);
    assert!(stdout_json(&o)["chain"].as_array().unwrap().len() <= 1);
    // ID inconnu : refus (1), erreur structurée ; profondeur hors bornes :
    // usage (2).
    let o = run(&["--now", NOW, "explain", s(&house), "--task", "999", "--json"]);
    assert_eq!(code(&o), 1);
    assert_eq!(stdout_json(&o)["error"]["code"], json!("task.not_found"));
    assert_eq!(
        code(&run(&["explain", s(&house), "--task", "3", "--depth", "0"])),
        2
    );
    assert_eq!(code(&run(&["explain", s(&house)])), 2);
}

#[test]
fn cli_explain_text() {
    // Texte : le résumé du moteur (dates de la vue), puis le côté au plus
    // tard ; visible dans l'aide.
    let house = house_path();
    let o = run(&["--now", NOW, "explain", s(&house), "--task", "3"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let text = String::from_utf8(o.stdout).unwrap();
    let mut ws = workspace_with(&house, Language::Fr);
    let uid = task_uid_of_id(&ws, 3);
    let view = ws.call("analysis.explain_date", json!({ "task": uid })).unwrap();
    assert!(text.starts_with(view["summary_fr"].as_str().unwrap()), "{text}");
    assert!(text.contains(view["late"]["finish"].as_str().unwrap()), "{text}");
    assert!(text.contains("Marge totale"), "{text}");
    let o = run(&["--now", NOW, "--lang", "en", "explain", s(&house), "--task", "3"]);
    assert_eq!(code(&o), 0);
    let text = String::from_utf8(o.stdout).unwrap();
    assert!(text.starts_with(view["summary_en"].as_str().unwrap()), "{text}");
    let o = run(&["--help"]);
    assert_eq!(code(&o), 0);
    let help = String::from_utf8(o.stdout).unwrap();
    assert!(help.contains("explain"), "{help}");
}

#[test]
fn cli_usage_errors_exit_2() {
    let house = house_path();
    let o = run(&["--help"]);
    assert_eq!(code(&o), 0);
    let help = String::from_utf8(o.stdout).unwrap();
    assert!(help.contains("summary") && help.contains("import-csv"), "{help}");
    // Usage : sous-commande inconnue, argument manquant.
    assert_eq!(code(&run(&["inconnue"])), 2);
    assert_eq!(code(&run(&["summary"])), 2);
    assert_eq!(code(&run(&["mcp", "--http", ":8080"])), 2);
    // `--approval-timeout` hors de 1..=3600 s (J7, relais) : usage.
    for bad in ["0", "3601", "x"] {
        assert_eq!(code(&run(&["mcp", "--approval-timeout", bad])), 2, "{bad}");
    }
    // `--limit` hors de 1..500 : usage (2), même sur un plan absent (le
    // contrôle passe avant la lecture) ; bornes acceptées.
    for bad in ["0", "501"] {
        assert_eq!(code(&run(&["tasks", "absent.xml", "--limit", bad])), 2, "{bad}");
        assert_eq!(code(&run(&["tasks", s(&house), "--limit", bad])), 2, "{bad}");
    }
    for ok in ["1", "500"] {
        assert_eq!(
            code(&run(&["--now", NOW, "tasks", s(&house), "--limit", ok])),
            0,
            "{ok}"
        );
    }
}

#[test]
fn cli_import_csv_roundtrip() {
    let dir = TempDir::new("csv");
    let csv = dir.join("taches.csv");
    std::fs::write(
        &csv,
        "Nom;Durée;Prédécesseurs\nFondations;5j;\nMurs;10j;1\nToit;3j;2\n",
    )
    .unwrap();
    let out = dir.join("taches.xml");
    let o = run(&["--now", NOW, "import-csv", s(&csv), "-o", s(&out)]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let text = String::from_utf8(o.stdout).unwrap();
    assert!(text.contains("3 tâche(s), 2 lien(s)"), "{text}");
    // Relu : mêmes tâches que l'import en mémoire (ce que fait le MCP).
    let mut ws = workspace_with(&out, Language::Fr);
    let reread = ws.call("task.list", json!({})).unwrap();
    let mut direct = Workspace::new(now(), Language::Fr);
    direct
        .call("project.import_csv", json!({ "path": s(&csv) }))
        .unwrap();
    let expected = direct.call("task.list", json!({})).unwrap();
    assert_eq!(reread["tasks"], expected["tasks"]);
    assert_eq!(reread["total"], json!(3));
    // CSV vide : erreur de fichier (3).
    let empty = dir.join("vide.csv");
    std::fs::write(&empty, "").unwrap();
    let o = run(&["--now", NOW, "import-csv", s(&empty), "-o", s(&out)]);
    assert_eq!(code(&o), 3, "{}", String::from_utf8_lossy(&o.stderr));
}

#[test]
fn cli_lang_flag_and_env() {
    let house = house_path();
    let o = run(&["--now", NOW, "--lang", "en", "summary", s(&house)]);
    assert_eq!(code(&o), 0);
    assert!(String::from_utf8(o.stdout).unwrap().contains("Finish"));
    let o = plannercraft()
        .env("LANG", "en_US.UTF-8")
        .args(["--now", NOW, "summary", s(&house)])
        .output()
        .unwrap();
    assert!(String::from_utf8(o.stdout).unwrap().contains("Finish"));
    let o = run(&["--now", NOW, "summary", s(&house)]);
    assert!(String::from_utf8(o.stdout).unwrap().contains("Fin"));
}

#[test]
fn cli_mcp_http_options_checked() {
    let dir = TempDir::new("http");
    let root = s(dir.path());
    // --root obligatoire, --root sans --http, adresse invalide : usage.
    assert_eq!(code(&run(&["mcp", "--http", ":0"])), 2);
    assert_eq!(code(&run(&["mcp", "--root", root])), 2);
    assert_eq!(code(&run(&["mcp", "--http", "exemple:80", "--root", root])), 2);
    // Adresse non locale sans --allow-remote, puis sans jeton : usage.
    let o = run(&["mcp", "--http", "192.0.2.1:8080", "--root", root]);
    assert_eq!(code(&o), 2);
    assert!(String::from_utf8_lossy(&o.stderr).contains("--allow-remote"));
    let o = run(&[
        "mcp",
        "--http",
        "192.0.2.1:8080",
        "--root",
        root,
        "--allow-remote",
    ]);
    assert_eq!(code(&o), 2);
    assert!(String::from_utf8_lossy(&o.stderr).contains("--token-file"));
    // Racine introuvable : erreur de fichier.
    let missing = dir.join("absent");
    assert_eq!(code(&run(&["mcp", "--http", ":0", "--root", s(&missing)])), 3);
}

/// Plan à ressources (J5) enregistré dans `dir` : Maçon (ID 1) sur A et,
/// à 150 %, sur B en parallèle (surutilisé la première semaine),
/// Électricien (ID 2) sur C.
fn resource_plan(dir: &TempDir) -> PathBuf {
    let path = dir.join("chantier.xml");
    let mut ws = Workspace::new(now(), Language::Fr);
    ws.call(
        "project.new",
        json!({ "name": "Chantier", "start": "2026-10-19T08:00" }),
    )
    .unwrap();
    ws.call(
        "batch.apply",
        json!({ "ops": [
            { "op": "resource.add", "name": "Maçon", "as": "$m" },
            { "op": "resource.add", "name": "Électricien", "as": "$e" },
            { "op": "task.add", "name": "A", "as": "$a", "duration": { "text": "5j" } },
            { "op": "task.add", "name": "B", "as": "$b", "duration": { "text": "5j" } },
            { "op": "task.add", "name": "C", "as": "$c", "duration": { "text": "5j" } },
            { "op": "assignment.add", "task": "$a", "resource": "$m" },
            { "op": "assignment.add", "task": "$b", "resource": "$m", "units": { "text": "150%" } },
            { "op": "assignment.add", "task": "$c", "resource": "$e" }
        ] }),
    )
    .unwrap();
    ws.call("project.save", json!({ "path": s(&path) })).unwrap();
    path
}

#[test]
fn cli_resources_json() {
    let dir = TempDir::new("resources");
    let plan = resource_plan(&dir);
    let o = run(&["--now", NOW, "resources", s(&plan), "--json"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let cli = stdout_json(&o);
    let mut ws = workspace_with(&plan, Language::Fr);
    let view = ws.call("resource.list", json!({ "limit": 500 })).unwrap();
    assert_eq!(cli["resources"], view["resources"]);
    assert_eq!(cli["total"], json!(2));
    assert_eq!(cli["resources"][0]["name"], json!("Maçon"));
    assert_eq!(cli["resources"][0]["overallocated"], json!(true));
    // Filtre : la seule ressource surutilisée.
    let o = run(&["--now", NOW, "resources", s(&plan), "--json", "--overallocated"]);
    assert_eq!(code(&o), 0);
    assert_eq!(stdout_json(&o)["total"], json!(1));
    // Texte : en-tête, deux lignes, pied ; « ! » devant le Maçon.
    let o = run(&["--now", NOW, "resources", s(&plan)]);
    assert_eq!(code(&o), 0);
    let text = String::from_utf8(o.stdout).unwrap();
    assert_eq!(text.lines().count(), 4, "{text}");
    assert!(
        text.lines().any(|l| l.starts_with("!1") && l.contains("Maçon")),
        "{text}"
    );
    // `tasks --resource ID` : tâches du Maçon (ID 1 → son UID).
    let o = run(&["--now", NOW, "tasks", s(&plan), "--json", "--resource", "1"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let uid = view["resources"][0]["uid"].clone();
    let mcp = ws
        .call("task.list", json!({ "resource": uid, "limit": 500 }))
        .unwrap();
    assert_eq!(stdout_json(&o)["tasks"], mcp["tasks"]);
    assert_eq!(mcp["total"], json!(2));
    // ID inconnu : refus (code 1), erreur structurée en `--json`.
    let o = run(&["--now", NOW, "tasks", s(&plan), "--json", "--resource", "9"]);
    assert_eq!(code(&o), 1);
    assert_eq!(stdout_json(&o)["error"]["code"], json!("resource.not_found"));
}

#[test]
fn cli_usage_by_week() {
    let dir = TempDir::new("usage");
    let plan = resource_plan(&dir);
    let o = run(&[
        "--now",
        NOW,
        "usage",
        s(&plan),
        "--json",
        "--by",
        "week",
        "--resource",
        "1",
    ]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let cli = stdout_json(&o);
    let mut ws = workspace_with(&plan, Language::Fr);
    let uid = ws.call("resource.list", json!({})).unwrap()["resources"][0]["uid"].clone();
    let view = ws
        .call(
            "analysis.resource_usage",
            json!({ "by": "week", "resources": [uid] }),
        )
        .unwrap();
    assert_eq!(cli, view);
    assert_eq!(cli["by"], json!("week"));
    // 40 h + 60 h la semaine du lundi 19/10, surutilisée.
    assert_eq!(cli["rows"][0]["work"][0]["minutes"], json!(6_000));
    assert_eq!(cli["rows"][0]["overallocated"][0], json!(true));
    // Plage explicite, toutes les ressources, par jour.
    let o = run(&[
        "--now",
        NOW,
        "usage",
        s(&plan),
        "--json",
        "--by",
        "day",
        "--from",
        "2026-10-19",
        "--to",
        "2026-10-21",
    ]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let days = stdout_json(&o);
    assert_eq!(days["periods"], json!(["2026-10-19T00:00", "2026-10-20T00:00"]));
    assert_eq!(days["rows"].as_array().unwrap().len(), 2);
    // Texte : nom puis une ligne par période, « ! » surutilisée.
    let o = run(&["--now", NOW, "usage", s(&plan), "--resource", "1"]);
    assert_eq!(code(&o), 0);
    let text = String::from_utf8(o.stdout).unwrap();
    assert!(text.starts_with("Maçon\n! 2026-10-19T00:00"), "{text}");
    // Granularité inconnue : erreur d'usage (code 2).
    let o = run(&["--now", NOW, "usage", s(&plan), "--by", "year"]);
    assert_eq!(code(&o), 2);
}

/// Aide de `plannercraft tasks --late` alignée sur R-236 (arbitrage du
/// 2026-10-20) : les quatre critères et la date d'état effective.
#[test]
fn cli_tasks_late_help_follows_r236() {
    let o = run(&["tasks", "--help"]);
    assert_eq!(code(&o), 0);
    let text = String::from_utf8_lossy(&o.stdout)
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for part in [
        "fin planifiée après l'échéance ou après la fin de la ligne de base 0",
        "non achevée avec une fin planifiée au plus tard à la date d'état",
        "non commencée avec un début planifié avant la date d'état",
        "Date d'état : celle du projet, sinon la date du jour",
    ] {
        assert!(text.contains(part), "« {part} » absent de : {text}");
    }
}

/// J8 (lot L6) : `plannercraft style list|get|apply` ; `apply` écrit le style dans
/// le fichier (`<?plannercraft-style?>`), `--dry-run` ne touche rien, `--reset`
/// redonne le fichier sans style ; les JSON sont ceux du MCP.
#[test]
fn cli_style_list_get_apply() {
    let dir = TempDir::new("style");
    let house = house_copy(&dir);
    let original = std::fs::read(&house).unwrap();
    let o = run(&["--now", NOW, "style", "list", s(&house), "--json"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let mut ws = workspace_with(&house, Language::Fr);
    assert_eq!(stdout_json(&o), ws.call("style.list", json!({})).unwrap());
    let o = run(&[
        "--now",
        NOW,
        "style",
        "apply",
        s(&house),
        "--template",
        "sobre",
        "--dry-run",
    ]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    assert_eq!(std::fs::read(&house).unwrap(), original);
    let patch = dir.join("patch.json");
    std::fs::write(&patch, r#"{"density":"airy"}"#).unwrap();
    let o = run(&[
        "--now",
        NOW,
        "style",
        "apply",
        s(&house),
        "--template",
        "presentation",
        "--patch",
        s(&patch),
    ]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let text = std::fs::read_to_string(&house).unwrap();
    assert!(
        text.contains(r#"<?plannercraft-style {"base":"presentation","over":{"density":"airy"},"v":1}?>"#),
        "{}",
        text.lines().take(3).collect::<Vec<_>>().join("\n")
    );
    let o = run(&["--now", NOW, "style", "get", s(&house), "--json"]);
    assert_eq!(code(&o), 0);
    let v = stdout_json(&o);
    assert_eq!(
        (v["source"].as_str(), v["base"].as_str()),
        (Some("document"), Some("presentation"))
    );
    let o = run(&["--now", NOW, "--lang", "en", "style", "get", s(&house)]);
    assert!(String::from_utf8_lossy(&o.stdout).starts_with("Template : presentation (document)"));
    // Refus : rien à appliquer (code 1), `--reset` avec `--template` (usage).
    assert_eq!(code(&run(&["--now", NOW, "style", "apply", s(&house)])), 1);
    assert_eq!(
        code(&run(&[
            "style",
            "apply",
            s(&house),
            "--reset",
            "--template",
            "sobre"
        ])),
        2
    );
    let o = run(&["--now", NOW, "style", "apply", s(&house), "--reset"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    assert!(
        !std::fs::read_to_string(&house)
            .unwrap()
            .contains("plannercraft-style")
    );
}

/// `plannercraft export-pdf` (J8, lot L7) : PDF écrit, plan inchangé, mêmes
/// octets à deux lancements ; A3 paysage sur 2 pages en largeur ; sortie
/// JSON = vue du MCP ; refus `param.invalid` (code 1).
#[test]
fn cli_export_pdf() {
    let dir = TempDir::new("pdf");
    let plan = house_copy(&dir);
    let before = std::fs::read(&plan).unwrap();
    let first = dir.join("a.pdf");
    let o = run(&[
        "--now",
        NOW,
        "export-pdf",
        s(&plan),
        s(&first),
        "--style",
        "sobre",
    ]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    assert!(String::from_utf8_lossy(&o.stdout).contains("page(s)"));
    let bytes = std::fs::read(&first).unwrap();
    assert!(bytes.starts_with(b"%PDF-"));
    assert_eq!(std::fs::read(&plan).unwrap(), before, "plan inchangé");
    let second = dir.join("b.pdf");
    let o = run(&[
        "--now",
        NOW,
        "export-pdf",
        s(&plan),
        s(&second),
        "--style",
        "sobre",
    ]);
    assert_eq!(code(&o), 0);
    assert_eq!(std::fs::read(&second).unwrap(), bytes, "H-R13 : mêmes octets");
    let a3 = dir.join("c.pdf");
    let o = run(&[
        "--now",
        NOW,
        "export-pdf",
        s(&plan),
        s(&a3),
        "--paper",
        "A3",
        "--landscape",
        "--pages-wide",
        "2",
        "--json",
    ]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let view = stdout_json(&o);
    assert!(view["pages"].as_u64().unwrap() >= 1);
    assert_eq!(view["path"], json!(s(&a3)));
    let text = String::from_utf8_lossy(&std::fs::read(&a3).unwrap()).into_owned();
    assert!(text.contains("/MediaBox[0 0 1191 842]"));
    let o = run(&[
        "--now",
        NOW,
        "export-pdf",
        s(&plan),
        s(&dir.join("d.pdf")),
        "--from",
        "2027-01-01",
        "--to",
        "2026-01-01",
    ]);
    assert_eq!(code(&o), 1);
    assert!(String::from_utf8_lossy(&o.stderr).contains("param.invalid"));
    let o = run(&["export-pdf", s(&plan), s(&dir.join("e.pdf")), "--pages-wide", "0"]);
    assert_eq!(code(&o), 2, "usage");
}

/// `plannercraft import-pdf` (ADR-0032, lot P1a) : un PDF exporté avec les
/// colonnes d'impression se relit en MSPDI (rapport texte et JSON) ; un PDF
/// sans dates (colonnes N° et Nom seules) est refusé avec le code 4 ; un
/// fichier absent donne le code 3.
#[test]
fn cli_import_pdf() {
    let dir = TempDir::new("importpdf");
    let plan = house_copy(&dir);
    // Sans colonnes de dates : refus.
    let bare = dir.join("bare.pdf");
    assert_eq!(code(&run(&["--now", NOW, "export-pdf", s(&plan), s(&bare)])), 0);
    let o = run(&["--now", NOW, "import-pdf", s(&bare), "-o", s(&dir.join("x.xml"))]);
    assert_eq!(code(&o), 4, "{}", String::from_utf8_lossy(&o.stderr));
    assert!(String::from_utf8_lossy(&o.stderr).contains("pdfimport.no_dates"));
    assert!(!dir.join("x.xml").exists());
    // Colonnes N°, Nom, Durée, Début, Fin.
    let mut page =
        plannercraft_render::style::template(plannercraft_render::style::TemplateId::Classique).page;
    page.columns = Some(plannercraft_render::style::TableColumn::PRINT_DEFAULT.to_vec());
    let batch = dir.join("style.json");
    let ops = json!({"format_version": 1, "ops": [{"op": "style.apply", "patch": {"page": page}}]});
    std::fs::write(&batch, ops.to_string()).unwrap();
    let o = run(&["--now", NOW, "apply", s(&plan), s(&batch)]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let pdf = dir.join("full.pdf");
    assert_eq!(code(&run(&["--now", NOW, "export-pdf", s(&plan), s(&pdf)])), 0);
    let out = dir.join("relu.xml");
    let o = run(&["--now", NOW, "--lang", "fr", "import-pdf", s(&pdf), "-o", s(&out)]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    let text = String::from_utf8_lossy(&o.stdout).into_owned();
    assert!(text.contains("Tâches lues"), "{text}");
    assert!(
        text.contains("Dates calculées par le moteur et dates lues"),
        "{text}"
    );
    let o = run(&["summary", s(&out), "--json"]);
    assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
    assert!(stdout_json(&o)["tasks"].as_u64().unwrap() > 0);
    let o = run(&["--now", NOW, "import-pdf", s(&pdf), "--dry-run", "--json"]);
    assert_eq!(code(&o), 0);
    let view = stdout_json(&o);
    assert!(view["tasks_imported"].as_u64().unwrap() > 0);
    assert!(view["conversion"]["dates_compared"].as_u64().unwrap() > 0);
    let o = run(&["import-pdf", s(&dir.join("absent.pdf")), "--dry-run"]);
    assert_eq!(code(&o), 3);
}

/// `plannercraft import-pdf` (lots P1b et P1c) : liens relus dans le dessin
/// posés par défaut (`--no-drawn-links` les coupe), jours fériés français
/// (`--holidays`) et périodes chômées déduites (`--periods`). PDF produit
/// par PlannerCraft : planning sans colonne Prédécesseurs, 14 juillet et
/// congés d'août chômés dans son calendrier.
#[test]
fn cli_import_pdf_drawn_links_holidays_periods() {
    use plannercraft_automation::{AnchorSpec, ApplyMode, Batch, NewProject, Session};
    let dir = TempDir::new("importpdflinks");
    let moment = |s: &str| s.parse::<Moment>().unwrap();
    let mut session = Session::new_project(
        NewProject {
            name: "Chantier".to_owned(),
            anchor: AnchorSpec::Start(moment("2026-07-06T08:00")),
            lang: Some(Language::Fr),
        },
        moment("2026-07-01T08:00"),
    )
    .unwrap();
    let mut page =
        plannercraft_render::style::template(plannercraft_render::style::TemplateId::Classique).page;
    page.columns = Some(plannercraft_render::style::TableColumn::PRINT_DEFAULT.to_vec());
    let batch = json!({
        "format_version": 1,
        "lang": "fr",
        "ops": [
            {"op": "calendar.set_exception", "cal": "project", "from": "2026-07-14", "to": "2026-07-14", "ranges": [], "name": "Fête nationale"},
            {"op": "calendar.set_exception", "cal": "project", "from": "2026-08-03", "to": "2026-08-21", "ranges": [], "name": "Congés"},
            {"op": "task.add", "name": "Terrassement", "as": "$t", "duration": {"text": "5j"}},
            {"op": "task.add", "name": "Fondations", "as": "$f", "duration": {"text": "10j"}},
            {"op": "task.add", "name": "Élévation", "as": "$e", "duration": {"text": "24j"}},
            {"op": "task.add", "name": "Réseaux", "as": "$r", "duration": {"text": "4j"}},
            {"op": "task.add", "name": "Charpente", "as": "$c", "duration": {"text": "20j"}},
            {"op": "link.add", "pred": "$t", "succ": "$f"},
            {"op": "link.add", "pred": "$f", "succ": "$e"},
            {"op": "link.add", "pred": "$e", "succ": "$r", "kind": "ss"},
            {"op": "link.add", "pred": "$r", "succ": "$c"},
            {"op": "style.apply", "patch": {"page": page}},
        ]
    });
    session
        .apply(&Batch::from_json(batch).unwrap(), ApplyMode::Commit)
        .unwrap();
    let plan = dir.join("chantier.xml");
    let written = session.save_bytes(&plannercraft_mspdi::write::WriteOptions::default());
    std::fs::write(&plan, &written.xml).unwrap();
    let pdf = dir.join("chantier.pdf");
    assert_eq!(code(&run(&["--now", NOW, "export-pdf", s(&plan), s(&pdf)])), 0);
    let import = |extra: &[&str]| {
        let mut args = vec![
            "--now",
            NOW,
            "--lang",
            "fr",
            "import-pdf",
            s(&pdf),
            "--dry-run",
            "--json",
        ];
        args.extend_from_slice(extra);
        let o = run(&args);
        assert_eq!(code(&o), 0, "{}", String::from_utf8_lossy(&o.stderr));
        stdout_json(&o)
    };
    // Liens dessinés posés par défaut.
    let view = import(&[]);
    assert_eq!(view["drawn"]["links"].as_array().unwrap().len(), 4);
    assert_eq!(view["conversion"]["drawn"]["placed"], 4);
    assert_eq!(view["conversion"]["links"], 4);
    let kinds: Vec<&str> = view["drawn"]["links"]
        .as_array()
        .unwrap()
        .iter()
        .map(|l| l["kind"].as_str().unwrap())
        .collect();
    assert_eq!(kinds, ["fs", "fs", "ss", "fs"]);
    // Coupés.
    let view = import(&["--no-drawn-links"]);
    assert_eq!(view["conversion"]["links"], 0);
    assert!(view["conversion"].get("drawn").is_none());
    // Fériés seuls, puis fériés et périodes déduites : toutes les dates
    // retombent sur le PDF.
    let standard = import(&[]);
    let holidays = import(&["--holidays"]);
    let both = import(&["--holidays", "--periods"]);
    let equal = |v: &Value| v["conversion"]["dates_equal"].as_u64().unwrap();
    let compared = both["conversion"]["dates_compared"].as_u64().unwrap();
    assert!(
        equal(&standard) < equal(&holidays),
        "{} {}",
        equal(&standard),
        equal(&holidays)
    );
    assert!(equal(&holidays) < compared);
    assert_eq!(equal(&both), compared);
    // Texte : ligne des liens dessinés.
    let o = run(&["--now", NOW, "--lang", "fr", "import-pdf", s(&pdf), "--dry-run"]);
    let text = String::from_utf8_lossy(&o.stdout).into_owned();
    assert!(text.contains("Liens dessinés : "), "{text}");
    assert!(text.contains("Liens dessinés posés"), "{text}");
}
