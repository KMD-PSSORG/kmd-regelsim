import { renderRuleGraph } from './rule_graph.js';

const DKK = new Intl.NumberFormat('da-DK', { style: 'currency', currency: 'DKK', maximumFractionDigits: 0 });
const NUM = new Intl.NumberFormat('da-DK');

const HUSSTAND_LABELS = {
  enlig: 'Enlig',
  par_uden_boern: 'Par uden b\u00f8rn',
  par_med_boern: 'Par med b\u00f8rn',
  enlig_forsoerger: 'Enlig fors\u00f8rger',
};

const BESK_LABELS = {
  fuldtid: 'Fuldtid',
  deltid: 'Deltid',
  ledig: 'Ledig',
  aktivitetsparat: 'Aktivitetsparat',
  sygemeldt: 'Sygemeldt',
};

function clearChildren(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
}

export function createCasePanel() {
  const overlay = document.createElement('div');
  overlay.className = 'case-overlay closed';
  overlay.hidden = true;
  overlay.setAttribute('role', 'dialog');
  overlay.setAttribute('aria-label', 'Borger detaljer');

  const backdrop = document.createElement('div');
  backdrop.className = 'case-backdrop';

  const panel = document.createElement('div');
  panel.className = 'case-panel';

  overlay.appendChild(backdrop);
  overlay.appendChild(panel);

  let previousFocus = null;

  function close() {
    overlay.classList.remove('open');
    overlay.classList.add('closed');
    overlay.hidden = true;
    if (previousFocus && previousFocus.focus) {
      previousFocus.focus();
      previousFocus = null;
    }
  }

  backdrop.addEventListener('click', close);
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape' && !overlay.hidden) close();
  });

  function open(caseData) {
    previousFocus = document.activeElement;
    clearChildren(panel);
    overlay.classList.remove('closed');
    overlay.classList.add('open');
    overlay.hidden = false;

    const header = document.createElement('div');
    header.className = 'case-header';

    const title = document.createElement('h2');
    title.textContent = 'Borger #' + caseData.borger_id;
    header.appendChild(title);

    const closeBtn = document.createElement('button');
    closeBtn.className = 'case-close';
    closeBtn.textContent = '\u2715';
    closeBtn.setAttribute('aria-label', 'Luk');
    closeBtn.addEventListener('click', close);
    header.appendChild(closeBtn);

    panel.appendChild(header);

    const info = document.createElement('div');
    info.className = 'case-info';
    const fields = [
      ['Alder', caseData.alder + ' \u00e5r'],
      ['Kommune', caseData.kommune_id.toString()],
      ['Husstand', HUSSTAND_LABELS[caseData.husstandstype] || caseData.husstandstype],
      ['Besk\u00e6ftigelse', BESK_LABELS[caseData.beskaeftigelse] || caseData.beskaeftigelse],
      ['Indkomst', DKK.format(caseData.indkomst) + '/\u00e5r'],
      ['Husleje', DKK.format(caseData.husleje) + '/md'],
      ['Boligareal', caseData.boligareal + ' m\u00b2'],
      ['B\u00f8rn', caseData.antal_boern.toString()],
    ];
    for (const [label, value] of fields) {
      const row = document.createElement('div');
      row.className = 'case-field';
      const labelEl = document.createElement('span');
      labelEl.className = 'case-field-label';
      labelEl.textContent = label;
      const valueEl = document.createElement('span');
      valueEl.className = 'case-field-value';
      valueEl.textContent = value;
      row.appendChild(labelEl);
      row.appendChild(valueEl);
      info.appendChild(row);
    }
    panel.appendChild(info);

    const maxDeltaRule = caseData.rules.reduce((a, b) =>
      Math.abs(b.delta) > Math.abs(a.delta) ? b : a
    );

    const graphContainer = document.createElement('div');
    graphContainer.className = 'case-graph';
    renderRuleGraph(graphContainer, caseData.rules, maxDeltaRule.rule);
    panel.appendChild(graphContainer);

    const rulesSection = document.createElement('div');
    rulesSection.className = 'case-rules';
    for (const rule of caseData.rules) {
      const card = document.createElement('div');
      card.className = 'case-rule-card';
      if (rule.rule === maxDeltaRule.rule && Math.abs(rule.delta) > 0.01) {
        card.classList.add('highlighted');
      }

      const ruleTitle = document.createElement('div');
      ruleTitle.className = 'case-rule-title';
      ruleTitle.textContent = formatRuleName(rule.rule);
      card.appendChild(ruleTitle);

      const amounts = document.createElement('div');
      amounts.className = 'case-rule-amounts';
      amounts.textContent = DKK.format(rule.baseline_amount);
      if (Math.abs(rule.delta) > 0.01) {
        amounts.textContent += ' \u2192 ' + DKK.format(rule.scenario_amount);
        const deltaSpan = document.createElement('span');
        deltaSpan.className = 'delta ' + (rule.delta > 0 ? 'positive' : 'negative');
        deltaSpan.textContent = ' (' + (rule.delta > 0 ? '+' : '') + DKK.format(rule.delta) + ')';
        amounts.appendChild(deltaSpan);
      }
      card.appendChild(amounts);

      const explanation = document.createElement('div');
      explanation.className = 'case-rule-explanation';
      explanation.textContent = rule.explanation;
      card.appendChild(explanation);

      if (rule.threshold_warnings && rule.threshold_warnings.length > 0) {
        for (const warning of rule.threshold_warnings) {
          const warn = document.createElement('div');
          warn.className = 'case-threshold-warning';
          warn.textContent = '\u26a0 ' + warning;
          card.appendChild(warn);
        }
      }

      rulesSection.appendChild(card);
    }
    panel.appendChild(rulesSection);

    requestAnimationFrame(() => {
      const first = panel.querySelector('button, [tabindex="0"]');
      if (first) first.focus();
    });
  }

  overlay.addEventListener('keydown', (e) => {
    if (e.key !== 'Tab' || overlay.hidden) return;
    const focusable = Array.from(panel.querySelectorAll(
      'button, [href], input, select, textarea, [tabindex="0"]'
    ));
    if (focusable.length === 0) return;
    const first = focusable[0];
    const last = focusable[focusable.length - 1];
    if (e.shiftKey && document.activeElement === first) {
      e.preventDefault();
      last.focus();
    } else if (!e.shiftKey && document.activeElement === last) {
      e.preventDefault();
      first.focus();
    }
  });

  return { open, close, element: overlay };
}

function formatRuleName(rule) {
  const names = {
    kontanthjaelp: 'Kontanthj\u00e6lp',
    boligstoette: 'Boligst\u00f8tte',
    boerneydelse: 'B\u00f8rneydelse',
  };
  return names[rule] || rule;
}
