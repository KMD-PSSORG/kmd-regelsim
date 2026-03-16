# Epic 01: Rust Engine Foundation

## Goal
Byg den komplette Rust-engine: kolonnebaseret datastruktur, regelmotor med dependency graph, batch-evaluering med aggregering, og scenarie-diff med incremental recompute. Alt compute-logik lever her.

## Wards
| Ward | Name | Status |
|------|------|--------|
| 1 | Kolonnebaseret Datastruktur + Syntetisk Data | planned |
| 2 | Regelmotor med Dependency Graph | planned |
| 3 | Batch-evaluering og Kerneaggregering | planned |
| 4 | Scenarie-engine med Incremental Recompute og Diff | planned |

## Integration Points
- Ward 5 (WASM Bridge) eksponerer engine til JS — afhænger af at Epic 1 er komplet
- Engine producerer alle data som JS-laget konsumerer

## Completion Criteria
- 100K borgere i hukommelsen, 3 regler evalueret, scenarie-diff under 100ms
- "Ændr kontanthjælp +500 kr → 12.400 borgere påvirkes, total merudgift 6.2M kr/md" — under 100ms
