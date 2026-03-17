# Backlog

Issues identified in post-review (GPT 5.4 code review, Ward 1–7).

## P1 — Before Ward 11 (Demo)

### BL-001: Split KontanthjaelpBasis i Enlig/Par
**Source:** Fix 2 (Ward 4 scope)
**Files:** `src/scenario/param_mapping.rs`, `src/scenario/scenario.rs`, `web/components/slider_panel.js`
**Problem:** `ParamId::KontanthjaelpBasis` only changes `kontanthjaelp_basis_enlig`. The name implies a general base rate, but only the single-person rate changes. The UI slider will mislead.
**Options:**
- **A (recommended):** Split into `KontanthjaelpBasisEnlig` + `KontanthjaelpBasisPar`. Update `apply_override()`, `ParamRuleMapping`, slider config, Ward 4 + Ward 6 tests.
- **B:** Introduce a "shared rate" abstraction that scales both proportionally.

### BL-002: Histogram skal bruge reelle distribution-buckets
**Source:** Fix 3 (Ward 6 scope)
**Files:** `web/components/histogram.js`, `src/batch/`, `src/wasm_api.rs`
**Problem:** `buildBuckets()` uses sine-based pseudo-visualization of aggregate totals — not a real distribution of citizen benefits. A sharp reviewer will spot it immediately.
**Fix requires:**
1. New WASM API: `get_histogram_data(rule_id, bucket_count)` returning actual frequency buckets from `BatchResult.borger_results`
2. New Rust function in `src/batch/` for bucketing amounts
3. `histogram.js` updated to consume real data

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
