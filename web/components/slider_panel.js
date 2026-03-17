const SLIDERS = [
  { paramId: 0, label: 'Kontanthjælp basis (enlig)', min: 8000, max: 18000, step: 50, default: 12550 },
  { paramId: 4, label: 'Kontanthjælp basis (par)', min: 5000, max: 14000, step: 50, default: 8710 },
  { paramId: 1, label: 'Forsørgertillæg pr. barn', min: 500, max: 3000, step: 10, default: 1710 },
  { paramId: 2, label: 'Boligstøtte grænsebeløb', min: 200000, max: 500000, step: 5000, default: 350000 },
  { paramId: 3, label: 'Børneydelse aftrapningsgrænse', min: 500000, max: 1200000, step: 10000, default: 828100 },
];

const DEBOUNCE_MS = 24;
const FMT = new Intl.NumberFormat('da-DK');

export function renderSliderPanel(container, onChange) {
  let existing = container.querySelector('.slider-group');
  if (existing) existing.remove();

  const group = document.createElement('div');
  group.className = 'slider-group';

  for (const cfg of SLIDERS) {
    const item = document.createElement('div');
    item.className = 'slider-item';

    const label = document.createElement('label');
    const labelText = document.createTextNode(cfg.label + ' ');
    const valueSpan = document.createElement('span');
    valueSpan.className = 'slider-value';
    valueSpan.textContent = FMT.format(cfg.default);
    label.appendChild(labelText);
    label.appendChild(valueSpan);

    const input = document.createElement('input');
    input.type = 'range';
    input.min = cfg.min;
    input.max = cfg.max;
    input.step = cfg.step;
    input.value = cfg.default;
    input.dataset.paramId = cfg.paramId;
    input.setAttribute('aria-label', cfg.label);

    let timer = null;
    input.addEventListener('input', () => {
      valueSpan.textContent = FMT.format(Number(input.value));
      clearTimeout(timer);
      timer = setTimeout(() => {
        onChange(cfg.paramId, Number(input.value));
      }, DEBOUNCE_MS);
    });

    input.addEventListener('pointerup', () => {
      clearTimeout(timer);
      onChange(cfg.paramId, Number(input.value));
    });

    item.appendChild(label);
    item.appendChild(input);
    group.appendChild(item);
  }

  container.appendChild(group);
}
