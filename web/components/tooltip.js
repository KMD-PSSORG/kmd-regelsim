/**
 * Positioned tooltip for map hover.
 * Usage: const tip = createTooltip(); container.appendChild(tip.element);
 *        tip.show(x, y, 'content'); tip.hide();
 */
export function createTooltip() {
  const el = document.createElement('div');
  el.className = 'geo-tooltip';
  el.hidden = true;
  Object.assign(el.style, {
    position: 'absolute',
    pointerEvents: 'none',
    zIndex: '100',
    padding: '8px 12px',
    borderRadius: '6px',
    fontSize: '0.82rem',
    lineHeight: '1.4',
    maxWidth: '220px',
    transition: 'opacity 0.12s ease',
  });

  return {
    element: el,
    show(x, y, content) {
      el.hidden = false;
      el.textContent = '';
      if (typeof content === 'string') {
        el.textContent = content;
      } else if (content instanceof Node) {
        el.appendChild(content);
      }
      el.style.left = (x + 12) + 'px';
      el.style.top = (y - 8) + 'px';
    },
    hide() {
      el.hidden = true;
    },
  };
}
