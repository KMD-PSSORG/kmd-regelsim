---
ward: 3
revision: null
name: "Batch-evaluering og Kerneaggregering"
epic: "Rust Engine Foundation"
status: "complete"
dependencies: [1, 2]
layer: "rust"
estimated_tests: 7
created: "2026-03-16"
completed: "2026-03-16"
---
# Ward 003: Batch-evaluering og Kerneaggregering

## Scope
Evaluer alle 100K borgere i ét sweep. Aggreger kun features der bruges af 2+ views (rule-of-two). CompactBorgerResult layout verificeret med size_of.

## Inputs
- `BorgerStore` fra Ward 1
- Regelmotor + dependency graph fra Ward 2

## Outputs
- `BatchEvaluator` — itererer store, evaluerer regelkæde
- `Aggregator` — sum, count, mean per regel og per segment
- `TopN` — partial sort for top-N uden fuld sortering
- `BatchResult` med per-regel aggregering og kompakte borgerresultater

## Specification

### Kerne-aggregeringer (rule-of-two)
| Feature | Dashboard | Explainer | Geo | → Placering |
|---------|:---------:|:---------:|:---:|-------------|
| Batch-evaluering | ✓ | ✓ | ✓ | Kerne |
| Aggregering (sum, count, mean) | ✓ | | ✓ | Kerne |
| Segmentering (group-by) | ✓ | | ✓ | Kerne |
| Top-N ranking | ✓ | ✓ | | Kerne |

### Structs
- `BatchResult { per_rule: [RuleAggregation; 3], borger_results: Vec<CompactBorgerResult> }`
- `CompactBorgerResult` — amounts + eligible flags, layout verificeret med `std::mem::size_of` (ikke antaget)
- `RuleAggregation { total_amount, eligible_count, mean_amount }`

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_batch_eval_100k_performance | Batch-eval af 100K under 50ms (release) |
| 2 | test_core_aggregation | Total beløb per regel, antal eligible, gennemsnit |
| 3 | test_segmentation | Group-by på kommune_id og husstandstype |
| 4 | test_top_n | De N borgere med størst ydelse per regel (partial sort) |
| 5 | test_deterministic_results | Resultater er deterministiske |
| 6 | test_memory_batch_result | Batch-resultat for 100K target under 10MB (eksplicit memory accounting) |
| 7 | test_compact_borger_result_layout | `std::mem::size_of::<CompactBorgerResult>()` verificeret |

## Must NOT
- Inkludere forklaringsstrenge i batch-resultater
- Sortere hele datasættet for top-N (brug partial sort)
- Antage CompactBorgerResult layout — verificer med size_of

## Must DO
- CompactBorgerResult layout verificeret med size_of test
- Kerne-aggregeringer kun for features brugt af 2+ views
- Under 50ms for 100K i release mode
- Under 10MB memory for batch-resultater

## Verification
`cargo test` — alle 7 tests passerer. Performance <50ms. Memory <10MB. size_of verificeret.
