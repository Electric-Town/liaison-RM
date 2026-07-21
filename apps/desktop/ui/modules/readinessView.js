/**
 * ReadinessView Module - Screen 03: EVENT ATTENDEE READINESS
 */
import { renderStepper } from './components/stepper.js';
import { renderDataTable } from './components/dataTable.js';
import { renderStatusChip } from './components/statusChip.js';

export function renderReadinessView(state = {}) {
  const attendees = [
    {
      initials: 'AB',
      name: 'Aisling Byrne',
      availability: 'Confirmed (09 Jul)',
      dietary: 'No beef',
      disclosure: 'Instruction available',
      readiness: 'Ready',
      readinessVariant: 'good'
    },
    {
      initials: 'LL',
      name: 'Liam Lynch',
      availability: 'Pending',
      dietary: '—',
      disclosure: 'None',
      readiness: 'Action needed',
      readinessVariant: 'danger'
    },
    {
      initials: 'JH',
      name: 'John Hale',
      availability: 'Confirmed (02 Jul)',
      dietary: 'No shellfish',
      disclosure: 'Instruction available',
      readiness: 'Ready',
      readinessVariant: 'good'
    },
    {
      initials: 'AC',
      name: 'Adriana Cerry',
      availability: 'Confirmed (16 Jan)',
      dietary: 'Gluten-free',
      disclosure: 'Instruction available',
      readiness: 'Ready',
      readinessVariant: 'good'
    }
  ];

  return `
    <section class="page" data-page="readiness" aria-labelledby="readiness-heading">
      <!-- Header -->
      <div style="display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 1rem; margin-bottom: 1rem;">
        <div>
          <h1 id="readiness-heading" style="font-family: var(--font-title); font-size: 1.8rem; margin: 0;">
            All-hands lunch <small style="font-family: var(--font-body); font-size: 1rem; color: var(--muted); font-weight: 400;">· 24 Jul 2026 ✏️</small>
          </h1>
          <p style="margin: 0.25rem 0 0 0; color: var(--muted);">Resolve remaining gap before creating the least-disclosure catering brief.</p>
        </div>
        <button class="secondary-button" type="button" style="display: flex; align-items: center; gap: 0.4rem; min-height: 40px; padding: 0.4rem 0.85rem;">
          <span>⚙</span> Event details
        </button>
      </div>

      <!-- 5-Stage Stepper -->
      <div class="card" style="padding: 1rem; margin-bottom: 1.25rem;">
        ${renderStepper([
          { label: 'Details', status: 'confirmed' },
          { label: 'Cohort', status: 'confirmed' },
          { label: 'Attendees', status: 'confirmed' },
          { label: 'Readiness', status: 'action-needed' },
          { label: 'Brief', status: 'not-started' }
        ])}
      </div>

      <!-- Main Two-Column Layout -->
      <div class="two-column" style="display: grid; grid-template-columns: 2fr 1fr; gap: 1.25rem; align-items: start;">
        
        <!-- Column 1: Attendee Reconciliation Table -->
        <div class="card" style="padding: 1.2rem;">
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.85rem;">
            <div>
              <h2 style="font-size: 1.15rem; margin: 0;">Attendee reconciliation</h2>
              <p style="margin: 0.2rem 0 0 0; font-size: 0.85rem; color: var(--muted);">4 attending · 1 remaining gap</p>
            </div>
          </div>

          <div class="table-wrap">
            <table style="width: 100%; border-collapse: collapse; font-size: 0.88rem;">
              <thead>
                <tr style="border-bottom: 2px solid var(--border); text-align: left; font-size: 0.78rem; text-transform: uppercase; color: var(--muted);">
                  <th style="padding: 0.55rem 0.5rem;">Attendee</th>
                  <th style="padding: 0.55rem 0.5rem;">Availability</th>
                  <th style="padding: 0.55rem 0.5rem;">Dietary needs</th>
                  <th style="padding: 0.55rem 0.5rem;">Disclosure</th>
                  <th style="padding: 0.55rem 0.5rem;">Readiness</th>
                  <th style="padding: 0.55rem 0.5rem;">Next action</th>
                </tr>
              </thead>
              <tbody>
                ${attendees.map(att => `
                  <tr style="border-bottom: 1px solid var(--border);">
                    <td style="padding: 0.75rem 0.5rem;">
                      <div style="display: flex; align-items: center; gap: 0.5rem;">
                        <span style="display: inline-grid; place-items: center; width: 28px; height: 28px; border-radius: 50%; background: var(--highlight); font-weight: 700; font-size: 0.75rem; color: var(--content-on-highlight);">${att.initials}</span>
                        <strong>${att.name}</strong>
                      </div>
                    </td>
                    <td style="padding: 0.75rem 0.5rem; color: var(--muted);">${att.availability}</td>
                    <td style="padding: 0.75rem 0.5rem;">${att.dietary}</td>
                    <td style="padding: 0.75rem 0.5rem; color: var(--muted);">${att.disclosure}</td>
                    <td style="padding: 0.75rem 0.5rem;">
                      ${renderStatusChip({ label: att.readiness, variant: att.readinessVariant })}
                    </td>
                    <td style="padding: 0.75rem 0.5rem;">
                      <button class="view-history-trigger" type="button" style="background: none; border: none; padding: 0; min-height: initial; min-width: initial; color: var(--action); font-size: 0.85rem; font-weight: 600; cursor: pointer;">View evidence</button>
                    </td>
                  </tr>
                `).join('')}
              </tbody>
            </table>
          </div>
        </div>

        <!-- Column 2: Least-disclosure catering brief & Notes -->
        <div style="display: grid; gap: 1.25rem;">
          
          <!-- Catering Brief Card -->
          <div class="card" style="padding: 1.2rem;">
            <h2 style="font-size: 1.15rem; margin-top: 0;">Least-disclosure catering brief</h2>
            <p style="margin: 0 0 0.75rem 0; font-size: 0.85rem; color: var(--muted);">Only information necessary for catering and seating.</p>

            <ul style="margin: 0 0 1rem 0; padding-left: 1.2rem; font-size: 0.88rem; display: grid; gap: 0.4rem;">
              <li>6 attendees total</li>
              <li>Dietary needs will be provided privately</li>
              <li>Seating preference: Mixed tables</li>
            </ul>

            <button class="secondary-button full-width" type="button" style="min-height: 40px; font-size: 0.88rem;">
              Open brief →
            </button>
          </div>

          <!-- Notes Card -->
          <div class="card" style="padding: 1.2rem;">
            <h2 style="font-size: 1.15rem; margin-top: 0;">Notes</h2>
            <textarea placeholder="Add a note about this event..." rows="4" style="width: 100%; font-family: inherit; font-size: 0.88rem; padding: 0.6rem; border: 1px solid var(--border); border-radius: 0.35rem; background: var(--surface); color: var(--ink); resize: vertical;"></textarea>
          </div>

        </div>

      </div>
    </section>
  `;
}
