use crate::error::IndexResult;
use crate::reader::CorpusIndex;
use regex::Regex;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KwicSort {
    None,
    L1,
    R1,
    R2,
}

impl KwicSort {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_ascii_uppercase().as_str() {
            "L1" => Some(Self::L1),
            "R1" => Some(Self::R1),
            "R2" => Some(Self::R2),
            "NONE" | "" => Some(Self::None),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct KwicOptions {
    pub regex: bool,
    pub sort: KwicSort,
    pub context: usize,
    pub limit: usize,
}

impl Default for KwicOptions {
    fn default() -> Self {
        Self {
            regex: false,
            sort: KwicSort::None,
            context: 5,
            limit: 100,
        }
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct KwicHit {
    pub doc: String,
    pub position: u32,
    pub left: Vec<String>,
    pub match_text: String,
    pub right: Vec<String>,
}

pub fn search_kwic(
    index: &CorpusIndex,
    query: &str,
    opts: &KwicOptions,
) -> IndexResult<Vec<KwicHit>> {
    let mut positions = if opts.regex {
        collect_regex_positions(index, query)?
    } else {
        collect_plain_positions(index, query)?
    };

    // Deterministic base order before optional secondary sort.
    positions.sort_unstable();

    let selected = select_positions(index, &positions, opts)?;
    let mut hits = Vec::with_capacity(selected.len());
    let token_count = index.token_count() as u32;
    let ctx = opts.context as u32;

    for pos in selected {
        let left_start = pos.saturating_sub(ctx);
        let right_end = (pos + 1 + ctx).min(token_count);

        let mut left = Vec::new();
        for p in left_start..pos {
            left.push(index.word_at(p)?.to_string());
        }
        let match_text = index.word_at(pos)?.to_string();
        let mut right = Vec::new();
        for p in (pos + 1)..right_end {
            right.push(index.word_at(p)?.to_string());
        }

        let doc = index.document_for_token(pos)?.filename.clone();
        hits.push(KwicHit {
            doc,
            position: pos,
            left,
            match_text,
            right,
        });
    }

    // Preserve sort key order already applied in select_positions; for None,
    // hits are in document/position order.
    Ok(hits)
}

fn select_positions(
    index: &CorpusIndex,
    positions: &[u32],
    opts: &KwicOptions,
) -> IndexResult<Vec<u32>> {
    if positions.is_empty() || opts.limit == 0 {
        return Ok(Vec::new());
    }

    match opts.sort {
        KwicSort::None => Ok(positions.iter().copied().take(opts.limit).collect()),
        sort => {
            // Sort by neighbour key first; only then materialise full context for top-N.
            let mut keyed: Vec<(String, String, u32)> = Vec::with_capacity(positions.len());
            for &pos in positions {
                let key = sort_key(index, pos, sort)?;
                let match_text = index.word_at(pos)?.to_string();
                keyed.push((key, match_text, pos));
            }
            keyed.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)).then(a.2.cmp(&b.2)));
            Ok(keyed
                .into_iter()
                .take(opts.limit)
                .map(|(_, _, pos)| pos)
                .collect())
        }
    }
}

fn sort_key(index: &CorpusIndex, pos: u32, sort: KwicSort) -> IndexResult<String> {
    let token_count = index.token_count() as u32;
    Ok(match sort {
        KwicSort::None => String::new(),
        KwicSort::L1 => {
            if pos == 0 {
                String::new()
            } else {
                index.word_at(pos - 1)?.to_string()
            }
        }
        KwicSort::R1 => {
            if pos + 1 >= token_count {
                String::new()
            } else {
                index.word_at(pos + 1)?.to_string()
            }
        }
        KwicSort::R2 => {
            if pos + 2 >= token_count {
                String::new()
            } else {
                index.word_at(pos + 2)?.to_string()
            }
        }
    })
}

fn collect_plain_positions(index: &CorpusIndex, query: &str) -> IndexResult<Vec<u32>> {
    let needle = query.to_lowercase();
    let col = index.attr("lower")?;
    let Some(id) = col.lookup_term(&needle) else {
        return Ok(Vec::new());
    };
    col.postings.positions(id)
}

fn collect_regex_positions(index: &CorpusIndex, query: &str) -> IndexResult<Vec<u32>> {
    let re = Regex::new(query)?;
    let col = index.attr("word")?;
    let mut positions = Vec::new();
    for (id, form) in index.word_forms.iter().enumerate() {
        if re.is_match(form) {
            let mut ps = col.postings.positions(id as u32)?;
            positions.append(&mut ps);
        }
    }
    positions.sort_unstable();
    positions.dedup();
    Ok(positions)
}

/// Format KWIC hits as aligned plain text (left | match | right).
pub fn format_kwic_plain(hits: &[KwicHit], context: usize) -> String {
    let left_width = hits
        .iter()
        .map(|h| h.left.join(" ").chars().count())
        .max()
        .unwrap_or(0)
        .max(context);
    let match_width = hits
        .iter()
        .map(|h| h.match_text.chars().count())
        .max()
        .unwrap_or(0);

    let mut out = String::new();
    for h in hits {
        let left = h.left.join(" ");
        let right = h.right.join(" ");
        let left_pad = format!("{left:>left_width$}");
        let match_pad = format!("{:match_width$}", h.match_text);
        out.push_str(&format!("{left_pad}  {match_pad}  {right}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build::build_corpus_index;
    use crate::reader::CorpusIndex;
    use glossa_io::{Document, DocumentKind, ImportBatch};
    use indexmap::IndexMap;
    use tempfile::tempdir;

    #[test]
    fn kwic_finds_hello() {
        let dir = tempdir().unwrap();
        let mut batch = ImportBatch::default();
        batch.documents.push(Document {
            filename: "a.txt".into(),
            source_path: "a.txt".into(),
            kind: DocumentKind::Txt,
            text: "Say hello to the world. Then hello again.".into(),
            metadata: IndexMap::new(),
            annotations: None,
        });
        build_corpus_index(&batch, dir.path()).unwrap();
        let index = CorpusIndex::open(dir.path()).unwrap();
        let hits = search_kwic(
            &index,
            "hello",
            &KwicOptions {
                limit: 10,
                ..Default::default()
            },
        )
        .unwrap();
        assert_eq!(hits.len(), 2);
        assert!(hits
            .iter()
            .all(|h| h.match_text.eq_ignore_ascii_case("hello")));
    }
}
