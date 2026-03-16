---
ward: 4
revision: null
name: "Scenarie-engine med Incremental Recompute og Diff"
epic: "Rust Engine Foundation"
status: "complete"
dependencies: [1, 2, 3]
layer: "rust"
estimated_tests: 9
created: "2026-03-16"
completed: "2026-03-16"
---
# Ward 004: Scenarie-engine med Incremental Recompute og Diff

## Scope
Simulatorens hjerte. Ændr en parameter → se forskellen. Eksplicit param→rule mapping. Dirty propagation via dependency graph.

## Inputs
- `BorgerStore` fra Ward 1
- Regelmotor + dependency graph fra Ward 2
- `BatchEvaluator` + `BatchResult` fra Ward 3

## Outputs
- `Scenario` — parameter overrides med eksplicit `param_id → Vec<RuleId>` mapping
- `IncrementalEvaluator` — dirty flags propageret via dependency graph
- `DiffResult` — per-regel aggregat-diff, per-segment diff, top-N med borger_id

## Specification

### Parameter→regel mapping (eksplicit, ikke if-chain)
| Slider-parameter | Affected root rules | Dirty propagation |
|------------------|--------------------|--------------------|
| Kontanthjælp basissats | Kontanthjælp | → Boligstøtte (downstream) |
| Forsørgertillæg | Kontanthjælp | → Boligstøtte (downstream) |
| Boligstøtte grænsebeløb | Boligstøtte | (ingen downstream) |
| Børneydelse aftrapningsgrænse | Børneydelse | (ingen downstream) |

### Scenarie-flow
1. Bruger ændrer parameter (fx kontanthjælp basissats +500 kr)
2. `param_id → Vec<RuleId>` lookup finder affected root rules
3. Dirty propagation via dependency graph finder downstream-regler
4. Kun dirty regler re-evalueres; unchanged rules reuse baseline results
5. Diff: sammenlign baseline vs. scenarie

### Structs
- `Scenario` — parameter overrides med eksplicit mapping
- `IncrementalEvaluator` — dirty flags, kun downstream re-eval
- `DiffResult { per_rule_diff, per_segment_diff, top_n_affected }` — top-N med borger_id

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_create_scenario | Kan oprette scenarie: kontanthjælp basissats +500 kr |
| 2 | test_param_rule_mapping | Parameter→root-rule mapping er eksplicit konfigureret |
| 3 | test_dirty_propagation | Dirty propagation følger dependency edges fra affected root rules |
| 4 | test_only_dirty_reevaluated | Ændring af kontanthjælp re-evaluerer kontanthjælp + boligstøtte, IKKE børneydelse |
| 5 | test_incremental_speedup | Incremental recompute mindst 2x hurtigere end fuld re-eval |
| 6 | test_diff_eligibility | Diff: antal borgere der ændrer eligibility |
| 7 | test_diff_per_segment | Diff per segment: kommune → merudgift |
| 8 | test_top_n_with_borger_id | Top-N mest påvirkede med borger_id (|delta| ranked) |
| 9 | test_end_to_end_100ms | Parameter-ændring → diff-resultat under 100ms for 100K |

## Must NOT
- Bruge hardcoded if-chains for param→rule mapping
- Re-evaluere regler der ikke er downstream af ændringen
- Kopiere hele BorgerStore for scenarie (copy-on-write)

## Must DO
- Eksplicit `param_id → Vec<RuleId>` mapping (konfigurerbar)
- Dirty propagation via dependency graph edges
- Top-N returneret med borger_id
- End-to-end under 100ms for 100K borgere

## Verification
`cargo test` — alle 9 tests passerer. Incremental 2x+. End-to-end under 100ms.
