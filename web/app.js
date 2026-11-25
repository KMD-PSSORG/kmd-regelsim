import { renderSliderPanel } from './components/slider_panel.js';
import { renderStatsPanel } from './components/stats_panel.js';
import { renderAffectedList } from './components/affected_list.js';
import { renderHistogram } from './components/histogram.js';
import { createKommuneKort } from './components/kommune_kort.js';
import { createCasePanel } from './components/case_panel.js';

function clearChildren(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
}

function create(tag, className, attrs) {
  const e = document.createElement(tag);
  if (className) e.className = className;
  if (attrs) for (const [k, v] of Object.entries(attrs)) e.setAttribute(k, v);
  return e;
}

export function createApp(container, engine) {
  clearChildren(container);

  // ── State ──
  let currentView = 0;
  let activeKommune = null;
  let scenarioDiff = engine.diff;
  let started = false;
  const perfTimings = { lastEval: 0, count: engine.baseline?.[0]?.eligible || 0 };

  // ── Intro overlay ──
  const intro = create('div', 'intro-overlay');
  const introContent = create('div', 'intro-content');

  const introTitle = create('h1', 'intro-title');
  introTitle.textContent = 'kmd-regelsim';

  const introSub = create('p', 'intro-subtitle');
  introSub.textContent = 'Browser-native konsekvensmotor for regelændringer i dansk offentlig forvaltning';

  const disclaimer = create('p', 'intro-disclaimer');
  disclaimer.textContent = 'Simulerede data — ikke juridisk bindende. 100.000 syntetiske borgere, deterministisk seed.';

  const startBtn = create('button', 'intro-start');
  startBtn.textContent = 'Start simulering';

  introContent.appendChild(introTitle);
  introContent.appendChild(introSub);
  introContent.appendChild(disclaimer);
  introContent.appendChild(startBtn);
  intro.appendChild(introContent);
  container.appendChild(intro);

  // ── App shell ──
  const shell = create('div', 'app-shell');
  shell.hidden = true;

  // ── View navigation ──
  const nav = create('nav', 'view-nav');
  const viewNames = ['Dashboard', 'Case Explainer', 'Kommunekort'];
  const navBtns = viewNames.map((name, i) => {
    const btn = create('button', i === 0 ? 'active' : '');
    btn.textContent = name;
    btn.addEventListener('click', () => switchView(i));
    nav.appendChild(btn);
    return btn;
  });
  shell.appendChild(nav);

  // ── Filter indicator ──
  const filterIndicator = create('div', 'filter-indicator');
  filterIndicator.hidden = true;
  shell.appendChild(filterIndicator);

  // ── View panels ──
  const views = [];
  for (let i = 0; i < 3; i++) {
    const panel = create('div', 'view-panel');
    panel.hidden = i !== 0;
    views.push(panel);
    shell.appendChild(panel);
  }

  // ── Performance HUD ──
  const hud = create('div', 'perf-hud');
  hud.hidden = true;
  hud.textContent = 'Perf: --';
  shell.appendChild(hud);

  container.appendChild(shell);

  // ── Case panel ──
  const casePanel = createCasePanel();
  container.appendChild(casePanel.element);

  // ── View 1: Dashboard ──
  function buildDashboard() {
    const v = views[0];
    clearChildren(v);

    const layout = create('div', 'dashboard-layout');

    const sidebar = create('div', 'dashboard-sidebar');
    const sliderSection = create('section', 'slider-panel');
    renderSliderPanel(sliderSection, handleSliderChange);
    sidebar.appendChild(sliderSection);

    const affectedSection = create('ul', 'affected-list');
    if (engine.topAffected) {
      renderAffectedList(affectedSection, engine.topAffected.map(a => ({
        borger_id: a.borger_id,
        total_delta: a.total_delta ?? a.delta,
      })), handleBorgerClick);
    }
    sidebar.appendChild(affectedSection);
    layout.appendChild(sidebar);

    const main = create('div', 'dashboard-main');

    const statsSection = create('section', 'stats-panel');
    statsSection.setAttribute('aria-label', 'Nøgletal');
    renderStatsPanel(statsSection, engine.baseline, scenarioDiff);
    main.appendChild(statsSection);

    const histSection = create('div', 'histogram-section');
    const histSvg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    histSvg.classList.add('histogram');
    histSvg.style.width = '100%';
    histSvg.style.height = '200px';
    histSection.appendChild(histSvg);
    main.appendChild(histSection);

    layout.appendChild(main);
    v.appendChild(layout);

    if (engine.histogramData) {
      requestAnimationFrame(() => renderHistogram(histSvg, engine.histogramData, null));
    }
  }

  // ── View 2: Case Explainer ──
  function buildCaseView() {
    const v = views[1];
    clearChildren(v);

    const info = create('p', 'case-view-info');
    info.textContent = 'Klik på en borger i listen for at se detaljeret forklaring.';
    v.appendChild(info);

    const list = create('ul', 'affected-list');
    if (engine.topAffected) {
      renderAffectedList(list, engine.topAffected.map(a => ({
        borger_id: a.borger_id,
        total_delta: a.total_delta ?? a.delta,
      })), handleBorgerClick);
    }
    v.appendChild(list);
  }

  // ── View 3: Geo View ──
  let geoKort = null;
  function buildGeoView() {
    const v = views[2];
    clearChildren(v);

    const geoContainer = create('div', 'geo-container');
    geoKort = createKommuneKort(geoContainer);
    if (engine.geoData) geoKort.update(engine.geoData, false);

    geoKort.onFilterChange((id) => {
      activeKommune = id;
      updateFilterIndicator();
      updateStatsForFilter();
    });

    v.appendChild(geoContainer);
  }

  // ── Handlers ──
  function refreshDashboardData() {
    const v = views[0];
    const statsSection = v.querySelector('.stats-panel');
    if (statsSection) renderStatsPanel(statsSection, engine.baseline, scenarioDiff);

    const affectedSection = v.querySelector('.affected-list');
    if (affectedSection && engine.topAffected) {
      clearChildren(affectedSection);
      renderAffectedList(affectedSection, engine.topAffected.map(a => ({
        borger_id: a.borger_id,
        total_delta: a.total_delta ?? a.delta,
      })), handleBorgerClick);
    }

    const histSvg = v.querySelector('.histogram');
    if (histSvg && engine.histogramData) {
      renderHistogram(histSvg, engine.histogramData, null);
    }
  }

  function handleSliderChange(paramId, value) {
    const t0 = performance.now();
    if (engine.applyScenario) {
      engine.applyScenario(paramId, value);
      scenarioDiff = engine.diff;
    }
    perfTimings.lastEval = performance.now() - t0;
    refreshDashboardData();
    if (geoKort && engine.geoData) geoKort.update(engine.geoData, false);
    updateHud();
  }

  function handleBorgerClick(borgerId) {
    if (engine.getCaseDetail) {
      engine.getCaseDetail(borgerId);
    }
    if (engine.caseDetail) {
      casePanel.open(engine.caseDetail);
    }
  }

  function switchView(idx) {
    currentView = idx;
    views.forEach((v, i) => { v.hidden = i !== idx; });
    navBtns.forEach((b, i) => {
      b.className = i === idx ? 'active' : '';
    });
  }

  function updateFilterIndicator() {
    if (activeKommune != null) {
      filterIndicator.textContent = 'Filter: Kommune ' + activeKommune;
      filterIndicator.hidden = false;
    } else {
      filterIndicator.hidden = true;
    }
  }

  function updateStatsForFilter() {
    const statsSection = views[0].querySelector('.stats-panel');
    if (statsSection) {
      renderStatsPanel(statsSection, engine.baseline, scenarioDiff);
    }
  }

  function updateHud() {
    hud.textContent = 'Eval: ' + perfTimings.lastEval.toFixed(1) + 'ms · Borgere: ' + perfTimings.count;
  }

  // ── Keyboard shortcuts ──
  document.addEventListener('keydown', (e) => {
    if (!started) return;
    if (e.target.tagName === 'INPUT' || e.target.tagName === 'TEXTAREA') return;

    switch (e.key) {
      case '1': switchView(0); break;
      case '2': switchView(1); break;
      case '3': switchView(2); break;
      case 'p':
      case 'P':
        hud.hidden = !hud.hidden;
        break;
      case 'f':
      case 'F':
        if (!document.fullscreenElement) {
          container.requestFullscreen?.();
        } else {
          document.exitFullscreen?.();
        }
        break;
    }
  });

  // ── Start ──
  startBtn.addEventListener('click', () => {
    started = true;
    intro.hidden = true;
    shell.hidden = false;

    buildDashboard();
    buildCaseView();
    buildGeoView();
    updateHud();
  });

  return {
    switchView,
    getState: () => ({ currentView, activeKommune, started }),
  };
}
