use clap::Parser;
use opentakeoff_rs::Takeoff;

#[derive(Parser, Debug)]
#[command(name = "opentakeoff-rs", about = "Native construction takeoff from blueprints")]
struct Cli {
    /// Path to a takeoff project file (JSON, future)
    #[arg(short, long)]
    project: Option<String>,

    /// Print version and exit
    #[arg(short, long)]
    version: bool,
}

fn main() {
    let cli = Cli::parse();

    if cli.version {
        println!("opentakeoff-rs {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let mut takeoff = Takeoff::new("Sample Project");
    takeoff.add_condition(opentakeoff_rs::Condition {
        name: "CPT-1".into(),
        waste_percent: 7.0,
        notes: Some("Carpet tile".into()),
    });

    println!("opentakeoff-rs foundation loaded.");
    println!("Project: {}", takeoff.project_name);
    println!("Conditions: {}", takeoff.conditions.len());
    println!("\nThis is the early Rust rewrite of OpenTakeoff.");
    println!("Core geometry, scale, and provenance model are in place.");
    println!("Next: PDF rendering, interactive canvas, MCP server.");
}
