import { initEngine, getBaselineStats, applyScenario, getTopAffected, getFilteredStats } from './bridge.js';
import { renderStatsPanel } from './components/stats_panel.js';
import { renderSliderPanel } from './components/slider_panel.js';
import { renderHistogram } from './components/histogram.js';
import { renderAffectedList } from './components/affected_list.js';
import { renderSegmentTable } from './components/segment_table.js';

let activeKommuneFilter = null;

export async function boot() {
  // TODO: Ward 6 implementation
}

export function setKommuneFilter(kommuneId) {
  // TODO: Ward 6 implementation
}

export function clearKommuneFilter() {
  // TODO: Ward 6 implementation
}

boot();
