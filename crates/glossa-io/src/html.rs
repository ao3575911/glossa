use crate::error::IoResult;
use crate::txt::decode_bytes;
use crate::types::{Document, DocumentKind};
use indexmap::IndexMap;
use std::fs;
use std::path::Path;

/// Strip HTML tags to plain text (script/style removed). Preserves block breaks as newlines.
pub fn import_html(path: &Path) -> IoResult<Document> {
    let bytes = fs::read(path)?;
    let raw = decode_bytes(&bytes)?;
    let text = strip_html(&raw);
    Ok(Document {
        filename: path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string(),
        source_path: path.to_path_buf(),
        kind: DocumentKind::Html,
        text,
        metadata: IndexMap::new(),
        annotations: None,
    })
}

pub fn strip_html(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut in_tag = false;
    let mut tag_buf = String::new();
    let mut skip_depth: u32 = 0;
    let mut skip_kind: Option<&'static str> = None;

    while let Some(c) = chars.next() {
        if c == '<' {
            in_tag = true;
            tag_buf.clear();
            continue;
        }
        if in_tag {
            if c == '>' {
                in_tag = false;
                let tag = tag_buf.to_ascii_lowercase();
                let name = tag
                    .trim()
                    .trim_start_matches('/')
                    .split_whitespace()
                    .next()
                    .unwrap_or("");
                let closing = tag.trim_start().starts_with('/');
                if matches!(name, "script" | "style" | "noscript") {
                    if closing {
                        if skip_kind == Some(name) {
                            skip_depth = skip_depth.saturating_sub(1);
                            if skip_depth == 0 {
                                skip_kind = None;
                            }
                        }
                    } else if !tag.trim_end().ends_with('/') {
                        skip_kind = Some(match name {
                            "script" => "script",
                            "style" => "style",
                            _ => "noscript",
                        });
                        skip_depth += 1;
                    }
                } else if skip_depth == 0
                    && matches!(
                        name,
                        "p" | "div" | "br" | "li" | "tr" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
                    )
                {
                    out.push('\n');
                }
                continue;
            }
            tag_buf.push(c);
            continue;
        }
        if skip_depth > 0 {
            continue;
        }
        if c == '&' {
            let mut ent = String::new();
            while let Some(&n) = chars.peek() {
                if n == ';' || ent.len() > 10 {
                    break;
                }
                ent.push(n);
                chars.next();
            }
            if chars.peek() == Some(&';') {
                chars.next();
                out.push(decode_entity(&ent));
            } else {
                out.push('&');
                out.push_str(&ent);
            }
            continue;
        }
        out.push(c);
    }

    collapse_ws(&out)
}

fn decode_entity(ent: &str) -> char {
    match ent {
        "amp" => '&',
        "lt" => '<',
        "gt" => '>',
        "quot" => '"',
        "apos" => '\'',
        "nbsp" => ' ',
        _ if ent.starts_with('#') => {
            let num = &ent[1..];
            let code = if let Some(hex) = num.strip_prefix('x').or_else(|| num.strip_prefix('X')) {
                u32::from_str_radix(hex, 16).ok()
            } else {
                num.parse().ok()
            };
            code.and_then(char::from_u32).unwrap_or('�')
        }
        _ => '�',
    }
}

fn collapse_ws(s: &str) -> String {
    let mut out = String::new();
    let mut prev_nl = false;
    for line in s.lines() {
        let trimmed = line.split_whitespace().collect::<Vec<_>>().join(" ");
        if trimmed.is_empty() {
            if !prev_nl && !out.is_empty() {
                out.push('\n');
                prev_nl = true;
            }
        } else {
            if !out.is_empty() {
                out.push('\n');
            }
            out.push_str(&trimmed);
            prev_nl = false;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_tags_and_script() {
        let html = "<html><script>x()</script><p>Hello&nbsp;<b>world</b></p></html>";
        assert_eq!(strip_html(html), "Hello world");
    }
}
