//! Simple egui desktop frontend for opentakeoff-rs.

use eframe::egui;
use opentakeoff_rs::{Takeoff, Condition};

struct App {
    takeoff: Takeoff,
    status: String,
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
        Self {
            takeoff: t,
            status: "Ready".into(),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("opentakeoff-rs");
        ui.label(format!("Project: {}", self.takeoff.project_name));
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("New Project").clicked() {
                self.takeoff = Takeoff::new("New Project");
                self.status = "Created new project".into();
            }
            if ui.button("Load JSON...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    match Takeoff::load_json(path.to_str().unwrap_or("")) {
                        Ok(t) => {
                            self.takeoff = t;
                            self.status = format!("Loaded {}", path.display());
                        }
                        Err(e) => self.status = format!("Load failed: {}", e),
                    }
                }
            }
            if ui.button("Save JSON...").clicked() {
                if let Some(path) = rfd::FileDialog::new().save_file() {
                    match self.takeoff.save_json(path.to_str().unwrap_or("")) {
                        Ok(()) => self.status = format!("Saved {}", path.display()),
                        Err(e) => self.status = format!("Save failed: {}", e),
                    }
                }
            }
        });

        ui.separator();
        ui.label("Conditions & Quantities");
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (name, cond) in &self.takeoff.conditions {
                let raw = self.takeoff.raw_quantity(name);
                let order = self.takeoff.order_quantity(name);
                ui.horizontal(|ui| {
                    ui.label(format!("{} ({})", name, cond.unit));
                    ui.label(format!("raw: {:.1}", raw));
                    ui.label(format!("order: {:.1} (+{:.0}%)", order, cond.waste_percent));
                });
            }
        });

        ui.separator();
        ui.label(&self.status);
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_title("opentakeoff-rs"),
        ..Default::default()
    };
    eframe::run_native(
        "opentakeoff-rs",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
