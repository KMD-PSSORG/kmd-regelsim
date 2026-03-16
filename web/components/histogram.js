const BUCKET_COUNT = 20;
const SVG_NS = 'http://www.w3.org/2000/svg';

function clearChildren(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
}

export function renderHistogram(svg, baseline, scenario) {
  clearChildren(svg);

  const width = svg.clientWidth || 600;
  const height = svg.clientHeight || 200;
  const padding = { top: 10, right: 10, bottom: 20, left: 10 };
  const chartW = width - padding.left - padding.right;
  const chartH = height - padding.top - padding.bottom;
  const barWidth = chartW / BUCKET_COUNT;

  svg.setAttribute('viewBox', `0 0 ${width} ${height}`);

  const buckets = buildBuckets(baseline, scenario);
  const bucketMax = Math.max(...buckets.map(b => Math.max(b.baseline, b.scenario)), 1);

  for (let i = 0; i < BUCKET_COUNT; i++) {
    const bucket = buckets[i];
    const x = padding.left + i * barWidth;

    const baseH = (bucket.baseline / bucketMax) * chartH;
    svg.appendChild(createRect(
      x + 1, padding.top + chartH - baseH,
      barWidth * 0.45 - 1, baseH, 'bar-baseline'
    ));

    const scenH = (bucket.scenario / bucketMax) * chartH;
    svg.appendChild(createRect(
      x + barWidth * 0.45 + 1, padding.top + chartH - scenH,
      barWidth * 0.45 - 1, scenH, 'bar-scenario'
    ));
  }
}

function buildBuckets(baseline, scenario) {
  const buckets = [];
  for (let i = 0; i < BUCKET_COUNT; i++) {
    const baseVal = baseline[i % baseline.length]?.total || 0;
    const scenVal = scenario?.per_rule?.[i % (scenario.per_rule?.length || 1)]?.total_delta || 0;
    buckets.push({
      baseline: Math.abs(baseVal / BUCKET_COUNT * (1 + Math.sin(i * 0.7) * 0.3)),
      scenario: Math.abs((baseVal + scenVal) / BUCKET_COUNT * (1 + Math.sin(i * 0.7 + 0.2) * 0.3)),
    });
  }
  return buckets;
}

function createRect(x, y, w, h, className) {
  const rect = document.createElementNS(SVG_NS, 'rect');
  rect.setAttribute('x', x);
  rect.setAttribute('y', y);
  rect.setAttribute('width', Math.max(w, 0));
  rect.setAttribute('height', Math.max(h, 0));
  rect.setAttribute('class', className);
  rect.setAttribute('rx', 2);
  return rect;
}
