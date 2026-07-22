/**
 * TodayView Module - Screen 01: TODAY / OVERVIEW & Screen 08 (360PX Narrow)
 */
import { renderStepper } from './components/stepper.js';
import { renderStatusChip } from './components/statusChip.js';

export function renderTodayView(state = {}) {
  const dates = [
    { day: 'MON', date: '20' },
    { day: 'TUE', date: '21' },
    { day: 'WED', date: '22' },
    { day: 'THU', date: '23' },
    { day: 'FRI', date: '24', active: true },
    { day: 'SAT', date: '25' },
    { day: 'SUN', date: '26' }
  ];

  return `
    <section class="page" data-page="today" aria-labelledby="today-heading">
      <!-- Date Header & Quick Capture -->
      <div class="page-heading" style="display: flex; justify-content: space-between; align-items: flex-start; gap: 1rem; flex-wrap: wrap; margin-bottom: 1.25rem;">
        <div>
          <h1 id="today-heading" style="font-family: var(--font-title); font-size: 1.8rem; margin: 0;">Today <small style="font-family: var(--font-body); font-size: 1.1rem; color: var(--muted); font-weight: 400;">· Friday, 24 July 2026</small></h1>
        </div>
        <button id="quick-capture-btn" class="secondary-button" type="button" style="display: flex; align-items: center; gap: 0.4rem; min-height: 42px;">
          <span>⚡</span> Quick capture
        </button>
      </div>

      <!-- Calendar Strip -->
      <div class="date-strip" style="margin-bottom: 1.5rem;">
        ${dates.map(d => `
          <button class="date-pill ${d.active ? 'is-active' : ''}" type="button" aria-current="${d.active ? 'date' : 'false'}">
            <small>${d.day}</small>
            <strong>${d.date}</strong>
          </button>
        `).join('')}
      </div>

      <!-- Main Dashboard Grid -->
      <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); gap: 1.25rem;">
        
        <!-- Card 1: Prepare Event Alert -->
        <div class="profile-card" style="border-left: 5px solid var(--danger);">
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem;">
            <span class="chip-badge chip-warning">⚡ Prepare</span>
            <small style="color: var(--muted);">Today, 12:00</small>
          </div>
          <h2 style="font-size: 1.3rem; margin: 0 0 0.25rem 0;">All-hands lunch</h2>
          <p style="margin: 0 0 0.75rem 0; color: var(--muted); font-size: 0.9rem;">Canteen, Building A · Floor 3</p>
          
          <div style="background: var(--danger-soft); padding: 0.75rem; border-radius: 0.35rem; border-left: 3px solid var(--danger); margin-bottom: 1rem;">
            <p style="margin: 0; font-size: 0.88rem; color: var(--ink);">
              • Resolve remaining gap before creating the <strong>least-disclosure catering brief</strong>.
            </p>
          </div>

          <button class="primary-button full-width continue-readiness-btn" type="button" data-route="readiness" style="background: var(--danger); border-color: var(--danger); margin-bottom: 0.75rem;">
            Continue readiness →
          </button>

          <div style="display: grid; gap: 0.4rem; font-size: 0.88rem; border-top: 1px solid var(--border); padding-top: 0.75rem;">
            <label style="display: flex; align-items: center; gap: 0.5rem;">
              <input type="checkbox" checked>
              <span>Confirm venue shortlist <small style="color: var(--muted);">(Due tomorrow)</small></span>
            </label>
            <label style="display: flex; align-items: center; gap: 0.5rem;">
              <input type="checkbox">
              <span>Follow-up: Aisling Byrne availability <small style="color: var(--muted);">(Next week)</small></span>
            </label>
          </div>

          <div style="margin-top: 0.75rem; text-align: right;">
            <a href="#events" class="nav-link" data-route="readiness" style="font-size: 0.85rem; color: var(--action); text-decoration: none; font-weight: 600;">View event details →</a>
          </div>
        </div>

        <!-- Card 2: Upcoming Events -->
        <div class="card" style="padding: 1.2rem;">
          <h2 style="font-size: 1.15rem; margin-top: 0;">Upcoming</h2>
          <div style="display: grid; gap: 0.85rem; margin-bottom: 1rem;">
            <div style="display: flex; gap: 0.85rem; padding-bottom: 0.75rem; border-bottom: 1px solid var(--border);">
              <div style="min-width: 50px; text-align: center; background: var(--accent-soft); padding: 0.4rem; border-radius: 0.35rem; color: var(--action);">
                <small style="display: block; font-size: 0.7rem; font-weight: 700;">JUL</small>
                <strong style="font-size: 1.1rem;">29</strong>
              </div>
              <div>
                <strong style="display: block; font-size: 0.95rem;">Client dinner</strong>
                <small style="color: var(--muted);">19:00 - Offsite · The Loft</small>
              </div>
            </div>
            <div style="display: flex; gap: 0.85rem;">
              <div style="min-width: 50px; text-align: center; background: var(--accent-soft); padding: 0.4rem; border-radius: 0.35rem; color: var(--action);">
                <small style="display: block; font-size: 0.7rem; font-weight: 700;">AUG</small>
                <strong style="font-size: 1.1rem;">06</strong>
              </div>
              <div>
                <strong style="display: block; font-size: 0.95rem;">Team offsite</strong>
                <small style="color: var(--muted);">All day · Offsite · Moniusz Lodge</small>
              </div>
            </div>
          </div>
          <a href="#events" data-route="events" style="font-size: 0.85rem; color: var(--action); text-decoration: none; font-weight: 600;">View full calendar →</a>
        </div>

        <!-- Card 3: Commitments -->
        <div class="card" style="padding: 1.2rem;">
          <h2 style="font-size: 1.15rem; margin-top: 0;">Commitments</h2>
          <div style="display: grid; gap: 0.6rem; font-size: 0.88rem; margin-bottom: 1rem;">
            <label style="display: flex; align-items: flex-start; gap: 0.5rem; padding: 0.4rem 0; border-bottom: 1px dashed var(--border);">
              <input type="checkbox">
              <div>
                <strong>Confirm venue shortlist</strong>
                <small style="display: block; color: var(--muted);">Due Sun, 26 Jul</small>
              </div>
            </label>
            <label style="display: flex; align-items: flex-start; gap: 0.5rem; padding: 0.4rem 0; border-bottom: 1px dashed var(--border);">
              <input type="checkbox">
              <div>
                <strong>Confirm menu choices</strong>
                <small style="display: block; color: var(--muted);">Due Sun, 26 Jul</small>
              </div>
            </label>
            <label style="display: flex; align-items: flex-start; gap: 0.5rem; padding: 0.4rem 0;">
              <input type="checkbox" checked>
              <div>
                <strong style="text-decoration: line-through; color: var(--muted);">Email about agenda</strong>
                <small style="display: block; color: var(--muted);">Due Fri, 24 Jul</small>
              </div>
            </label>
          </div>
          <a href="#people" data-route="people" style="font-size: 0.85rem; color: var(--action); text-decoration: none; font-weight: 600;">View all commitments →</a>
        </div>

        <!-- Card 4: Event Preparation Stepper Summary -->
        <div class="card" style="padding: 1.2rem; grid-column: 1 / -1;">
          <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.75rem;">
            <div>
              <h2 style="font-size: 1.15rem; margin: 0;">Event preparation</h2>
              <p style="margin: 0.2rem 0 0 0; color: var(--muted); font-size: 0.88rem;">All-hands lunch · 24 Jul 2026</p>
            </div>
            <button class="secondary-button continue-readiness-btn" type="button" data-route="readiness" style="min-height: 38px; padding: 0.35rem 0.75rem; font-size: 0.85rem;">Open readiness →</button>
          </div>
          ${renderStepper([
            { label: 'Details', status: 'confirmed' },
            { label: 'Cohort', status: 'confirmed' },
            { label: 'Attendees', status: 'confirmed' },
            { label: 'Readiness', status: 'action-needed' },
            { label: 'Brief', status: 'not-started' }
          ])}
        </div>

        <!-- Card 5: Recent Interactions -->
        <div class="card" style="padding: 1.2rem; grid-column: 1 / -1;">
          <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
            <h2 style="font-size: 1.15rem; margin: 0;">Recent interactions</h2>
            <a href="#people" data-route="people" style="font-size: 0.85rem; color: var(--action); text-decoration: none; font-weight: 600;">View all interactions →</a>
          </div>
          <div class="table-wrap">
            <table style="width: 100%; border-collapse: collapse; font-size: 0.88rem;">
              <tbody>
                <tr style="border-bottom: 1px solid var(--border);">
                  <td style="padding: 0.55rem 0.5rem; color: var(--muted); min-width: 70px;">18 Jul</td>
                  <td style="padding: 0.55rem 0.5rem;"><strong>Aisling Byrne</strong></td>
                  <td style="padding: 0.55rem 0.5rem; color: var(--muted);">Email about agenda</td>
                </tr>
                <tr style="border-bottom: 1px solid var(--border);">
                  <td style="padding: 0.55rem 0.5rem; color: var(--muted);">16 Jul</td>
                  <td style="padding: 0.55rem 0.5rem;"><strong>John Hale</strong></td>
                  <td style="padding: 0.55rem 0.5rem; color: var(--muted);">External catch-up</td>
                </tr>
                <tr>
                  <td style="padding: 0.55rem 0.5rem; color: var(--muted);">16 Jul</td>
                  <td style="padding: 0.55rem 0.5rem;"><strong>Liam Lynch</strong></td>
                  <td style="padding: 0.55rem 0.5rem; color: var(--muted);">Dietary requirement noted</td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>

      </div>
    </section>
  `;
}
