use crate::output::{self, param_str, params_from, EmitArgs};
use anyhow::Result;
use chrono::Utc;
use glossa_commands::init as init_cmd;
use glossa_core::OutputMode;
use std::path::PathBuf;
use std::time::Instant;

pub fn run(dir: Option<PathBuf>, name: Option<String>, mode: OutputMode) -> Result<()> {
    let started = Instant::now();
    let started_at = Utc::now();
    let target = dir.clone().unwrap_or_else(|| PathBuf::from("."));
    let (ws, data) = init_cmd::execute(dir, name)?;

    let params = params_from([
        param_str("dir", target.display().to_string()),
        param_str("name", data.name.clone()),
    ]);

    let _ = ws.session_log().append(&glossa_workspace::SessionEvent {
        ts: started_at,
        command: "init".into(),
        args: serde_json::json!({ "dir": target, "name": &data.name }),
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
