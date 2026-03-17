# Context — kmd-regelsim

## Last Updated
2026-03-16 — Post-review fixes applied (GPT 5.4 review of Ward 1–7)

## Current State
Wards 1–7 complete. Ward 5b (filtered_stats scenario bug) fixed and gold — awaiting human complete.
Backlog created with 4 items from post-review.

## Architecture Decisions Made
| Decision | Rationale | Ward |
|----------|-----------|------|
| Struct-of-arrays (egen SoA) | Cache-friendly batch-eval, lavt memory footprint. Arrow udelukket i v1. | 1 |
| borger_id: u32 stabilt ID | Stabil identifikator der overlever re-sortering, bruges i API | 1 |
| 3 regler med dependency graph | Nok til downstream-effekter uden overkompleksitet | 2 |
| Result over panic | EngineError med thiserror, cycle detection returnerer Err | 2 |
| Compact batch-resultater | size_of-verificeret layout, ingen strings i batch | 3 |
| Eksplicit param→rule mapping | Konfigurerbar, ikke if-chain. Muliggør dirty propagation. | 4 |
| JSON kun på public bridge | Simpelt, debugbart. Intern engine bruger typed Rust structs. | 5 |
| On-demand forklaringer | Dyrt — kun for 1 borger ad gangen, ikke batch | 7 |
| Kommune-filter via WASM | Click → get_filtered_stats(kommune_id), data filtreret server-side | 8 |

## Active Constraints
- Performance: incremental diff under 30ms, end-to-end under 100ms (100K)
- Memory: under 50MB for 100K
- WASM binary: under 1MB gzipped (stretch: 500KB)
- Domænemodel: 3 regler (kontanthjælp, boligstøtte, børneydelse)
- Data: 100K syntetiske borgere (500K stretch)
- Deterministisk: samme seed → identiske data for same build target
- Fejlhåndtering: Result med EngineError, aldrig panic

## Key Metrics
| Metric | Target | Measured in |
|--------|--------|-------------|
| Borgere (baseline) | 100,000 | Ward 1 |
| Borgere (stretch) | 500,000 | Ward 9 |
| Regler | 3 | Ward 2 |
| Incremental diff | <30ms | Ward 9 |
| End-to-end slider→visual | <100ms (100K) | Ward 9 |
| WASM→JS transfer | <5ms | Ward 5 |
| DOM update | <16ms | Ward 6 |

## Known Limitations / Backlog (from post-review)
| ID | Priority | Description | Target Ward |
|----|----------|-------------|-------------|
| BL-001 | P1 | Split KontanthjaelpBasis into Enlig/Par | Before Ward 11 |
| BL-002 | P1 | Histogram must use real distribution buckets from engine | Before Ward 11 |
| BL-003 | P1 | Replace panic/unwrap/expect with Result/JSON error responses | Ward 10 |
| BL-004 | P2 | find_by_id() O(n) → O(1) via direct index mapping | Ward 9 |

## What Comes Next
- Ward 5b: Awaiting human complete
- Ward 8: Geo View (on hold until user says go)
- Backlog P1 items before Ward 11
