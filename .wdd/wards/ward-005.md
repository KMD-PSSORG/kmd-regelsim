---
ward: 5
revision: null
name: "WASM Bridge"
epic: "Bridge og Render Layer"
status: "complete"
dependencies: [1, 2, 3, 4]
layer: "rust"
estimated_tests: 6
created: "2026-03-16"
completed: "2026-03-16"
---
# Ward 005: WASM Bridge

## Scope
Broen. JS kalder Rust, får JSON tilbage. Simpelt og pragmatisk. JSON kun på public bridge — intern engine bruger typed Rust structs, aldrig JSON.

## Inputs
- Komplet Rust engine fra Ward 1-4

## Outputs
- `wasm_api.rs` med `#[wasm_bindgen]` funktioner
- `bridge.js` — ~30 linjer, loader WASM, eksponerer async API
- Kompileret WASM binary via `wasm-pack build --target web`

## Specification

### API
| Funktion | Input | Output | Bruges af |
|----------|-------|--------|-----------|
| `init()` | — | baseline stats | Alle |
| `get_baseline_stats()` | — | per-regel aggregat | Dashboard, Geo |
| `apply_scenario(param_id, value)` | param enum + f64 | DiffResult JSON | Alle |
| `get_top_affected(n)` | count | Vec<{borger_id, delta}> | Dashboard, Explainer |
| `get_case_detail(borger_id)` | borger_id: u32 | On-demand forklaring | Explainer |
| `get_geo_data()` | — | per-kommune diff | Geo |
| `get_filtered_stats(kommune_id)` | kommune_id: Option<u8> | filtreret aggregat | Dashboard (post geo-klik) |

### JSON-regel
JSON valgt i v1 for simplicity og debuggability. Kun på public bridge. Intern engine bruger typed Rust structs. Kan erstattes med binary protocol hvis transfer er målbar flaskehals.

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_wasm_exports | Alle 7 API-funktioner callable, returnerer valid JSON |
| 2 | test_init_performance | init() under 2 sekunder |
| 3 | test_apply_scenario_performance | apply_scenario() under 100ms |
| 4 | test_case_detail_performance | get_case_detail(borger_id) under 5ms |
| 5 | test_round_trip_overhead | JS→WASM→JS round-trip overhead under 2ms |
| 6 | test_wasm_binary_size | WASM binary under 1MB gzipped (stretch: 500KB) |

## Must NOT
- Bruge JSON i interne engine hot paths
- Eksponere interne Rust-typer direkte til JS
- Implementere rendering

## Must DO
- Alle API-funktioner returnerer JSON via serde
- borger_id-baseret API (ikke array-index)
- get_filtered_stats for kommune-filter support
- Round-trip overhead under 2ms
- Under 1MB gzipped WASM binary

## Verification
`wasm-pack build` kompilerer. bridge.js kalder alle funktioner. JSON valid. Round-trip under 2ms.

## Reopened — 2026-03-16
Reason: get_filtered_stats() uses baseline instead of scenario results — breaks demo consistency when kommune filter is active after scenario change
Fix Ward: ward-005b.md
