use crate::error::{WorkspaceError, WorkspaceResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const CONFIG_FILE: &str = "glossa.toml";
pub const DOT_DIR: &str = ".glossa";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkspaceConfig {
    pub name: String,
    #[serde(default = "default_version")]
    pub version: u32,
}

fn default_version() -> u32 {
    1
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self {
            name: "glossa".to_string(),
            version: 1,
        }
    }
}

/// Default contents of `glossa.toml` written by `glossa init`.
pub fn default_glossa_toml(name: &str) -> String {
    format!("# Glossa workspace configuration\nname = \"{name}\"\nversion = 1\n")
}

/// Walk upward from `start` looking for `glossa.toml`.
pub fn find_workspace_root(start: &Path) -> WorkspaceResult<PathBuf> {
    let mut cur = if start.is_file() {
        start
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| start.to_path_buf())
    } else {
        start.to_path_buf()
    };

    loop {
        let candidate = cur.join(CONFIG_FILE);
        if candidate.is_file() {
            return Ok(cur);
        }
        if !cur.pop() {
            break;
        }
    }
    Err(WorkspaceError::NotFound(start.display().to_string()))
}

pub fn load_config(root: &Path) -> WorkspaceResult<WorkspaceConfig> {
    let text = fs::read_to_string(root.join(CONFIG_FILE))?;
    parse_simple_toml(&text)
}

/// Minimal TOML subset parser for name/version (avoids adding toml dep for Stage 0).
fn parse_simple_toml(text: &str) -> WorkspaceResult<WorkspaceConfig> {
    let mut cfg = WorkspaceConfig::default();
    for line in text.lines() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim().trim_matches('"');
        match key {
            "name" => cfg.name = value.to_string(),
            "version" => {
                cfg.version = value
                    .parse()
                    .map_err(|_| WorkspaceError::Message(format!("invalid version: {value}")))?;
            }
            _ => {}
        }
    }
    Ok(cfg)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn finds_root_walking_up() {
        let dir = tempdir().expect("temp");
        fs::write(dir.path().join(CONFIG_FILE), default_glossa_toml("t")).expect("write");
        let nested = dir.path().join("a/b");
        fs::create_dir_all(&nested).expect("mkdir");
        let found = find_workspace_root(&nested).expect("find");
        assert_eq!(found, dir.path());
    }
}
