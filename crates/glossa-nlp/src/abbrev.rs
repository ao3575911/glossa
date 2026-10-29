/// English abbreviations that should not trigger a sentence boundary when followed by `.`.
/// Stored lowercase without the trailing period.
pub static ENGLISH_ABBREVIATIONS: &[&str] = &[
    "mr", "mrs", "ms", "dr", "prof", "sr", "jr", "st", "mt", "ft", "vs", "etc", "approx", "dept",
    "univ", "assn", "bros", "co", "corp", "inc", "ltd", "no", "nos", "vol", "vols", "fig", "eq",
    "ed", "eds", "rev", "gen", "gov", "sen", "rep", "sgt", "capt", "col", "lt", "cmdr", "adm",
    "ave", "blvd", "rd", "ln", "jan", "feb", "mar", "apr", "jun", "jul", "aug", "sep", "sept",
    "oct", "nov", "dec", "mon", "tue", "tues", "wed", "thu", "thur", "thurs", "fri", "sat", "sun",
    "a.m", "p.m", "am", "pm", "e.g", "i.e", "cf", "al", "approx",
];

pub fn is_abbreviation(word: &str) -> bool {
    let lower = word.trim_end_matches('.').to_ascii_lowercase();
    ENGLISH_ABBREVIATIONS.binary_search(&lower.as_str()).is_ok()
        || ENGLISH_ABBREVIATIONS.iter().any(|a| *a == lower)
}
