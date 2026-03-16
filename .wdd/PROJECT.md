# kmd-regelsim

## Identity
- **Name:** kmd-regelsim
- **One-liner:** Browser-native konsekvenssimulator for regelændringer i dansk offentlig forvaltning
- **License:** MIT
- **Author:** Dennis Ejby Schmock (KMD)
- **Methodology:** Ward-Driven Development (WDD) med GS-TDD enforcement
- **Version:** v3.0

## North Star
> A user can change a parameter and understand the effect on totals, segments, geography, and one concrete case in under 100ms, without a backend.

## Architecture Overview
Rust/WASM engine beregner ydelser for 100K syntetiske borgere. Vanilla JS render layer viser resultater i 3 views (Dashboard, Case Explainer, Geo View). Ingen server, ingen frameworks, ingen data forlader browseren.

```
Browser
├── JS Render Layer (vanilla JS + SVG)
│   ├── Dashboard (sliders, stats, histogram, diff)
│   ├── Case Explainer (borger-drilldown, regelkæde)
│   └── Geo View (kommunekort, heatmap)
│
└── kmd-engine (Rust → WASM)
    ├── Regelmotor + dependency graph
    ├── Batch eval + kerneaggregering
    ├── Scenarie-diff + incremental recompute
    ├── WASM API bridge (JSON on public surface only)
    └── Borger Store (struct-of-arrays, 100K)
```

## Principles
1. **North Star Test:** Bidrager dette til slider→forståelse under 100ms? Hvis nej → ikke i v1.
2. **Rule of Two:** Intet i kernen medmindre 2+ views bruger det
3. **Rust ejer compute, JS ejer pixels:** Klar grænse, ingen krydsning
4. **Plausibel, ikke korrekt:** Troværdig domænemodel, ikke juridisk autoritativ
5. **Dependencies:** Små, velforståede crates. Ingen tunge frameworks, async, eller kitchen-sink utils.
6. **JSON-reglen:** JSON kun på public WASM bridge. Aldrig i interne engine hot paths.

## Technology Stack
- **Rust:** Engine, regelmotor, batch-eval, data-generering
- **wasm-pack:** Rust → WASM compilation
- **serde/serde_json:** Serialisering (kun bridge)
- **thiserror:** Fejlhåndtering (Result, ikke panic)
- **Vanilla JS:** Render layer, DOM, SVG, events
- **CSS:** Custom properties, responsive grid, mørkt tema

## Non-Goals (v1)
- No general rule DSL or plugin system
- No persistence, authentication, or backend
- No attempt at legal correctness
- No generic charting or UI component library
- No config-driven rule loading (regler er Rust structs)
- No Arrow, Parquet, or external data formats
- No multi-language support beyond da/en labels
