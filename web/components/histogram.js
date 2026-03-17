const SVG_NS = 'http://www.w3.org/2000/svg';
const FMT = new Intl.NumberFormat('da-DK', { maximumFractionDigits: 0 });

function clearChildren(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
}

export function renderHistogram(svg, histogramData, scenarioHistogramData) {
  clearChildren(svg);

  if (!histogramData || !histogramData.buckets || histogramData.buckets.length === 0) return;

  const width = svg.clientWidth || 600;
  const height = svg.clientHeight || 200;
  const padding = { top: 10, right: 10, bottom: 20, left: 10 };
  const chartW = width - padding.left - padding.right;
  const chartH = height - padding.top - padding.bottom;
  const bucketCount = histogramData.buckets.length;
  const barWidth = chartW / bucketCount;

  svg.setAttribute('viewBox', `0 0 ${width} ${height}`);

  const baseBuckets = histogramData.buckets;
  const scenBuckets = scenarioHistogramData?.buckets || [];

  const maxCount = Math.max(
    ...baseBuckets.map(b => b.count),
    ...scenBuckets.map(b => b.count),
    1
  );

  for (let i = 0; i < bucketCount; i++) {
    const x = padding.left + i * barWidth;
    const baseCount = baseBuckets[i]?.count || 0;

    const baseH = (baseCount / maxCount) * chartH;
    svg.appendChild(createRect(
      x + 1, padding.top + chartH - baseH,
      barWidth * 0.45 - 1, baseH, 'bar-baseline'
    ));

    if (scenBuckets.length > 0) {
      const scenCount = scenBuckets[i]?.count || 0;
      const scenH = (scenCount / maxCount) * chartH;
      svg.appendChild(createRect(
        x + barWidth * 0.45 + 1, padding.top + chartH - scenH,
        barWidth * 0.45 - 1, scenH, 'bar-scenario'
      ));
    }
  }
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
