//! Takeoff document model with full provenance.

use crate::geometry::Polygon;
use crate::scale::Scale;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Method {
    Manual,
    Agent,
    AutoDetect,
    OneClick, // when re-implemented
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub method: Method,
    pub actor: String,          // "human:estm8r" or "agent:claude"
    pub confidence: f32,        // 0.0–1.0
    pub timestamp: String,      // ISO 8601
    pub original_geometry: Option<Polygon>, // frozen machine original
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    pub id: String,
    pub condition: String,
    pub geometry: Polygon,
    pub provenance: Provenance,
    pub approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub waste_percent: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sheet {
    pub id: String,
    pub name: String,
    pub scale: Option<Scale>,
    pub measurements: Vec<Measurement>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Takeoff {
    pub project_name: String,
    pub sheets: Vec<Sheet>,
    pub conditions: HashMap<String, Condition>,
}

impl Takeoff {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            project_name: name.into(),
            ..Default::default()
        }
    }

    pub fn add_condition(&mut self, cond: Condition) {
        self.conditions.insert(cond.name.clone(), cond);
    }

    /// Raw measured area (SF) for a condition across all sheets.
    /// Waste is applied only in reports.
    pub fn raw_area_sf(&self, condition: &str) -> f64 {
        self.sheets.iter().map(|sheet| {
            let upp = sheet.scale.as_ref().map(|s| s.upp).unwrap_or(1.0);
            sheet.measurements.iter()
                .filter(|m| m.condition == condition && m.approved)
                .map(|m| crate::geometry::area_sf(&m.geometry, upp))
                .sum::<f64>()
        }).sum()
    }
}
