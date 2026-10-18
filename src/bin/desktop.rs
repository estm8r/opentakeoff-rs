//! egui desktop frontend for opentakeoff-rs with basic measurement tools.

use eframe::egui;
use opentakeoff_rs::{
    Takeoff, Condition, Sheet, Measurement, MeasurementKind, Provenance, Method, Scale, Polygon,
};

struct App {
    takeoff: Takeoff,
    status: String,
    selected_condition: String,
}

impl Default for App {
    fn default() -> Self {
        let mut t = Takeoff::new("Untitled Project");
        t.add_condition(Condition {
            name: "Floor".into(),
            waste_percent: 5.0,
            unit: "SF".into(),
            notes: None,
        });
        t.add_condition(Condition {
            name: "Wall".into(),
            waste_percent: 10.0,
            unit: "LF".into(),
            notes: None,
        });
        Self {
            takeoff: t,
            status: "Ready — add sheets and measurements".into(),
            selected_condition: "Floor".into(),
        }
    }
}

impl App {
    fn add_sample_area(&mut self) {
        let scale = Scale::calibrate(144.0, 1.0).unwrap();
        let mut sheet = Sheet {
            id: format!("S{}", self.takeoff.sheets.len() + 1),
            name: "Sample Sheet".into(),
            scale: Some(scale),
            measurements: vec![],
        };
        sheet.measurements.push(Measurement {
            id: format!("m{}", sheet.measurements.len() + 1),
            condition: self.selected_condition.clone(),
            kind: MeasurementKind::Area {
                geometry: Polygon::new(vec![
                    (0.0, 0.0), (1440.0, 0.0), (1440.0, 1440.0), (0.0, 1440.0),
                ]),
            },
            provenance: Provenance {
                method: Method::Manual,
                actor: "human:desktop".into(),
                confidence: 1.0,
                timestamp: "2026-10-18T00:00:00Z".into(),
                original_geometry: None,
            },
            approved: true,
        });
        self.takeoff.add_sheet(sheet);
        self.status = format!("Added 10×10 ft area to {}", self.selected_condition);
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("opentakeoff-rs");
        ui.label(format!("Project: {}", self.takeoff.project_name));
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("New").clicked() {
                *self = App::default();
                self.status = "New project".into();
            }
            if ui.button("Load...").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
                    match Takeoff::load_json(path.to_str().unwrap_or("")) {
                        Ok(t) => {
                            self.takeoff = t;
                            self.status = format!("Loaded {}", path.display());
                        }
                        Err(e) => self.status = format!("Load failed: {}", e),
                    }
                }
            }
            if ui.button("Save...").clicked() {
                if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).save_file() {
                    match self.takeoff.save_json(path.to_str().unwrap_or("")) {
                        Ok(()) => self.status = format!("Saved {}", path.display()),
                        Err(e) => self.status = format!("Save failed: {}", e),
                    }
                }
            }
        });

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Condition:");
            egui::ComboBox::from_label("")
                .selected_text(&self.selected_condition)
                .show_ui(ui, |ui| {
                    for name in self.takeoff.conditions.keys() {
                        ui.selectable_value(&mut self.selected_condition, name.clone(), name);
                    }
                });
            if ui.button("Add 10×10 ft Area").clicked() {
                self.add_sample_area();
            }
        });

        ui.separator();
        ui.label("Quantities");
        egui::ScrollArea::vertical().max_height(200.0).show(ui, |ui| {
            for (name, cond) in &self.takeoff.conditions {
                let raw = self.takeoff.raw_quantity(name);
                let order = self.takeoff.order_quantity(name);
                ui.horizontal(|ui| {
                    ui.strong(name);
                    ui.label(format!("({})", cond.unit));
                    ui.label(format!("raw {:.1}", raw));
                    ui.label(format!("order {:.1} (+{:.0}%)", order, cond.waste_percent));
                });
            }
        });

        ui.separator();
        ui.label(format!("Sheets: {}", self.takeoff.sheets.len()));
        ui.label(&self.status);
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 640.0])
            .with_title("opentakeoff-rs"),
        ..Default::default()
    };
    eframe::run_native(
        "opentakeoff-rs",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
