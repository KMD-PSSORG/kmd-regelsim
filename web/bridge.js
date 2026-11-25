import init, {
  wasm_init,
  wasm_get_baseline_stats,
  wasm_apply_scenario,
  wasm_get_top_affected,
  wasm_get_filtered_stats,
  wasm_get_geo_data,
  wasm_get_histogram_data,
  wasm_get_case_detail,
} from '../pkg/kmd_engine.js';

export async function createEngine() {
  await init();
  wasm_init();

  function tryParse(fn) {
    const raw = fn();
    const parsed = JSON.parse(raw);
    return parsed.error ? null : parsed;
  }

  const engine = {
    baseline: JSON.parse(wasm_get_baseline_stats()),
    diff: null,
    topAffected: null,
    histogramData: JSON.parse(wasm_get_histogram_data(0, 20)),
    geoData: null,
    caseDetail: null,

    applyScenario(paramId, value) {
      const result = JSON.parse(wasm_apply_scenario(paramId, value));
      if (result.error) return result;
      engine.diff = result;
      engine.topAffected = tryParse(() => wasm_get_top_affected(10));
      engine.histogramData = tryParse(() => wasm_get_histogram_data(0, 20));
      engine.geoData = tryParse(() => wasm_get_geo_data());
      return result;
    },

    getFilteredStats(kommuneId) {
      return JSON.parse(wasm_get_filtered_stats(kommuneId ?? -1));
    },

    getCaseDetail(borgerId) {
      const detail = JSON.parse(wasm_get_case_detail(borgerId));
      if (!detail.error) engine.caseDetail = detail;
      return detail;
    },
  };

  return engine;
}
