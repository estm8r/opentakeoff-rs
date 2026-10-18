//! PDF text extraction for scale detection and plan metadata.

use anyhow::Result;

/// Extract text from all pages of a PDF.
pub fn extract_text(path: &str) -> Result<String> {
    let bytes = std::fs::read(path)?;
    let text = pdf_extract::extract_text_from_mem(&bytes)?;
    Ok(text)
}

/// Attempt to detect a common architectural scale note.
pub fn detect_scale_note(text: &str) -> Option<String> {
    let lower = text.to_lowercase();
    for candidate in [
        r#"1/4\" = 1'-0\""#,
        r#"1/8\" = 1'-0\""#,
        r#"1/2\" = 1'-0\""#,
        r#"3/16\" = 1'-0\""#,
        "1:50",
        "1:100",
        "scale:",
    ] {
        if lower.contains(&candidate.to_lowercase()) {
            return Some(candidate.to_string());
        }
    }
    None
}
