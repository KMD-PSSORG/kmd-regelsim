const DKK = new Intl.NumberFormat('da-DK', { style: 'currency', currency: 'DKK', maximumFractionDigits: 0 });
const NUM = new Intl.NumberFormat('da-DK');

function clearChildren(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
}

export function renderSegmentTable(container, segments) {
  const thead = container.querySelector('thead');
  const tbody = container.querySelector('tbody');
  clearChildren(thead);
  clearChildren(tbody);

  const headerRow = document.createElement('tr');
  for (const col of ['Husstandstype', 'Total', 'Berettigede', 'Gennemsnit']) {
    const th = document.createElement('th');
    th.textContent = col;
    th.setAttribute('scope', 'col');
    headerRow.appendChild(th);
  }
  thead.appendChild(headerRow);

  for (const seg of segments) {
    const tr = document.createElement('tr');

    const tdType = document.createElement('td');
    tdType.textContent = seg.type;
    tr.appendChild(tdType);

    const tdTotal = document.createElement('td');
    tdTotal.textContent = DKK.format(seg.total);
    tr.appendChild(tdTotal);

    const tdEligible = document.createElement('td');
    tdEligible.textContent = NUM.format(seg.eligible);
    tr.appendChild(tdEligible);

    const tdMean = document.createElement('td');
    tdMean.textContent = DKK.format(seg.mean);
    tr.appendChild(tdMean);

    tbody.appendChild(tr);
  }
}
