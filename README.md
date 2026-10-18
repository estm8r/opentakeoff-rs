# opentakeoff-rs

**Rust rewrite of [OpenTakeoff](https://github.com/Kentucky-ai/opentakeoff)** — native construction quantity takeoff from PDF blueprints.

Ground-up rewrite focused on performance, correctness, type safety, and a reliable offline experience.

## Current Capabilities (v0.1)

- **Geometry engine** (`geo` crate): area (SF), perimeter, holes
- **Scale model**: explicit calibration (`Scale::calibrate`), no silent assumptions
- **Measurement types**: Area, Linear (polyline), Count
- **Provenance**: method, actor, confidence, approval flag, original geometry
- **Conditions**: name, waste %, unit (SF/LF/EA)
- **JSON project persistence**: save/load full takeoffs
- **CLI**:
  - `cargo run -- demo` — shows a 10×10 ft room → 100 SF raw / 107 SF with 7% waste
  - `cargo run -- new -n "My Job" -o project.json`
  - `cargo run -- summary project.json`
  - `cargo run -- inspect-pdf plan.pdf` — text extraction + scale note detection
- **Desktop app** (egui):
  - `cargo run --bin desktop`
  - Load/save JSON projects
  - Condition selector
  - Add sample 10×10 ft area measurements
  - Live quantity view (raw + order with waste)
- PDF text extraction via `pdf-extract`

## Quick Start

```bash
git clone https://github.com/estm8r/opentakeoff-rs.git
cd opentakeoff-rs
cargo run -- demo
cargo run --bin desktop          # GUI with measurement tools
```

## Architecture

```
src/
  geometry.rs   # Polygon ops, area_sf, perimeter_lf
  scale.rs      # Scale calibration
  takeoff.rs    # Document model, quantities, JSON I/O
  pdf.rs        # Text extraction + scale detection
  main.rs       # CLI
  bin/desktop.rs # egui desktop application
```

The library is pure and can be consumed by the desktop UI or a future Rust MCP server.

## Roadmap

- [x] Core geometry + scale + provenance model
- [x] Area / Linear / Count measurements
- [x] JSON persistence + functional CLI
- [x] PDF text extraction for scale notes
- [x] Desktop UI with project management and sample measurements
- [ ] Interactive canvas with PDF rendering + vector snapping
- [ ] Symbol detection / counting
- [ ] Marked-up PDF export
- [ ] Rust MCP server for AI agents
- [ ] Materials / roll-goods calculations

## License

AGPL-3.0. Independent rewrite inspired by Kentucky-ai/opentakeoff.

## Improvements over original

- Native speed and stronger typing
- Explicit scale and provenance as first-class concepts
- Reliable offline file access
- Geometry backed by a robust crate instead of custom floating-point code
- Desktop-native UI with measurement tools
