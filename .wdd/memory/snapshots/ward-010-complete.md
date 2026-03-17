# Context — kmd-regelsim

## Last Updated
2026-03-17 — Ward 10 Gold (Accessibility + Error Handling)

## Current State
Wards 1–10 complete + Ward 5b (post-review fix). Ward 10 Gold — all tests passing.
11 of 13 wards complete. 54 Rust tests + 29 browser tests = 83 total.
Test HTML files in `web/__tests__/` with dynamic module imports for cache-busting.

## Architecture Decisions Made
| Decision | Rationale | Ward |
|----------|-----------|------|
| Struct-of-arrays (egen SoA) | Cache-friendly batch-eval, lavt memory footprint. Arrow udelukket i v1. | 1 |
| borger_id: u32 stabilt ID | Stabil identifikator der overlever re-sortering, bruges i API | 1 |
| find_by_id O(1) | Direct index mapping (borger_id - 1), ikke iter().position() | 9 |
| 3 regler med dependency graph | Nok til downstream-effekter uden overkompleksitet | 2 |
| Result over panic | EngineError med thiserror, cycle detection returnerer Err | 2 |
| Compact batch-resultater | size_of-verificeret layout, ingen strings i batch | 3 |
| Eksplicit param→rule mapping | Konfigurerbar, ikke if-chain. Muliggør dirty propagation. | 4 |
| JSON kun på public bridge | Simpelt, debugbart. Intern engine bruger typed Rust structs. | 5 |
| filtered_stats bruger scenarie | unwrap_or(&baseline) — viser aktive data, ikke altid baseline | 5b |
| On-demand forklaringer | Dyrt — kun for 1 borger ad gangen, ikke batch | 7 |
| Alle 98 kommuner i geo data | build_geo_entries returnerer altid 98 entries inkl. zero-delta | 8 |
| Per-capita normalisering | total_delta / population i GeoResponse | 8 |
| Kommune-filter via WASM | Click → get_filtered_stats(kommune_id), data filtreret engine-side | 8 |
| JSON error responses | Alle boundary-panics erstattet med `{"error":"..."}` (BL-003 lukket) | 10 |
| Focus trap + restore | Case panel fanger Tab, gendanner fokus ved luk | 10 |
| aria-live on stats | Screen readers annoncerer ændringer dynamisk | 10 |
| Keyboard-accessible SVG paths | tabindex=0, role=button, Enter/Space handler på kommune-paths | 10 |

## Verified Performance (Release, Ward 9)
| Metric | Target (100K) | Measured | Target (500K) | Measured |
|--------|---------------|---------|---------------|---------|
| Batch-eval | <30ms | 11.15ms | <150ms | 62.91ms |
| Incremental | <15ms | 8.18ms | <80ms | 39.34ms |
| WASM→JS transfer | <5ms | 0.89ms | — | — |
| E2E scenario cycle | <100ms | 10.66ms | — | — |
| Memory | <50MB | 8.01MB | — | — |

## Active Constraints
- Performance: all targets met 3-10x under budget (Ward 9 verified)
- Memory: 8MB for 100K (6x under 50MB budget)
- WASM binary: not yet built (Ward 11)
- Domænemodel: 3 regler (kontanthjælp, boligstøtte, børneydelse)
- Data: 100K syntetiske borgere, 500K stretch verified
- Deterministisk: seed 42 → identiske data
- Fejlhåndtering: Result med EngineError, boundary-lag bruger JSON error responses (BL-003 lukket)

## Known Limitations / Backlog
| ID | Priority | Description | Status |
|----|----------|-------------|--------|
| BL-001 | P1 | Split KontanthjaelpBasis into Enlig/Par | Open — before Ward 11 |
| BL-002 | P1 | Histogram must use real distribution buckets | Open — before Ward 11 |
| BL-003 | P1 | Replace panic/unwrap/expect with Result/JSON errors | **Closed** — Ward 10 |
| BL-004 | P2 | find_by_id() O(1) | **Closed** — Ward 9 |

## What Comes Next
- Ward 10: Awaiting human `wdd complete`
- BL-001 + BL-002: Fix before Ward 11
- Ward 11: Demo Orchestration (wasm-pack build, index.html wiring, preview)
- Ward 12: Documentation
