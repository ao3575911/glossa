use crate::error::IoResult;
use indexmap::IndexMap;
use std::fs::File;
use std::path::Path;

/// Load optional metadata CSV keyed by filename (column `filename` or `file`).
pub fn load_metadata_csv(
    path: &Path,
) -> IoResult<IndexMap<String, IndexMap<String, serde_json::Value>>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let headers = rdr.headers()?.clone();
    let mut out: IndexMap<String, IndexMap<String, serde_json::Value>> = IndexMap::new();

    for result in rdr.records() {
        let record = result?;
        let mut map = IndexMap::new();
        let mut filename: Option<String> = None;
        for (i, header) in headers.iter().enumerate() {
            let value = record.get(i).unwrap_or("").to_string();
            let key = header.to_string();
            if key.eq_ignore_ascii_case("filename") || key.eq_ignore_ascii_case("file") {
                filename = Some(value.clone());
            }
            map.insert(key, serde_json::Value::String(value));
        }
        if let Some(name) = filename {
            // Prefer basename if a path was given.
            let base = Path::new(&name)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or(&name)
                .to_string();
            out.insert(base, map);
        }
    }
    Ok(out)
}
