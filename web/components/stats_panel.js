const DKK = new Intl.NumberFormat('da-DK', { style: 'currency', currency: 'DKK', maximumFractionDigits: 0 });
const NUM = new Intl.NumberFormat('da-DK');

function clearChildren(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
}

export function renderStatsPanel(container, baseline, scenarioDiff) {
  clearChildren(container);

  for (const rule of baseline) {
    const card = document.createElement('div');
    card.className = 'stat-card';

    const label = document.createElement('div');
    label.className = 'label';
    label.textContent = formatRuleName(rule.rule);

    const value = document.createElement('div');
    value.className = 'value';
    value.textContent = DKK.format(rule.total / 12);

    card.appendChild(label);
    card.appendChild(value);

    if (scenarioDiff) {
      const diff = scenarioDiff.per_rule?.find(d => d.rule === rule.rule);
      if (diff && Math.abs(diff.total_delta) > 0.01) {
        const delta = document.createElement('div');
        delta.className = `delta ${diff.total_delta > 0 ? 'positive' : 'negative'}`;
        delta.textContent = `${diff.total_delta > 0 ? '+' : ''}${DKK.format(diff.total_delta / 12)}/md`;
        card.appendChild(delta);
      }
    }

    const eligible = document.createElement('div');
    eligible.className = 'label';
    eligible.textContent = `${NUM.format(rule.eligible)} berettigede \u00b7 gns. ${DKK.format(rule.mean)}`;
    card.appendChild(eligible);

    container.appendChild(card);
  }
}

function formatRuleName(rule) {
  const names = {
    kontanthjaelp: 'Kontanthj\u00e6lp',
    boligstoette: 'Boligst\u00f8tte',
    boerneydelse: 'B\u00f8rneydelse',
  };
  return names[rule] || rule;
}
