# kmd-regelsim

**Browser-native consequence simulator for rule changes in Danish public administration.**

A user changes a parameter — and instantly sees the effect on totals, segments, geography, and one concrete citizen's case. No backend. No page reload. Under 100ms for 100,000 citizens.

Built with Rust/WASM for computation and vanilla JavaScript for rendering. Zero frameworks, zero server dependencies.

## What it does

kmd-regelsim simulates the financial consequences of adjusting welfare rule parameters across a synthetic population of 100,000 Danish citizens. It demonstrates how Rust/WASM can power complex, real-time calculations entirely in the browser.

**Three welfare rules are modeled:**

| Rule | Key parameters | Dependencies |
|------|---------------|-------------|
| **Kontanthjælp** (cash benefit) | Base rate (single/couple), child supplement, ceiling | — (root) |
| **Boligstøtte** (housing support) | Rent, household income, area, income ceiling | Kontanthjælp |
| **Børneydelse** (child benefit) | Age tiers (0-2, 3-6, 7-14, 15-17), income tapering | — (root) |

> **Disclaimer:** This is a technology demo with a plausible domain model, not an authoritative benefits system. The calculations are simplified and inspired by Danish rules but are neither legally correct nor administratively authoritative.

## Architecture

```
Browser (no server)
├── JS Render Layer (vanilla JS + SVG)
│   ├── Dashboard — sliders, stats panel, histogram, diff overlay
│   ├── Case Explainer — citizen drilldown, rule chain, threshold warnings
│   └── Geo View — municipality heatmap (Ward 8)
│
└── kmd-engine (Rust → WASM)
    ├── Rule engine + dependency graph
    ├── Batch evaluation + core aggregation
    ├── Scenario diff + incremental recompute
    ├── Case explainer + threshold detection
    ├── WASM API bridge (JSON on public surface only)
    └── BorgerStore (struct-of-arrays, 100K citizens)
```

**Key design principles:**

- **Rust owns compute, JS owns pixels.** Clear boundary, no crossing.
- **Rule of Two.** Nothing enters the engine core unless at least 2 views require it.
- **JSON only on the bridge.** Internal engine uses typed Rust structs. JSON never in hot paths.
- **Plausible, not correct.** Realistic enough for a convincing demo, explicitly not legally authoritative.
- **Result over panic.** All error paths return `Result<T, EngineError>`, never panic.

## Performance

Measured on Apple Silicon, release mode, 100,000 synthetic citizens:

| Operation | Time | Target |
|-----------|------|--------|
| Batch evaluation (100K, 3 rules) | 10.2ms | <50ms |
| Scenario → diff (incremental) | 9.2ms | <100ms |
| Case detail (single citizen) | <1ms | <5ms |
| JSON round-trip (ser+de) | <1ms | <2ms |
| Memory: BorgerStore (100K) | ~27MB | <50MB |
| Memory: BatchResult (100K) | 3.05MB | <10MB |

## Project status

7 of 12 wards complete. Built using [Ward-Driven Development](https://github.com/Biscuit-Consortium/wdd) (WDD).

| Ward | Name | Tests | Status |
|------|------|-------|--------|
| 1 | Column-based data structure + synthetic data | 8 | ✅ Complete |
| 2 | Rule engine with dependency graph | 8 | ✅ Complete |
| 3 | Batch evaluation and core aggregation | 7 | ✅ Complete |
| 4 | Scenario engine with incremental recompute and diff | 9 | ✅ Complete |
| 5 | WASM bridge | 6 | ✅ Complete |
| 6 | Dashboard view | 10 | ✅ Complete |
| 7 | Case explainer view | 9 | ✅ Complete |
| 8 | Geo view (municipality map) | 9 | 📋 Planned |
| 9 | Performance tuning + 500K stretch | 9 | 📋 Planned |
| 10 | Accessibility and error handling | 7 | 📋 Planned |
| 11 | Demo orchestration | 7 | 📋 Planned |
| 12 | Documentation and video | 4 | 📋 Planned |

**57 tests passing** (43 Rust + 14 browser).

## Codebase

| Layer | Lines | Description |
|-------|-------|-------------|
| Rust engine | ~2,100 | Rule engine, batch eval, scenario diff, explainer, WASM API |
| Rust tests | ~1,300 | 43 tests across 6 test files |
| JavaScript | ~580 | Dashboard, case panel, histogram, sliders, rule graph |
| CSS | ~315 | Dark theme, responsive grid, CSS custom properties |
| Browser tests | ~380 | 14 tests across 2 test runners |

## Building

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) (for WASM compilation)

### Run Rust tests

```bash
cargo test
```

### Run in release mode (benchmarks)

```bash
cargo test --release -- --nocapture
```

### Build WASM

```bash
wasm-pack build --target web --release
```

### Run browser tests

```bash
cd web && python3 -m http.server 8080
# Open http://localhost:8080/tests.html
# Open http://localhost:8080/tests_ward7.html
```

## Technology stack

| Component | Technology | Purpose |
|-----------|-----------|---------|
| Engine | Rust | Rule evaluation, batch processing, scenario diff |
| Data storage | Struct-of-Arrays (custom) | Cache-friendly columnar access for 100K citizens |
| Random generation | `rand`, `rand_chacha`, `rand_distr` | Deterministic synthetic data from seed |
| Error handling | `thiserror` | Structured `Result<T, EngineError>` |
| Serialization | `serde`, `serde_json` | JSON on WASM bridge surface only |
| WASM binding | `wasm-bindgen` | Rust ↔ JS interop |
| Rendering | Vanilla JS + SVG | DOM manipulation, no framework |
| Styling | CSS custom properties | Dark theme, responsive, `prefers-reduced-motion` |

## How it works

1. **Engine initialization:** Generate 100K synthetic citizens with realistic demographic distributions (age, income, household type, municipality). Evaluate all 3 rules for every citizen. ~500ms cold start.

2. **Slider interaction:** User adjusts a parameter (e.g., cash benefit base rate +500 kr).

3. **Dirty propagation:** The explicit `param → rule` mapping identifies affected root rules. The dependency graph propagates downstream (e.g., Kontanthjælp → Boligstøtte).

4. **Incremental recompute:** Only dirty rules are re-evaluated. Clean rules reuse baseline results. ~9ms for 100K citizens.

5. **Diff computation:** Compare baseline vs. scenario per citizen. Aggregate per rule, per municipality. Find top-N most affected citizens by |delta|. O(n) partial sort.

6. **Case explainer:** Click any citizen → on-demand evaluation with human-readable Danish explanations and threshold proximity warnings (configurable, default 3%).

## Author

Dennis Ejby Schmock — built at KMD.

## License

MIT
