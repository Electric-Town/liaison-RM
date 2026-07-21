/**
 * EventsView Module - Screen 02: EVENTS
 */
import { renderStatusChip } from './components/statusChip.js';

export function renderEventsView(state = {}) {
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
    <section class="page" data-page="events" aria-labelledby="events-heading">
      <!-- Calendar Header Controls -->
      <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem; margin-bottom: 1.25rem;">
        <div style="display: flex; align-items: center; gap: 0.5rem;">
          <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.75rem;">‹ Prev</button>
          <strong style="font-size: 1.1rem; font-weight: 700; padding: 0 0.5rem;">JUL 20–26, 2026 📅</strong>
          <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.75rem;">Next ›</button>
        </div>
        <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.85rem;">Today</button>
      </div>

      <!-- Date Strip -->
      <div class="date-strip" style="margin-bottom: 1.5rem;">
        ${dates.map(d => `
          <button class="date-pill ${d.active ? 'is-active' : ''}" type="button" aria-current="${d.active ? 'date' : 'false'}">
            <small>${d.day}</small>
            <strong>${d.date}</strong>
          </button>
        `).join('')}
      </div>

      <!-- Events Timeline List -->
      <div style="display: grid; gap: 1.25rem;">
        
        <!-- Timeline Item 1 -->
        <div style="display: grid; grid-template-columns: 70px 1fr; gap: 1rem; align-items: start;">
          <div style="text-align: right; padding-top: 0.25rem;">
            <small style="color: var(--muted); display: block;">WED</small>
            <strong style="font-size: 1.3rem;">22</strong>
            <small style="color: var(--muted); display: block;">JUL</small>
          </div>
          <div class="card" style="padding: 1.1rem;">
            <div style="display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.5rem;">
              <div>
                <h3 style="font-size: 1.15rem; margin: 0;">Team coffee</h3>
                <p style="margin: 0.25rem 0 0 0; color: var(--muted); font-size: 0.88rem;">10:00 - 10:30 · Kitchen, Building A · Floor 3</p>
              </div>
              <div style="display: flex; align-items: center; gap: 0.5rem;">
                ${renderStatusChip({ label: 'Brief started', variant: 'info' })}
                <button class="secondary-button" type="button" style="min-height: 36px; padding: 0.3rem 0.65rem; font-size: 0.82rem;">Open event ▾</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Timeline Item 2: Highlighted Readiness Action -->
        <div style="display: grid; grid-template-columns: 70px 1fr; gap: 1rem; align-items: start;">
          <div style="text-align: right; padding-top: 0.25rem; color: var(--danger);">
            <small style="display: block;">FRI</small>
            <strong style="font-size: 1.3rem;">24</strong>
            <small style="display: block;">JUL</small>
          </div>
          <div class="card" style="padding: 1.1rem; border-left: 4px solid var(--danger);">
            <div style="display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.5rem;">
              <div>
                <h3 style="font-size: 1.15rem; margin: 0;">All-hands lunch</h3>
                <p style="margin: 0.25rem 0 0 0; color: var(--muted); font-size: 0.88rem;">12:00 - 13:30 · Canteen, Building A · Floor 3</p>
              </div>
              <div style="display: flex; align-items: center; gap: 0.5rem;">
                ${renderStatusChip({ label: 'Readiness action needed', variant: 'warn' })}
                <button class="primary-button continue-readiness-btn" type="button" data-route="readiness" style="min-height: 36px; padding: 0.35rem 0.75rem; font-size: 0.85rem; background: var(--danger); border-color: var(--danger);">Continue readiness ▾</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Timeline Item 3 -->
        <div style="display: grid; grid-template-columns: 70px 1fr; gap: 1rem; align-items: start;">
          <div style="text-align: right; padding-top: 0.25rem;">
            <small style="color: var(--muted); display: block;">WED</small>
            <strong style="font-size: 1.3rem;">29</strong>
            <small style="color: var(--muted); display: block;">JUL</small>
          </div>
          <div class="card" style="padding: 1.1rem;">
            <div style="display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.5rem;">
              <div>
                <h3 style="font-size: 1.15rem; margin: 0;">Client dinner</h3>
                <p style="margin: 0.25rem 0 0 0; color: var(--muted); font-size: 0.88rem;">19:00 - 21:30 · Offsite, The Loft · Dublin 2</p>
              </div>
              <div style="display: flex; align-items: center; gap: 0.5rem;">
                ${renderStatusChip({ label: 'Details confirmed', variant: 'good' })}
                <button class="secondary-button" type="button" style="min-height: 36px; padding: 0.3rem 0.65rem; font-size: 0.82rem;">Open event ▾</button>
              </div>
            </div>
          </div>
        </div>

        <!-- Timeline Item 4 -->
        <div style="display: grid; grid-template-columns: 70px 1fr; gap: 1rem; align-items: start;">
          <div style="text-align: right; padding-top: 0.25rem;">
            <small style="color: var(--muted); display: block;">THU</small>
            <strong style="font-size: 1.3rem;">06</strong>
            <small style="color: var(--muted); display: block;">AUG</small>
          </div>
          <div class="card" style="padding: 1.1rem;">
            <div style="display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 0.5rem; margin-bottom: 0.5rem;">
              <div>
                <h3 style="font-size: 1.15rem; margin: 0;">Team offsite</h3>
                <p style="margin: 0.25rem 0 0 0; color: var(--muted); font-size: 0.88rem;">All day · Offsite, Moniusz Lodge · Wicklow</p>
              </div>
              <div style="display: flex; align-items: center; gap: 0.5rem;">
                ${renderStatusChip({ label: 'Details confirmed', variant: 'good' })}
                <button class="secondary-button" type="button" style="min-height: 36px; padding: 0.3rem 0.65rem; font-size: 0.82rem;">Open event ▾</button>
              </div>
            </div>
          </div>
        </div>

      </div>

      <!-- Floating Filter Drawer Trigger -->
      <div style="margin-top: 1.5rem; display: flex; justify-content: flex-end;">
        <button class="secondary-button" type="button" style="display: flex; align-items: center; gap: 0.4rem; min-height: 40px; padding: 0.4rem 0.85rem;">
          <span>⚙</span> Filters
        </button>
      </div>
    </section>
  `;
}
