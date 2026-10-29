use crate::error::{IoError, IoResult};
use crate::types::{Document, DocumentKind};
use indexmap::IndexMap;
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

/// Import DOCX by reading `word/document.xml` from the zip package and extracting `w:t` text.
pub fn import_docx(path: &Path) -> IoResult<Document> {
    let file = File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut xml_file = archive
        .by_name("word/document.xml")
        .map_err(|e| IoError::Message(format!("DOCX missing document.xml: {e}")))?;
    let mut xml = String::new();
    xml_file.read_to_string(&mut xml)?;
    let text = extract_docx_text(&xml)?;
    Ok(Document {
        filename: path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string(),
        source_path: path.to_path_buf(),
        kind: DocumentKind::Docx,
        text,
        metadata: IndexMap::new(),
        annotations: None,
    })
}

fn extract_docx_text(xml: &str) -> IoResult<String> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);
    let mut buf = Vec::new();
    let mut out = String::new();
    let mut in_t = false;
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let local = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                match local.as_str() {
                    "t" => in_t = true,
                    "tab" => out.push('\t'),
                    "br" | "cr" => out.push('\n'),
                    "p" if !out.is_empty() && !out.ends_with('\n') => out.push('\n'),
                    _ => {}
                }
            }
            Ok(Event::End(e)) => {
                let local = String::from_utf8_lossy(e.local_name().as_ref()).into_owned();
                if local == "t" {
                    in_t = false;
                }
                if local == "p" && !out.ends_with('\n') {
                    out.push('\n');
                }
            }
            Ok(Event::Text(t)) => {
                if in_t {
                    let decoded = t.decode().map_err(|e| IoError::Xml(e.to_string()))?;
                    out.push_str(&decoded);
                }
            }
            Ok(Event::GeneralRef(r)) => {
                if in_t {
                    if let Some(ch) = r
                        .resolve_char_ref()
                        .map_err(|e| IoError::Xml(e.to_string()))?
                    {
                        out.push(ch);
                    } else {
                        let name = r.decode().map_err(|e| IoError::Xml(e.to_string()))?;
                        if let Some(v) = quick_xml::escape::resolve_predefined_entity(&name) {
                            out.push_str(v);
                        }
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(IoError::Xml(e.to_string())),
            _ => {}
        }
        buf.clear();
    }
    Ok(out.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_wt_nodes() {
        let xml = r#"<?xml version="1.0"?><w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main"><w:body><w:p><w:r><w:t>Hello</w:t></w:r><w:r><w:t xml:space="preserve"> world</w:t></w:r></w:p></w:body></w:document>"#;
        let text = extract_docx_text(xml).expect("parse");
        assert!(text.contains("Hello"));
        assert!(text.contains("world"));
    }
}
