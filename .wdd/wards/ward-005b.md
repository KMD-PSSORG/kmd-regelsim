---
ward: 5
revision: "b"
name: "WASM Bridge (fix b)"
epic: "Bridge og Render Layer"
status: "complete"
dependencies: []
layer: "rust"
estimated_tests: 1
created: "2026-03-16"
completed: "2026-03-16"
---
# Ward 005b: WASM Bridge (fix)

## Reopened from
Original: ward-005.md
Reason: get_filtered_stats() uses baseline instead of scenario results — breaks demo consistency when kommune filter is active after scenario change

## Scope
Fix `get_filtered_stats()` in `src/wasm_api.rs` to use `last_scenario_result` when available, falling back to `baseline`. Previously it always iterated over `self.baseline.borger_results`, causing the dashboard to show baseline stats even after a scenario was applied.

## Tests

| # | Test Name | Verifies |
|---|-----------|----------|
| 1 | test_filtered_stats_uses_scenario | After apply_scenario, get_filtered_stats returns scenario totals, not baseline |

## Must NOT
- Break existing Ward 5 tests
- Change public API signatures

## Must DO
- Use `self.last_scenario_result.as_ref().unwrap_or(&self.baseline)` as active result source
- Verify kontanthjælp totals differ between baseline and post-scenario filtered stats

## Verification
All 7 Ward 5 tests pass (6 original + 1 new).
