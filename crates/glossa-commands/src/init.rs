use anyhow::Result;
use glossa_workspace::Workspace;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize)]
pub struct InitResult {
    pub root: String,
    pub name: String,
    pub config: String,
    pub db: String,
    pub session_log: String,
}

pub fn execute(dir: Option<PathBuf>, name: Option<String>) -> Result<(Workspace, InitResult)> {
    let target = dir.unwrap_or_else(|| PathBuf::from("."));
    let ws = Workspace::init(&target, name.as_deref())?;
    let data = InitResult {
        root: ws.root.display().to_string(),
        name: ws.config.name.clone(),
        config: ws.root.join("glossa.toml").display().to_string(),
        db: ws.dot_dir().join("workspace.db").display().to_string(),
        session_log: ws.session_log().path().display().to_string(),
    };
    Ok((ws, data))
}
