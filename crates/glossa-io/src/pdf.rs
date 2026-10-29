use crate::error::{IoError, IoResult};
use crate::types::{Document, DocumentKind};
use indexmap::IndexMap;
use std::path::Path;

/// Extract the PDF text layer (not OCR). Warns via empty text for scanned PDFs.
pub fn import_pdf(path: &Path) -> IoResult<Document> {
    let text = pdf_extract::extract_text(path).map_err(|e| IoError::Pdf(e.to_string()))?;
    Ok(Document {
        filename: path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string(),
        source_path: path.to_path_buf(),
        kind: DocumentKind::Pdf,
        text: text.trim().to_string(),
        metadata: IndexMap::new(),
        annotations: None,
    })
}
