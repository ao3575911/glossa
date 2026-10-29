use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

fn glossa_bin() -> PathBuf {
    env!("CARGO_BIN_EXE_glossa").into()
}

fn fixture_corpus() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/fixtures/corpus")
}

fn run_glossa(args: &[&str], cwd: &std::path::Path) -> (i32, String, String) {
    let output = Command::new(glossa_bin())
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("run glossa");
    (
        output.status.code().unwrap_or(1),
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// Redact volatile envelope fields for golden comparison.
fn redact_envelope(json: &str) -> serde_json::Value {
    let mut v: serde_json::Value = serde_json::from_str(json).expect("json");
    if let Some(obj) = v.as_object_mut() {
        obj.insert("started_at".into(), serde_json::json!("<ts>"));
        obj.insert("duration_ms".into(), serde_json::json!(0));
        if let Some(hash) = obj.get("corpus_hash").cloned() {
            if !hash.is_null() {
                obj.insert("corpus_hash".into(), serde_json::json!("<hash>"));
            }
        }
        if let Some(data) = obj.get_mut("data").and_then(|d| d.as_object_mut()) {
            if data.contains_key("corpus_hash") {
                data.insert("corpus_hash".into(), serde_json::json!("<hash>"));
            }
            if data.contains_key("path") {
                data.insert("path".into(), serde_json::json!("<path>"));
            }
            if data.contains_key("root") {
                data.insert("root".into(), serde_json::json!("<root>"));
            }
            if data.contains_key("config") {
                data.insert("config".into(), serde_json::json!("<config>"));
            }
            if data.contains_key("db") {
                data.insert("db".into(), serde_json::json!("<db>"));
            }
            if data.contains_key("session_log") {
                data.insert("session_log".into(), serde_json::json!("<log>"));
            }
            if let Some(corpus) = data.get_mut("corpus").and_then(|c| c.as_object_mut()) {
                corpus.insert("path".into(), serde_json::json!("<path>"));
                corpus.insert("corpus_hash".into(), serde_json::json!("<hash>"));
                corpus.insert("created_at".into(), serde_json::json!("<ts>"));
            }
            // Hit positions are stable for a fixed fixture; leave them.
        }
        if let Some(params) = obj.get_mut("params").and_then(|p| p.as_object_mut()) {
            if params.contains_key("dir") {
                params.insert("dir".into(), serde_json::json!("."));
            }
            if params.contains_key("path") {
                params.insert("path".into(), serde_json::json!("<fixture>"));
            }
        }
    }
    v
}

#[test]
fn corpus_add_and_kwic_json_golden() {
    let dir = tempdir().expect("temp");
    let (code, out, err) = run_glossa(&["init", "--json", "--name", "fixture", "."], dir.path());
    assert_eq!(code, 0, "init failed: {err}\n{out}");
    let init_json = redact_envelope(&out);
    insta::assert_json_snapshot!("init_json", init_json);

    let fixture = fixture_corpus().join("alice_excerpt.txt");
    let meta = fixture_corpus().join("meta.csv");
    let (code, out, err) = run_glossa(
        &[
            "corpus",
            "add",
            fixture.to_str().unwrap(),
            "--name",
            "alice",
            "--meta",
            meta.to_str().unwrap(),
            "--json",
        ],
        dir.path(),
    );
    assert_eq!(code, 0, "corpus add failed: {err}\n{out}");
    let add_json = redact_envelope(&out);
    // Drop document list volatility already stable; assert structure.
    insta::assert_json_snapshot!("corpus_add_json", add_json);

    let (code, out, err) = run_glossa(
        &[
            "kwic",
            "Alice",
            "--corpus",
            "alice",
            "--sort",
            "R1",
            "--context",
            "3",
            "--limit",
            "5",
            "--json",
        ],
        dir.path(),
    );
    assert_eq!(code, 0, "kwic failed: {err}\n{out}");
    let kwic_json = redact_envelope(&out);
    insta::assert_json_snapshot!("kwic_alice_json", kwic_json);

    // Session log should have recorded commands.
    let log = std::fs::read_to_string(dir.path().join(".glossa/session.jsonl")).expect("log");
    assert!(
        log.lines().count() >= 2,
        "expected session log lines, got:\n{log}"
    );
}

#[test]
fn version_json() {
    let dir = tempdir().expect("temp");
    let _ = run_glossa(&["init", "."], dir.path());
    let (code, out, err) = run_glossa(&["--version", "--json"], dir.path());
    assert_eq!(code, 0, "{err}");
    let v = redact_envelope(&out);
    assert_eq!(v["command"], "version");
    assert_eq!(v["data"]["version"], env!("CARGO_PKG_VERSION"));
}
