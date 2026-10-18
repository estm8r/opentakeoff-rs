use clap::{Parser, Subcommand};
use opentakeoff_rs::{Takeoff, Condition, Sheet, Measurement, MeasurementKind, Provenance, Method, Scale};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "opentakeoff-rs", about = "Native construction takeoff from blueprints")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new empty project
    New {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        output: PathBuf,
    },
    /// Show summary of a project
    Summary {
        project: PathBuf,
    },
    /// Extract text and try to detect scale from a PDF
    InspectPdf {
        pdf: PathBuf,
    },
    /// Demo: create a sample project with a calibrated area
    Demo,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { name, output } => {
            let mut t = Takeoff::new(&name);
            t.add_condition(Condition {
                name: "Floor".into(),
                waste_percent: 5.0,
                unit: "SF".into(),
                notes: None,
            });
            t.save_json(output.to_str().unwrap())?;
            println!("Created project '{}' at {}", name, output.display());
        }
        Commands::Summary { project } => {
            let t = Takeoff::load_json(project.to_str().unwrap())?;
            println!("Project: {}", t.project_name);
            println!("Sheets: {}", t.sheets.len());
            println!("Conditions:");
            for (name, cond) in &t.conditions {
                let raw = t.raw_quantity(name);
                let order = t.order_quantity(name);
                println!("  {} ({}): raw={:.2}  order={:.2} (waste {}%)",
                    name, cond.unit, raw, order, cond.waste_percent);
            }
        }
        Commands::InspectPdf { pdf } => {
            let text = opentakeoff_rs::pdf::extract_text(pdf.to_str().unwrap())?;
            println!("Extracted {} characters", text.len());
            if let Some(scale) = opentakeoff_rs::pdf::detect_scale_note(&text) {
                println!("Detected possible scale: {}", scale);
            } else {
                println!("No common scale note found");
            }
            let preview: String = text.chars().take(500).collect();
            println!("\nPreview:\n{}", preview);
        }
        Commands::Demo => {
            let mut t = Takeoff::new("Demo Takeoff");
            t.add_condition(Condition {
                name: "CPT-1".into(),
                waste_percent: 7.0,
                unit: "SF".into(),
                notes: Some("Carpet tile".into()),
            });

            let scale = Scale::calibrate(144.0, 1.0)?; // 144 px = 1 ft
            let mut sheet = Sheet {
                id: "A-101".into(),
                name: "Floor Plan".into(),
                scale: Some(scale),
                measurements: vec![],
            };

            // 10x10 ft room (1440 x 1440 px at that scale)
            sheet.measurements.push(Measurement {
                id: "m1".into(),
                condition: "CPT-1".into(),
                kind: MeasurementKind::Area {
                    geometry: opentakeoff_rs::Polygon::new(vec![
                        (0.0, 0.0), (1440.0, 0.0), (1440.0, 1440.0), (0.0, 1440.0)
                    ]),
                },
                provenance: Provenance {
                    method: Method::Manual,
                    actor: "human:demo".into(),
                    confidence: 1.0,
                    timestamp: "2026-10-18T00:00:00Z".into(),
                    original_geometry: None,
                },
                approved: true,
            });

            t.add_sheet(sheet);
            println!("Demo project created in memory");
            println!("CPT-1 raw: {:.1} SF", t.raw_quantity("CPT-1"));
            println!("CPT-1 order (7% waste): {:.1} SF", t.order_quantity("CPT-1"));
        }
    }

    Ok(())
}
