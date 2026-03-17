# Backlog

Issues identified in post-review (GPT 5.4 code review, Ward 1–7).

## P1 — Before Ward 11 (Demo)

### BL-001: Split KontanthjaelpBasis i Enlig/Par — **CLOSED**
**Source:** Fix 2 (Ward 4 scope)
**Resolution:** Option A implemented. `ParamId::KontanthjaelpBasisEnlig` (id=0) + `KontanthjaelpBasisPar` (id=4). Updated `apply_override()`, `ParamRuleMapping`, `parse_param_id`, slider config (5 sliders), Ward 4/7/9 tests. Default: enlig=12.550, par=8.710.

### BL-002: Histogram real distribution buckets — **CLOSED**
**Source:** Fix 3 (Ward 6 scope)
**Resolution:** `src/batch/histogram.rs` computes real frequency buckets from `BatchResult.borger_results`. WASM API `get_histogram_data(rule_idx, bucket_count)` exposed. `histogram.js` rewritten to consume `{rule, buckets: [{min, max, count}]}`. Sine-based pseudo-visualization removed. Verified by `test_histogram_real_distribution`.

### BL-003: Panic/unwrap/expect → Result/JSON error responses in boundary layer — **CLOSED (Ward 10)**
**Source:** Fix 5 (Ward 10 scope)
**Files:** `src/wasm_api.rs`, `src/explainer/case_explainer.rs`
**Resolution:** All boundary-layer panics replaced with `json_error()` helper returning `{"error":"..."}`. `explain_case` returns `Result<CaseExplanation, String>`. Verified by `test_error_responses_no_panic`.

## P2 — Ward 9 (Performance Tuning)

### BL-004: find_by_id() O(n) → O(1)
**Source:** Fix 4 (Ward 1 scope)
**File:** `src/borger_store.rs`
**Problem:** `find_by_id()` uses `iter().position()` — O(n) linear search. With monotonically increasing `borger_id` (1..=count), it can be O(1).
**Fix:**
```rust
pub fn find_by_id(&self, borger_id: u32) -> Option<usize> {
    let idx = borger_id.checked_sub(1)? as usize;
    if idx < self.len() { Some(idx) } else { None }
}
```
