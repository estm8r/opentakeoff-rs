//! opentakeoff-rs — core takeoff engine.
//!
//! Pure Rust geometry, scale, and quantity model inspired by OpenTakeoff.
//! Designed for a native desktop application.

pub mod geometry;
pub mod scale;
pub mod takeoff;
pub mod pdf;

pub use geometry::{area_sf, perimeter_lf, Polygon};
pub use scale::Scale;
pub use takeoff::{Condition, Measurement, MeasurementKind, Provenance, Takeoff, Method, Sheet};
