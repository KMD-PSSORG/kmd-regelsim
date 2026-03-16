const DKK = new Intl.NumberFormat('da-DK', { style: 'currency', currency: 'DKK', maximumFractionDigits: 0 });

function clearChildren(el) {
  while (el.firstChild) el.removeChild(el.firstChild);
}

export function renderAffectedList(container, topAffected, onClickBorger) {
  clearChildren(container);

  for (const entry of topAffected) {
    const li = document.createElement('li');
    li.setAttribute('tabindex', '0');
    li.setAttribute('role', 'button');
    li.setAttribute('aria-label', `Borger ${entry.borger_id}, delta ${DKK.format(entry.total_delta)}`);

    const idSpan = document.createElement('span');
    idSpan.className = 'borger-id';
    idSpan.textContent = `#${entry.borger_id}`;

    const deltaSpan = document.createElement('span');
    deltaSpan.className = 'borger-delta';
    deltaSpan.classList.add(entry.total_delta > 0 ? 'positive' : 'negative');
    deltaSpan.textContent = `${entry.total_delta > 0 ? '+' : ''}${DKK.format(entry.total_delta)}`;

    li.appendChild(idSpan);
    li.appendChild(deltaSpan);

    li.addEventListener('click', () => onClickBorger(entry.borger_id));
    li.addEventListener('keydown', (e) => {
      if (e.key === 'Enter' || e.key === ' ') {
        e.preventDefault();
        onClickBorger(entry.borger_id);
      }
    });

    container.appendChild(li);
  }
}
