//! PDF handling (stub).
//!
//! Full implementation will use pdfium-render or similar for page rendering
//! and vector extraction. For now, this documents the intended interface.

/// Placeholder for loading a PDF plan set.
pub struct PdfPlanSet {
    // path, page count, etc.
}

impl PdfPlanSet {
    pub fn open(_path: &str) -> anyhow::Result<Self> {
        // TODO: implement with pdfium or lopdf + renderer
        anyhow::bail!("PDF loading not yet implemented — foundation only")
    }
}
