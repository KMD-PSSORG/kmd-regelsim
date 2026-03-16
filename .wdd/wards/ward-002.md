---
ward: 2
revision: null
name: "Regelmotor med Dependency Graph"
epic: "Rust Engine Foundation"
status: "complete"
dependencies: [1]
layer: "rust"
estimated_tests: 8
created: "2026-03-16"
completed: "2026-03-16"
---
# Ward 002: Regelmotor med Dependency Graph

## Scope
Kernen. Evaluerer regler i korrekt rækkefølge via dependency graph. Kompakte resultater, ingen forklaringer i batch. Fejl er Result, ikke panic.

## Inputs
- `BorgerStore` og `BorgerView` fra Ward 1

## Outputs
- `Rule` trait med evaluate-metode
- 3 regelimplementeringer (kontanthjælp, boligstøtte, børneydelse)
- `DependencyGraph` med topologisk sortering → `Result<DepGraph, EngineError>`
- `EvalContext` der akkumulerer resultater for downstream-regler
- `RuleResult { rule_id, amount: f64, eligible: bool }`
- `EngineError` enum med thiserror

## Specification

### Domæneregler
| Regel | Nøgleparametre | Afhænger af |
|-------|----------------|-------------|
| Kontanthjælp | Basissats (enlig/par), forsørgertillæg, kontanthjælpsloft | — (root) |
| Boligstøtte | Husleje, husstandsindkomst, boligareal, grænsebeløb | Kontanthjælp |
| Børne- og ungeydelse | Alderstrin (0-2, 3-6, 7-14, 15-17), indkomstaftrapning | — (root) |

### Structs
- `trait Rule { fn evaluate(&self, borger: &BorgerView, ctx: &EvalContext) -> RuleResult }`
- `DependencyGraph` — topologisk sortering, cycle detection → `Result<DepGraph, EngineError>`
- `EvalContext` — akkumulerer resultater så downstream-regler kan referere upstream
- `error.rs` — `EngineError` enum: `CyclicDependency`, `InvalidParameter`, etc. (thiserror)
- Parametre eksplicitte og konfigurerbare: `kontanthjaelp_basis_enlig: f64 = 12_550.0`

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_single_rule_eval | En regel evalueres for én borger → RuleResult |
| 2 | test_dependency_order | Boligstøtte evalueres *efter* kontanthjælp |
| 3 | test_circular_dependency_result | Cirkulære afhængigheder → `Err(EngineError::CyclicDependency { .. })` |
| 4 | test_all_rules_individual | Alle 3 regler implementeret og individuelt testede |
| 5 | test_kontanthjaelp_manual_cases | Kontanthjælp matcher 4 håndberegnede cases (enlig u/børn, enlig m/børn, par u/børn, par m/børn) |
| 6 | test_configurable_params | Parametre er eksplicitte og konfigurerbare |
| 7 | test_eval_context_accumulates | EvalContext akkumulerer resultater for downstream-regler |
| 8 | test_no_explanations_in_batch | Batch-resultater er kompakte, ingen forklaringsstrenge |

## Must NOT
- Bruge panic for fejl — alt er Result med EngineError
- Generere forklaringer (det er Ward 7, on-demand)
- Batch-evaluere (det er Ward 3)
- Inkludere scenarie-logik (det er Ward 4)

## Must DO
- Topologisk sortering af dependency graph
- Cycle detection → Result, ikke panic
- `EngineError` enum med thiserror
- Alle parametre konfigurerbare
- Compact output: (f64, bool) per regel per borger

## Verification
`cargo test` — alle 8 tests passerer. Håndberegnede cases matcher. Cycle detection returnerer Err.
