# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

kmd-regelsim er en browser-native konsekvensmotor til simulering af danske velfærdsydelser. En Rust/WASM engine beregner ydelser for 100K syntetiske borgere, og en vanilla JS render layer viser resultater — ingen server, ingen frameworks.

**North Star:** En bruger ændrer en parameter og forstår effekten på totaler, segmenter, geografi og én konkret case under 100ms, uden backend.

## Build & Development Commands

### WASM Build (kræver wasm-pack)
```bash
wasm-pack build --target web --out-dir pkg
```

### Rust Tests
```bash
cargo test                          # kør alle tests
cargo test ward_002                 # kør én specifik ward's tests
cargo test test_name                # kør en enkelt test
```

### Serve Web UI (kræver en lokal HTTP-server pga. WASM)
```bash
python3 -m http.server 8080         # eller tilsvarende
# Åbn http://localhost:8080/web/
```

### Web Tests
Browser-baserede tests i `web/__tests__/tests_ward*.html` — åbnes direkte i browser.

## Architecture

```
Rust Engine (src/)                    JS Render Layer (web/)
├── borger_store.rs    SoA storage    ├── bridge.js         WASM ↔ JS facade
├── borger_generator.rs  100K gen     ├── app.js            app orchestration
├── rule_engine.rs     Rule trait     ├── dashboard.js      slider+stats view
├── rules/             3 ydelser      ├── components/       UI components
│   ├── kontanthjaelp.rs              │   ├── case_panel.js
│   ├── boligstoette.rs               │   ├── kommune_kort.js (SVG)
│   └── boerne_ydelse.rs              │   ├── histogram.js
├── batch/             eval+agg       │   ├── stats_panel.js
├── scenario/          diff+incr      │   └── ...
├── explainer/         case detail    ├── data/kommune_paths.js
├── geo/               kommune agg    ├── index.html
├── wasm_api.rs        WASM bridge    └── styles.css
└── lib.rs             module root
```

### Key Design Rules
- **Rust ejer compute, JS ejer pixels** — klar grænse, ingen krydsning
- **JSON kun på WASM bridge** (`wasm_api.rs`) — aldrig i interne engine hot paths
- **Result, ikke panic** — `thiserror` til fejlhåndtering
- **Plausibel, ikke korrekt** — troværdig domænemodel, ikke juridisk autoritativ

### WASM API Surface
Al kommunikation mellem JS og Rust sker via `wasm_bindgen` funktioner i `wasm_api.rs`. Engine state lever i `thread_local!`. Alle funktioner returnerer JSON strings. Kald-flow: `wasm_init()` → `wasm_apply_scenario(param_id, value)` → `wasm_get_*()`.

### Rule System
Nye regler implementerer `Rule` trait (`rule_engine.rs`) med `id()`, `deps()`, og `evaluate()`. Regler registreres i `Engine::new()` i `wasm_api.rs`. `RuleParams` holder konfigurerbare parametre. `DependencyGraph` sikrer evalueringsrækkefølge.

### Scenario System
`scenario/` håndterer what-if analyse: `Scenario` overrider parametre, `compute_dirty_set` finder berørte regler, `incremental_evaluate` reberegner kun det nødvendige, `compute_diff` giver delta mod baseline.

## Ward-Driven Development (WDD)

Projektet bruger WDD metodologi. Wards er i `.wdd/wards/`. Tests navngives `tests/ward_NNN_*.rs` (Rust) og `web/__tests__/tests_ward*.html` (JS).

Ved ny ward: brug `/ward-new` skill. Ved fortsættelse: brug `/ward` skill. Ved session-start: brug `/wdd` skill.

## Conventions

- Rust edition 2021, minimale dependencies
- Struct-of-arrays (SoA) layout i `BorgerStore` for cache performance
- `ParamId` enum mapper til `RuleParams` felter via `param_mapping.rs`
- Kommune-ID er `u8` (0-97 for 98 kommuner)
- Web components er standalone JS-filer der eksporterer render-funktioner
