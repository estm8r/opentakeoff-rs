//! Takeoff document model with full provenance.

use crate::geometry::{Polygon, area_sf};
use crate::scale::Scale;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Method {
    Manual,
    Agent,
    AutoDetect,
    OneClick,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provenance {
    pub method: Method,
    pub actor: String,
    pub confidence: f32,
    pub timestamp: String,
    pub original_geometry: Option<Polygon>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MeasurementKind {
    Area { geometry: Polygon },
    Linear { points: Vec<(f64, f64)> },
    Count { position: (f64, f64), symbol: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    pub id: String,
    pub condition: String,
    pub kind: MeasurementKind,
    pub provenance: Provenance,
    pub approved: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub waste_percent: f64,
    pub unit: String, // "SF", "LF", "EA"
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

    pub fn add_sheet(&mut self, sheet: Sheet) {
        self.sheets.push(sheet);
    }

    /// Raw measured quantity for a condition (waste not applied).
    pub fn raw_quantity(&self, condition: &str) -> f64 {
        let Some(_cond) = self.conditions.get(condition) else { return 0.0; };
        self.sheets.iter().map(|sheet| {
            let upp = sheet.scale.as_ref().map(|s| s.upp).unwrap_or(1.0);
            sheet.measurements.iter()
                .filter(|m| m.condition == condition && m.approved)
                .map(|m| match &m.kind {
                    MeasurementKind::Area { geometry } => area_sf(geometry, upp),
                    MeasurementKind::Linear { points } => {
                        points.windows(2).map(|w| {
                            let dx = w[1].0 - w[0].0;
                            let dy = w[1].1 - w[0].1;
                            (dx*dx + dy*dy).sqrt()
                        }).sum::<f64>() / upp
                    }
                    MeasurementKind::Count { .. } => 1.0,
                })
                .sum::<f64>()
        }).sum()
    }

    pub fn order_quantity(&self, condition: &str) -> f64 {
        let raw = self.raw_quantity(condition);
        let waste = self.conditions.get(condition).map(|c| c.waste_percent).unwrap_or(0.0);
        raw * (1.0 + waste / 100.0)
    }

    pub fn save_json(&self, path: &str) -> anyhow::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_json(path: &str) -> anyhow::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&data)?)
    }
}
