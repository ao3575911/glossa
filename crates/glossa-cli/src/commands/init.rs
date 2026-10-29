use crate::output::{self, param_str, params_from, EmitArgs};
use anyhow::Result;
use chrono::Utc;
use glossa_core::OutputMode;
use glossa_workspace::Workspace;
use serde::Serialize;
use std::path::PathBuf;
use std::time::Instant;

#[derive(Debug, Serialize)]
struct InitData {
    root: String,
    name: String,
    config: String,
    db: String,
    session_log: String,
}

pub fn run(dir: Option<PathBuf>, name: Option<String>, mode: OutputMode) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();
    let target = dir.unwrap_or_else(|| PathBuf::from("."));
    let ws = Workspace::init(&target, name.as_deref())?;

    let data = InitData {
        root: ws.root.display().to_string(),
        name: ws.config.name.clone(),
        config: ws.root.join("glossa.toml").display().to_string(),
        db: ws.dot_dir().join("workspace.db").display().to_string(),
        session_log: ws.session_log().path().display().to_string(),
    };

    let params = params_from([
        param_str("dir", target.display().to_string()),
        param_str("name", ws.config.name.clone()),
    ]);

    let _ = ws.session_log().append(&glossa_workspace::SessionEvent {
        ts: started_at,
        command: "init".into(),
        args: serde_json::json!({ "dir": target, "name": &ws.config.name }),
        ok: true,
        error: None,
        duration_ms: Some(started.elapsed().as_millis() as u64),
    });

    output::emit(
        EmitArgs {
            mode,
            command: "init",
            params,
            started_at,
            duration: started.elapsed(),
            corpus_hash: None,
            data: &data,
        },
        |d| {
            println!("Initialized Glossa workspace '{}'", d.name);
            println!("  root: {}", d.root);
            println!("  config: {}", d.config);
            println!("  db: {}", d.db);
            println!("  session log: {}", d.session_log);
        },
    )?;
    Ok(())
}
