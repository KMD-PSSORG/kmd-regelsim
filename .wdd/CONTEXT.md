# Context — kmd-regelsim

## Last Updated
2026-03-16 — v3 spec loaded, Ward 1 gold

## Current State
Ward 1 (Kolonnebaseret Datastruktur + Syntetisk Data) er i gold — 7/7 tests grønne. Skal opdateres med borger_id fra v3 spec.

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

## Known Limitations
_None yet_

## What Comes Next
- Ward 1: Tilføj borger_id, ekstra test (v3 opdatering)
- Ward 2: Regelmotor med Dependency Graph
