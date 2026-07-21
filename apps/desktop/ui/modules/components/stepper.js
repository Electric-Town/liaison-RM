/**
 * Stepper Component - Renders 5-stage Event Progress Stepper
 * Stages: Details, Cohort, Attendees, Readiness, Brief
 */
export function renderStepper(stages = []) {
  const defaultStages = [
    { label: 'Details', status: 'confirmed' },
    { label: 'Cohort', status: 'confirmed' },
    { label: 'Attendees', status: 'confirmed' },
    { label: 'Readiness', status: 'action-needed' },
    { label: 'Brief', status: 'not-started' }
  ];

  const items = stages.length > 0 ? stages : defaultStages;

  return `
    <div class="stepper" role="region" aria-label="Event progress stages">
      ${items.map((stage, idx) => {
        const isLast = idx === items.length - 1;
        let iconSymbol = '✓';
        let statusClass = 'is-confirmed';
        let statusLabel = 'Confirmed';

        if (stage.status === 'action-needed') {
          iconSymbol = '!';
          statusClass = 'is-action-needed';
          statusLabel = 'Action needed';
        } else if (stage.status === 'not-started') {
          iconSymbol = '○';
          statusClass = 'is-pending';
          statusLabel = 'Not started';
        } else if (stage.status === 'current') {
          iconSymbol = '●';
          statusClass = 'is-current';
          statusLabel = 'Current step';
        }

        const isCurrentAttr = stage.status === 'action-needed' || stage.status === 'current' ? 'aria-current="step"' : '';

        return `
          <div class="step-item ${statusClass}" ${isCurrentAttr}>
            <span class="step-icon" title="${stage.label}: ${statusLabel}">${iconSymbol}</span>
            <span>${stage.label} <small style="display:block; font-size:0.72rem; color:var(--muted); font-weight:400;">${statusLabel}</small></span>
          </div>
          ${!isLast ? `<div class="step-divider" aria-hidden="true"></div>` : ''}
        `;
      }).join('')}
    </div>
  `;
}
