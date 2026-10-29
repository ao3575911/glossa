use crate::error::{IoError, IoResult};
use crate::txt::decode_bytes;
use crate::types::{AnnotatedToken, Document, DocumentKind};
use indexmap::IndexMap;
use std::fs;
use std::path::Path;

/// Import CoNLL-U, keeping token annotations and reconstructing surface text.
pub fn import_conllu(path: &Path) -> IoResult<Document> {
    let bytes = fs::read(path)?;
    let raw = decode_bytes(&bytes)?;
    let (sentences, text) = parse_conllu(&raw)?;
    Ok(Document {
        filename: path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string(),
        source_path: path.to_path_buf(),
        kind: DocumentKind::Conllu,
        text,
        metadata: IndexMap::new(),
        annotations: Some(sentences),
    })
}

fn parse_conllu(raw: &str) -> IoResult<(Vec<Vec<AnnotatedToken>>, String)> {
    let mut sentences = Vec::new();
    let mut current: Vec<AnnotatedToken> = Vec::new();
    let mut text_parts: Vec<String> = Vec::new();
    let mut sent_forms: Vec<String> = Vec::new();

    for line in raw.lines() {
        let line = line.trim_end();
        if line.is_empty() {
            if !current.is_empty() {
                text_parts.push(sent_forms.join(" "));
                sentences.push(std::mem::take(&mut current));
                sent_forms.clear();
            }
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        // Skip multiword and empty nodes (ranges / decimal ids).
        let cols: Vec<&str> = line.split('\t').collect();
        if cols.len() < 10 {
            return Err(IoError::Message(format!(
                "invalid CoNLL-U line (need 10 columns): {line}"
            )));
        }
        let id = cols[0];
        if id.contains('-') || id.contains('.') {
            continue;
        }
        let form = cols[1].to_string();
        let lemma = none_if_underscore(cols[2]);
        let upos = none_if_underscore(cols[3]);
        let xpos = none_if_underscore(cols[4]);
        let feats = none_if_underscore(cols[5]);
        let head = cols[6].parse::<u32>().ok().filter(|_| cols[6] != "_");
        let deprel = none_if_underscore(cols[7]);
        sent_forms.push(form.clone());
        current.push(AnnotatedToken {
            form,
            lemma,
            upos,
            xpos,
            feats,
            head,
            deprel,
        });
    }
    if !current.is_empty() {
        text_parts.push(sent_forms.join(" "));
        sentences.push(current);
    }
    Ok((sentences, text_parts.join("\n")))
}

fn none_if_underscore(s: &str) -> Option<String> {
    if s == "_" {
        None
    } else {
        Some(s.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_simple_sentence() {
        let raw =
            "1\tThe\tthe\tDET\tDT\t_\t2\tdet\t_\t_\n2\tcat\tcat\tNOUN\tNN\t_\t0\troot\t_\t_\n\n";
        let (sents, text) = parse_conllu(raw).expect("parse");
        assert_eq!(sents.len(), 1);
        assert_eq!(sents[0][0].form, "The");
        assert_eq!(sents[0][0].upos.as_deref(), Some("DET"));
        assert!(text.contains("cat"));
    }
}
