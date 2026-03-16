const SVG_NS = 'http://www.w3.org/2000/svg';

const RULES = [
  { id: 'kontanthjaelp', label: 'Kontanthj\u00e6lp', x: 50, y: 60 },
  { id: 'boligstoette', label: 'Boligst\u00f8tte', x: 250, y: 60 },
  { id: 'boerneydelse', label: 'B\u00f8rneydelse', x: 450, y: 60 },
];

const DEPS = [
  { from: 'kontanthjaelp', to: 'boligstoette' },
];

const BOX_W = 130;
const BOX_H = 40;

function clearChildren(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
}

export function renderRuleGraph(container, rules, highlightRule) {
  clearChildren(container);

  const svg = document.createElementNS(SVG_NS, 'svg');
  svg.setAttribute('viewBox', '0 0 600 140');
  svg.setAttribute('width', '100%');
  svg.setAttribute('height', '140');
  svg.setAttribute('role', 'img');
  svg.setAttribute('aria-label', 'Regelafh\u00e6ngigheder');

  const defs = document.createElementNS(SVG_NS, 'defs');
  const marker = document.createElementNS(SVG_NS, 'marker');
  marker.setAttribute('id', 'arrowhead');
  marker.setAttribute('markerWidth', '10');
  marker.setAttribute('markerHeight', '7');
  marker.setAttribute('refX', '10');
  marker.setAttribute('refY', '3.5');
  marker.setAttribute('orient', 'auto');
  const polygon = document.createElementNS(SVG_NS, 'polygon');
  polygon.setAttribute('points', '0 0, 10 3.5, 0 7');
  polygon.setAttribute('fill', 'var(--accent, #4f8ff7)');
  marker.appendChild(polygon);
  defs.appendChild(marker);
  svg.appendChild(defs);

  for (const dep of DEPS) {
    const from = RULES.find(r => r.id === dep.from);
    const to = RULES.find(r => r.id === dep.to);
    if (!from || !to) continue;

    const line = document.createElementNS(SVG_NS, 'line');
    line.setAttribute('x1', from.x + BOX_W);
    line.setAttribute('y1', from.y + BOX_H / 2);
    line.setAttribute('x2', to.x);
    line.setAttribute('y2', to.y + BOX_H / 2);
    line.setAttribute('stroke', 'var(--accent, #4f8ff7)');
    line.setAttribute('stroke-width', '2');
    line.setAttribute('marker-end', 'url(#arrowhead)');
    line.setAttribute('class', 'dep-arrow');
    svg.appendChild(line);
  }

  for (const rule of RULES) {
    const g = document.createElementNS(SVG_NS, 'g');
    g.setAttribute('class', 'rule-box' + (rule.id === highlightRule ? ' highlighted' : ''));

    const rect = document.createElementNS(SVG_NS, 'rect');
    rect.setAttribute('x', rule.x);
    rect.setAttribute('y', rule.y);
    rect.setAttribute('width', BOX_W);
    rect.setAttribute('height', BOX_H);
    rect.setAttribute('rx', '6');
    rect.setAttribute('fill', rule.id === highlightRule ? 'var(--accent, #4f8ff7)' : 'var(--surface, #1a1d27)');
    rect.setAttribute('stroke', 'var(--border, #2a2e3a)');
    rect.setAttribute('stroke-width', '1.5');

    const text = document.createElementNS(SVG_NS, 'text');
    text.setAttribute('x', rule.x + BOX_W / 2);
    text.setAttribute('y', rule.y + BOX_H / 2 + 5);
    text.setAttribute('text-anchor', 'middle');
    text.setAttribute('fill', rule.id === highlightRule ? '#fff' : 'var(--text, #e4e6ed)');
    text.setAttribute('font-size', '13');
    text.setAttribute('font-family', 'var(--font, system-ui)');
    text.textContent = rule.label;

    const ruleData = rules?.find(r => r.rule === rule.id);
    if (ruleData && Math.abs(ruleData.delta) > 0.01) {
      const delta = document.createElementNS(SVG_NS, 'text');
      delta.setAttribute('x', rule.x + BOX_W / 2);
      delta.setAttribute('y', rule.y + BOX_H + 18);
      delta.setAttribute('text-anchor', 'middle');
      delta.setAttribute('fill', ruleData.delta > 0 ? 'var(--positive, #34d399)' : 'var(--negative, #f87171)');
      delta.setAttribute('font-size', '11');
      delta.setAttribute('font-family', 'var(--mono, monospace)');
      delta.textContent = (ruleData.delta > 0 ? '+' : '') + Math.round(ruleData.delta) + ' kr';
      g.appendChild(delta);
    }

    g.appendChild(rect);
    g.appendChild(text);
    svg.appendChild(g);
  }

  container.appendChild(svg);
}
