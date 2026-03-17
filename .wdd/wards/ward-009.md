---
ward: 9
revision: null
name: "Performance Tuning og 500K Stretch"
epic: "Geo View og Performance"
status: "complete"
dependencies: [1, 2, 3, 4, 5]
layer: "rust"
estimated_tests: 9
created: "2026-03-16"
completed: "2026-03-17"
---
# Ward 009: Performance Tuning og 500K Stretch

## Scope
Fra "det virker" til "det er absurd hurtigt." Faktiske benchmarks, ikke estimater. SIMD kun hvis profiling viser behov.

## Inputs
- Komplet Rust engine + WASM bridge fra Ward 1-5
- Fungerende views fra Ward 6-8

## Outputs
- Optimerede hot loops (SIMD kun hvis profiling viser flaskehals)
- Verificeret SoA layout (profiler for AoS-regression)
- wasm-opt -O3 med strip
- Benchmark-resultater for 100K og 500K (faktiske tal)

## Specification

### Performance-targets
| Metric | 100K | 500K (stretch) |
|--------|------|-----------------|
| Batch-eval | <30ms | <150ms |
| Incremental scenarie | <15ms | <80ms |
| WASM→JS transfer | <5ms | <5ms |
| DOM update | <16ms | <16ms |
| End-to-end slider→visual | <100ms | <200ms |
| Memory | <50MB | <250MB |
| WASM binary | <1MB gzipped | <500KB gzipped (stretch) |

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_batch_100k_30ms | 100K batch-eval under 30ms (release) |
| 2 | test_incremental_100k_15ms | 100K incremental under 15ms |
| 3 | test_batch_500k_150ms | 500K batch-eval under 150ms (stretch) |
| 4 | test_incremental_500k_80ms | 500K incremental under 80ms (stretch) |
| 5 | test_wasm_js_transfer_5ms | WASM→JS transfer under 5ms |
| 6 | test_dom_update_16ms | DOM update under 16ms (60fps) |
| 7 | test_e2e_100k_100ms | End-to-end slider→visual under 100ms for 100K |
| 8 | test_memory_100k | Memory under 50MB for 100K |
| 9 | test_wasm_binary_size | WASM binary under 1MB gzipped (stretch: 500KB) |

## Must NOT
- Bruge SIMD som ritual — kun hvis profiling viser det er flaskehalsen
- Regresse funktionalitet
- Bruge unsafe Rust medmindre profiling viser det er nødvendigt
- Rapportere estimater som benchmarks

## Must DO
- Profile hot paths med browser devtools FØR optimering
- Verificer struct-of-arrays (profiler for AoS-regression)
- wasm-opt -O3 med strip
- Faktiske, målte benchmark-tal
- Compact JSON: kun ændrede data ved incremental

## Verification
Alle performance-targets opnået og dokumenteret med faktiske tal. Tal der kan stå i en præsentation.
