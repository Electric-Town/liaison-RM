/**
 * NoticeBanner Component - Explicit notice & recovery banner
 */
export function renderNoticeBanner({ title, message, variant = 'info', actionText = '', actionId = '' }) {
  const borderColors = {
    info: 'var(--action)',
    warning: 'var(--warning)',
    danger: 'var(--danger)',
    success: 'var(--success)'
  };

  const bgColors = {
    info: 'var(--accent-soft)',
    warning: 'var(--warning-soft)',
    danger: 'var(--danger-soft)',
    success: 'var(--success-soft)'
  };

  const borderColor = borderColors[variant] || borderColors.info;
  const bgColor = bgColors[variant] || bgColors.info;

  return `
    <div class="callout" style="border-left-color: ${borderColor}; background: ${bgColor}; padding: 0.9rem 1.1rem;" role="note">
      <strong>${title}</strong>
      <p style="margin: 0.25rem 0 0.5rem 0; font-size: 0.9rem;">${message}</p>
      ${actionText ? `<button id="${actionId}" class="primary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.85rem; font-size: 0.88rem; background:${borderColor};">${actionText}</button>` : ''}
    </div>
  `;
}
