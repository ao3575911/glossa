use crate::error::IoResult;
use crate::types::{Document, DocumentKind};
use chardetng::{EncodingDetector, Iso2022JpDetection, Utf8Detection};
use encoding_rs::Encoding;
use indexmap::IndexMap;
use std::fs;
use std::path::Path;

/// Detect encoding with chardetng and decode with encoding_rs.
pub fn decode_bytes(bytes: &[u8]) -> IoResult<String> {
    let mut detector = EncodingDetector::new(Iso2022JpDetection::Deny);
    detector.feed(bytes, true);
    let encoding: &'static Encoding = detector.guess(None, Utf8Detection::Allow);
    let (cow, _enc, had_errors) = encoding.decode(bytes);
    if had_errors {
        // Fall back to lossy UTF-8 rather than failing hard on mixed corpora.
        return Ok(String::from_utf8_lossy(bytes).into_owned());
    }
    Ok(cow.into_owned())
}

pub fn import_txt(path: &Path) -> IoResult<Document> {
    let bytes = fs::read(path)?;
    let text = decode_bytes(&bytes)?;
    Ok(Document {
        filename: path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string(),
        source_path: path.to_path_buf(),
        kind: DocumentKind::Txt,
        text,
        metadata: IndexMap::new(),
        annotations: None,
    })
}
