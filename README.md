# opentakeoff-rs

**Rust rewrite of [OpenTakeoff](https://github.com/Kentucky-ai/opentakeoff)** — a construction quantity takeoff tool for measuring materials, areas, lengths, and counts from PDF blueprints.

This is a ground-up rewrite focused on performance, correctness, type safety, and a native desktop experience. The original is a sophisticated browser-based app (React + TypeScript + pdf.js + Canvas + MCP). This project reimplements the valuable core (geometry, scale, quantities, provenance) in pure Rust and builds toward a fast, offline-capable desktop application.

## Goals & Improvements

| Area | Original (JS/TS) | opentakeoff-rs |
|------|------------------|----------------|
| Runtime | Browser (pdf.js, Canvas) | Native desktop + CLI |
| Geometry | Custom TS + floating point | `geo` crate (robust polygon ops) |
| Type safety | Good TS, but dynamic | Strong Rust types for Scale, Quantity, Provenance |
| Performance | Good for web | Native speed for large plan sets |
| Offline | Client-only but browser-dependent | Fully offline native |
| AI / MCP | TypeScript MCP server | Planned Rust MCP (rmcp) |
| File access | Browser sandbox | Direct filesystem |

### Key Design Principles

- **Provenance first**: Every measurement records scale, method (manual/agent/auto), actor, confidence, and original geometry.
- **Scale is explicit**: Never assume a scale. Per-sheet calibration is mandatory.
- **Human review**: Agent-proposed measurements stay pending until approved.
- **Waste & assemblies**: Applied only at report time, not to raw measured quantities.
- **Open data**: Export formats designed for interoperability (JSON, CSV, DXF later).

## Current Status

**Early foundation** (v0.1.0-alpha).

Implemented:
- Core geometry (polygons, area, perimeter) using the `geo` crate
- Scale model with calibration
- Takeoff document model with conditions, measurements, and provenance
- CLI skeleton (`cargo run`)

Planned next:
- PDF loading and page rendering (pdfium or similar)
- Interactive canvas (egui or Tauri + webview)
- Symbol counting / OCR
- Full MCP server
- Marked-up PDF export
- Roll-goods / materials calculations

## Building & Running

```bash
git clone https://github.com/estm8r/opentakeoff-rs.git
cd opentakeoff-rs
cargo build --release
cargo run -- --help
```

## Architecture

```
src/
  lib.rs          # Public API
  geometry.rs     # Polygon ops, area, perimeter (geo)
  scale.rs        # Scale calibration & conversion
  takeoff.rs      # Document model, measurements, provenance
  pdf.rs          # PDF page handling (stub)
  main.rs         # CLI
```

The library is designed so a desktop GUI (Tauri/egui) or MCP server can consume the same pure engine.

## License

AGPL-3.0 — same spirit as the original. See [LICENSE](LICENSE).

Inspired by the excellent work at Kentucky-ai/opentakeoff. This is an independent rewrite, not a fork.

## Contributing

This is early. Issues and PRs welcome, especially around PDF rendering, UI, and geometry edge cases (doors, openings, hatches).
