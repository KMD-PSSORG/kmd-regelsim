let wasm = null;

export async function initEngine() {
  // Ward 5 WASM bridge: load and initialize
  // Stub until wasm-pack build is available
  throw new Error('Ward 6 stub: bridge not implemented');
}

export function getBaselineStats() {
  return JSON.parse(wasm.wasm_get_baseline_stats());
}

export function applyScenario(paramId, value) {
  return JSON.parse(wasm.wasm_apply_scenario(paramId, value));
}

export function getTopAffected(n) {
  return JSON.parse(wasm.wasm_get_top_affected(n));
}

export function getCaseDetail(borgerId) {
  return JSON.parse(wasm.wasm_get_case_detail(borgerId));
}

export function getGeoData() {
  return JSON.parse(wasm.wasm_get_geo_data());
}

export function getFilteredStats(kommuneId) {
  return JSON.parse(wasm.wasm_get_filtered_stats(kommuneId ?? -1));
}
