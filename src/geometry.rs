//! Robust geometry using the `geo` crate.

use geo::{Area, EuclideanLength, Polygon as GeoPolygon, Coord, LineString};
use geo_types::Point;
use serde::{Deserialize, Serialize};

/// A polygon defined by vertices in drawing units (pixels or calibrated).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Polygon {
    pub exterior: Vec<(f64, f64)>,
    pub holes: Vec<Vec<(f64, f64)>>,
}

impl Polygon {
    pub fn new(exterior: Vec<(f64, f64)>) -> Self {
        Self { exterior, holes: vec![] }
    }

    pub fn with_holes(exterior: Vec<(f64, f64)>, holes: Vec<Vec<(f64, f64)>>) -> Self {
        Self { exterior, holes }
    }

    fn to_geo(&self) -> GeoPolygon<f64> {
        let exterior = LineString::from(self.exterior.iter().map(|&(x, y)| Coord { x, y }).collect::<Vec<_>>());
        let holes: Vec<LineString<f64>> = self.holes.iter().map(|h| {
            LineString::from(h.iter().map(|&(x, y)| Coord { x, y }).collect::<Vec<_>>())
        }).collect();
        GeoPolygon::new(exterior, holes)
    }
}

/// Area in square feet after applying scale (drawing units per foot).
/// `upp` = units-per-foot (pixels per foot, or whatever the drawing unit is).
pub fn area_sf(poly: &Polygon, upp: f64) -> f64 {
    if upp <= 0.0 { return 0.0; }
    let geo = poly.to_geo();
    let area_units = geo.unsigned_area();
    area_units / (upp * upp)
}

/// Perimeter in linear feet.
pub fn perimeter_lf(poly: &Polygon, upp: f64) -> f64 {
    if upp <= 0.0 { return 0.0; }
    let geo = poly.to_geo();
    geo.exterior().euclidean_length() / upp
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn unit_square_area() {
        // 10x10 unit square, upp=1.0 → 100 SF
        let poly = Polygon::new(vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)]);
        assert_relative_eq!(area_sf(&poly, 1.0), 100.0, epsilon = 1e-6);
    }

    #[test]
    fn scaled_area() {
        // 144 px square at 1/4"=1' (upp = 144 px/ft for 1 ft) → 1 SF
        let poly = Polygon::new(vec![(0.0, 0.0), (144.0, 0.0), (144.0, 144.0), (0.0, 144.0)]);
        assert_relative_eq!(area_sf(&poly, 144.0), 1.0, epsilon = 1e-4);
    }
}
