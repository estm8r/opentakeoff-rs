//! Explicit scale model. Never assume a scale.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ScaleError {
    #[error("invalid scale: units per foot must be positive")]
    InvalidUpp,
    #[error("calibration distance must be positive")]
    InvalidCalibration,
}

/// Scale for a sheet. `upp` = drawing units per real foot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scale {
    /// Drawing units (pixels, etc.) per real foot.
    pub upp: f64,
    /// Human-readable label, e.g. "1/4\" = 1'-0\"".
    pub label: Option<String>,
    /// Whether this was explicitly confirmed by a human.
    pub confirmed: bool,
}

impl Scale {
    pub fn from_upp(upp: f64) -> Result<Self, ScaleError> {
        if upp <= 0.0 {
            return Err(ScaleError::InvalidUpp);
        }
        Ok(Self {
            upp,
            label: None,
            confirmed: false,
        })
    }

    /// Calibrate from a known distance.
    /// `pixel_distance` is the measured distance in drawing units,
    /// `real_feet` is the real-world length in feet.
    pub fn calibrate(pixel_distance: f64, real_feet: f64) -> Result<Self, ScaleError> {
        if pixel_distance <= 0.0 || real_feet <= 0.0 {
            return Err(ScaleError::InvalidCalibration);
        }
        let upp = pixel_distance / real_feet;
        Ok(Self {
            upp,
            label: Some(format!("calibrated ({:.2} units/ft)", upp)),
            confirmed: true,
        })
    }

    /// Common architectural scales (assuming standard PDF render DPI).
    /// These are approximate starting points; always verify.
    pub fn architectural_quarter_inch() -> Self {
        // 1/4" = 1'-0" → 48 units per foot at 96 DPI? Adjust as needed for actual render.
        // Placeholder: real implementation will derive from PDF render scale.
        Self {
            upp: 48.0,
            label: Some("1/4\" = 1'-0\"".into()),
            confirmed: false,
        }
    }
}
