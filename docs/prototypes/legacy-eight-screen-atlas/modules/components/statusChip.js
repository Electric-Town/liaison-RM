/**
 * StatusChip Component - Atomic text-labelled state indicator
 * Variants: good (green), warn (amber), bad (red), info (blue)
 */
export function renderStatusChip({ label, variant = 'info', icon = '' }) {
  const variantClassMap = {
    good: 'good',
    warn: 'warn',
    bad: 'bad',
    danger: 'bad',
    info: 'info',
    success: 'good'
  };

  const cssClass = variantClassMap[variant] || 'info';

  return `
    <span class="chip ${cssClass}" role="status">
      ${icon ? `<span aria-hidden="true">${icon}</span>` : ''}
      <span>${label}</span>
    </span>
  `;
}
