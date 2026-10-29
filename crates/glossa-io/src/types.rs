use crate::conllu::import_conllu;
use crate::docx::import_docx;
use crate::error::{IoError, IoResult};
use crate::html::import_html;
use crate::pdf::import_pdf;
use crate::txt::import_txt;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocumentKind {
    Txt,
    Html,
    Docx,
    Pdf,
    Conllu,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AnnotatedToken {
    pub form: String,
    pub lemma: Option<String>,
    pub upos: Option<String>,
    pub xpos: Option<String>,
    pub feats: Option<String>,
    pub head: Option<u32>,
    pub deprel: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub filename: String,
    pub source_path: PathBuf,
    pub kind: DocumentKind,
    pub text: String,
    pub metadata: IndexMap<String, serde_json::Value>,
    /// When present (CoNLL-U), annotations are kept and used instead of re-tokenising.
    pub annotations: Option<Vec<Vec<AnnotatedToken>>>,
}

#[derive(Debug, Clone, Default)]
pub struct ImportBatch {
    pub documents: Vec<Document>,
}

impl ImportBatch {
    pub fn sort_deterministic(&mut self) {
        self.documents.sort_by(|a, b| {
            a.filename
                .cmp(&b.filename)
                .then(a.source_path.cmp(&b.source_path))
        });
    }
}

fn detect_kind(path: &Path) -> Option<DocumentKind> {
    let ext = path.extension()?.to_str()?.to_ascii_lowercase();
    match ext.as_str() {
        "txt" | "text" | "md" => Some(DocumentKind::Txt),
        "html" | "htm" => Some(DocumentKind::Html),
        "docx" => Some(DocumentKind::Docx),
        "pdf" => Some(DocumentKind::Pdf),
        "conllu" | "conll" => Some(DocumentKind::Conllu),
        _ => None,
    }
}

pub fn import_file(path: &Path) -> IoResult<Document> {
    let kind = detect_kind(path).ok_or_else(|| IoError::Unsupported(path.display().to_string()))?;
    let filename = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .to_string();
    let mut doc = match kind {
        DocumentKind::Txt => import_txt(path)?,
        DocumentKind::Html => import_html(path)?,
        DocumentKind::Docx => import_docx(path)?,
        DocumentKind::Pdf => import_pdf(path)?,
        DocumentKind::Conllu => import_conllu(path)?,
    };
    doc.filename = filename;
    doc.source_path = path.to_path_buf();
    doc.kind = kind;
    Ok(doc)
}

/// Import a file or recursively import a directory of supported documents.
pub fn import_path(path: &Path) -> IoResult<ImportBatch> {
    let mut batch = ImportBatch::default();
    if path.is_file() {
        batch.documents.push(import_file(path)?);
    } else if path.is_dir() {
        let mut paths: Vec<PathBuf> = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.into_path())
            .filter(|p| detect_kind(p).is_some())
            .collect();
        paths.sort();
        for p in paths {
            batch.documents.push(import_file(&p)?);
        }
    } else {
        return Err(IoError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("path not found: {}", path.display()),
        )));
    }
    batch.sort_deterministic();
    Ok(batch)
}

pub fn apply_metadata(
    batch: &mut ImportBatch,
    meta: &IndexMap<String, IndexMap<String, serde_json::Value>>,
) {
    for doc in &mut batch.documents {
        if let Some(m) = meta.get(&doc.filename) {
            for (k, v) in m {
                doc.metadata.insert(k.clone(), v.clone());
            }
        }
    }
}
