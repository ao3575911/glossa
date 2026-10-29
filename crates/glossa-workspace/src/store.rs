use crate::config::{self, default_glossa_toml, WorkspaceConfig, CONFIG_FILE, DOT_DIR};
use crate::error::{WorkspaceError, WorkspaceResult};
use crate::log::SessionLog;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, OptionalExtension};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub const DB_FILE: &str = "workspace.db";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusRecord {
    pub name: String,
    pub path: String,
    pub token_count: u64,
    pub doc_count: u64,
    pub sentence_count: u64,
    pub corpus_hash: String,
    pub created_at: DateTime<Utc>,
}

pub struct Workspace {
    pub root: PathBuf,
    pub config: WorkspaceConfig,
    conn: Connection,
    log: SessionLog,
}

impl Workspace {
    pub fn dot_dir(&self) -> PathBuf {
        self.root.join(DOT_DIR)
    }

    pub fn corpora_dir(&self) -> PathBuf {
        self.dot_dir().join("corpora")
    }

    pub fn session_log(&self) -> &SessionLog {
        &self.log
    }

    pub fn connection(&self) -> &Connection {
        &self.conn
    }

    /// Create a new workspace at `dir` (defaults to cwd semantics handled by caller).
    pub fn init(dir: &Path, name: Option<&str>) -> WorkspaceResult<Self> {
        let root = if dir.as_os_str().is_empty() {
            PathBuf::from(".")
        } else {
            dir.to_path_buf()
        };
        fs::create_dir_all(&root)?;
        let root = root.canonicalize().unwrap_or(root);

        let config_path = root.join(CONFIG_FILE);
        if config_path.exists() {
            return Err(WorkspaceError::AlreadyExists(root.display().to_string()));
        }

        let ws_name = name.map(str::to_string).unwrap_or_else(|| {
            root.file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("glossa")
                .to_string()
        });

        fs::write(&config_path, default_glossa_toml(&ws_name))?;

        let dot = root.join(DOT_DIR);
        fs::create_dir_all(dot.join("corpora"))?;

        let db_path = dot.join(DB_FILE);
        let conn = Connection::open(&db_path)?;
        Self::migrate(&conn)?;

        let log = SessionLog::open(&dot);
        log.ensure_exists()?;

        Ok(Self {
            root,
            config: WorkspaceConfig {
                name: ws_name,
                version: 1,
            },
            conn,
            log,
        })
    }

    pub fn open(start: &Path) -> WorkspaceResult<Self> {
        let root = config::find_workspace_root(start)?;
        let config = config::load_config(&root)?;
        let dot = root.join(DOT_DIR);
        fs::create_dir_all(dot.join("corpora"))?;
        let conn = Connection::open(dot.join(DB_FILE))?;
        Self::migrate(&conn)?;
        let log = SessionLog::open(&dot);
        log.ensure_exists()?;
        Ok(Self {
            root,
            config,
            conn,
            log,
        })
    }

    fn migrate(conn: &Connection) -> WorkspaceResult<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS corpora (
                name TEXT PRIMARY KEY NOT NULL,
                path TEXT NOT NULL,
                token_count INTEGER NOT NULL,
                doc_count INTEGER NOT NULL,
                sentence_count INTEGER NOT NULL,
                corpus_hash TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                corpus TEXT NOT NULL,
                filename TEXT NOT NULL,
                title TEXT,
                author TEXT,
                year TEXT,
                extra_json TEXT NOT NULL DEFAULT '{}',
                token_count INTEGER NOT NULL DEFAULT 0,
                UNIQUE(corpus, filename),
                FOREIGN KEY(corpus) REFERENCES corpora(name) ON DELETE CASCADE
            );
            "#,
        )?;
        Ok(())
    }

    pub fn upsert_corpus(&self, record: &CorpusRecord) -> WorkspaceResult<()> {
        self.conn.execute(
            r#"
            INSERT INTO corpora (name, path, token_count, doc_count, sentence_count, corpus_hash, created_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(name) DO UPDATE SET
                path=excluded.path,
                token_count=excluded.token_count,
                doc_count=excluded.doc_count,
                sentence_count=excluded.sentence_count,
                corpus_hash=excluded.corpus_hash,
                created_at=excluded.created_at
            "#,
            params![
                record.name,
                record.path,
                record.token_count as i64,
                record.doc_count as i64,
                record.sentence_count as i64,
                record.corpus_hash,
                record.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn list_corpora(&self) -> WorkspaceResult<Vec<CorpusRecord>> {
        let mut stmt = self.conn.prepare(
            "SELECT name, path, token_count, doc_count, sentence_count, corpus_hash, created_at
             FROM corpora ORDER BY name ASC",
        )?;
        let rows = stmt.query_map([], |row| {
            let created_at: String = row.get(6)?;
            Ok(CorpusRecord {
                name: row.get(0)?,
                path: row.get(1)?,
                token_count: row.get::<_, i64>(2)? as u64,
                doc_count: row.get::<_, i64>(3)? as u64,
                sentence_count: row.get::<_, i64>(4)? as u64,
                corpus_hash: row.get(5)?,
                created_at: DateTime::parse_from_rfc3339(&created_at)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
            })
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn get_corpus(&self, name: &str) -> WorkspaceResult<CorpusRecord> {
        let mut stmt = self.conn.prepare(
            "SELECT name, path, token_count, doc_count, sentence_count, corpus_hash, created_at
             FROM corpora WHERE name = ?1",
        )?;
        let row = stmt
            .query_row(params![name], |row| {
                let created_at: String = row.get(6)?;
                Ok(CorpusRecord {
                    name: row.get(0)?,
                    path: row.get(1)?,
                    token_count: row.get::<_, i64>(2)? as u64,
                    doc_count: row.get::<_, i64>(3)? as u64,
                    sentence_count: row.get::<_, i64>(4)? as u64,
                    corpus_hash: row.get(5)?,
                    created_at: DateTime::parse_from_rfc3339(&created_at)
                        .map(|d| d.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now()),
                })
            })
            .optional()?;
        row.ok_or_else(|| WorkspaceError::CorpusNotFound(name.to_string()))
    }

    pub fn remove_corpus(&self, name: &str) -> WorkspaceResult<CorpusRecord> {
        let record = self.get_corpus(name)?;
        self.conn
            .execute("DELETE FROM documents WHERE corpus = ?1", params![name])?;
        self.conn
            .execute("DELETE FROM corpora WHERE name = ?1", params![name])?;
        let path = self.corpora_dir().join(name);
        if path.exists() {
            fs::remove_dir_all(&path)?;
        }
        Ok(record)
    }

    pub fn insert_document(
        &self,
        corpus: &str,
        filename: &str,
        meta: &serde_json::Map<String, serde_json::Value>,
        token_count: u64,
    ) -> WorkspaceResult<()> {
        let title = meta
            .get("title")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let author = meta
            .get("author")
            .and_then(|v| v.as_str())
            .map(str::to_string);
        let year = meta.get("year").map(|v| match v {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            _ => v.to_string(),
        });
        let extra = serde_json::to_string(meta)?;
        self.conn.execute(
            r#"
            INSERT INTO documents (corpus, filename, title, author, year, extra_json, token_count)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(corpus, filename) DO UPDATE SET
                title=excluded.title,
                author=excluded.author,
                year=excluded.year,
                extra_json=excluded.extra_json,
                token_count=excluded.token_count
            "#,
            params![
                corpus,
                filename,
                title,
                author,
                year,
                extra,
                token_count as i64
            ],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn init_and_open_roundtrip() {
        let dir = tempdir().expect("temp");
        let ws = Workspace::init(dir.path(), Some("demo")).expect("init");
        assert!(ws.root.join(CONFIG_FILE).exists());
        assert!(ws.dot_dir().join(DB_FILE).exists());
        assert!(ws.session_log().path().exists());

        let opened = Workspace::open(dir.path()).expect("open");
        assert_eq!(opened.config.name, "demo");
    }
}
