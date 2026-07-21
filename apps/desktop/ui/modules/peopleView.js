/**
 * PeopleView Module - Screen 04: PEOPLE / DIRECTORY
 */
export function renderPeopleView(state = {}) {
  const people = [
    { initials: 'AB', name: 'Aisling Byrne', email: 'aisling.byrne@co.com', location: 'Building A · Floor 3', lastInteraction: '18 Jul 2026 · Email' },
    { initials: 'LL', name: 'Liam Lynch', email: 'liam.lynch@co.com', location: 'Building A · Floor 3', lastInteraction: '16 Jul 2026 · Dietary noted' },
    { initials: 'JH', name: 'John Hale', email: 'john.hale@external.co', location: 'External co', lastInteraction: '02 Jul 2026 · External catch-up' },
    { initials: 'AC', name: 'Adriana Cerry', email: 'adriana@acdp.com', location: 'acdp.com', lastInteraction: '18 Jun 2026 · Menu preference' },
    { initials: 'DM', name: 'Declan Murphy', email: 'declan.murphy@co.com', location: 'co.com', lastInteraction: '16 Jul 2026 · Email' },
    { initials: 'NS', name: 'Nora Shah', email: 'nora.shah@co.com', location: 'co.com', lastInteraction: '05 Jul 2026 · Outlook' }
  ];

  return `
    <section class="page" data-page="people" aria-labelledby="people-heading">
      <!-- Search & Quick Action Toolbar -->
      <div style="display: flex; gap: 0.75rem; align-items: center; flex-wrap: wrap; margin-bottom: 1.25rem;">
        <div style="flex: 1; min-width: 260px; position: relative;">
          <input id="directory-search-input" type="search" placeholder="Search people by name, email, or organisation..." style="width: 100%; min-height: 44px; padding: 0.4rem 0.85rem; border: 1px solid var(--border); border-radius: 0.35rem; background: var(--surface); color: var(--ink);">
        </div>
        <button id="add-person-btn" class="primary-button" type="button" style="min-height: 44px; padding: 0.4rem 1rem;">+ Add person</button>
        <button class="secondary-button" type="button" style="min-height: 44px;">Columns ▾</button>
        <button class="secondary-button" type="button" style="min-height: 44px;">Advanced filters</button>
        <button class="secondary-button" type="button" style="min-height: 44px;">Filter by CSV ▾</button>
        <button class="secondary-button" type="button" style="min-height: 44px;">Preview</button>
      </div>

      <!-- Main Directory Section -->
      <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.75rem; margin-bottom: 1rem;">
        <h1 id="people-heading" style="font-family: var(--font-title); font-size: 1.6rem; margin: 0;">
          All people <small style="font-family: var(--font-body); font-size: 1rem; color: var(--muted); font-weight: 400;">(214 people)</small>
        </h1>
        <div style="display: flex; gap: 0.5rem;">
          <button id="import-data-btn" class="secondary-button" type="button" style="min-height: 38px; padding: 0.35rem 0.75rem; font-size: 0.85rem;">Import data ▾</button>
          <button id="export-data-btn" class="secondary-button" type="button" style="min-height: 38px; padding: 0.35rem 0.75rem; font-size: 0.85rem;">Export ▾</button>
          <button class="secondary-button" type="button" style="min-height: 38px; padding: 0.35rem 0.75rem; font-size: 0.85rem;">Saved views ▾</button>
        </div>
      </div>

      <!-- Layout: Table + Side Batch Operations Card -->
      <div class="people-layout" style="display: grid; grid-template-columns: 3fr 1fr; gap: 1.25rem; align-items: start;">
        
        <!-- Table -->
        <div class="card" style="padding: 1rem;">
          <div class="table-wrap">
            <table style="width: 100%; border-collapse: collapse; font-size: 0.88rem;">
              <thead>
                <tr style="border-bottom: 2px solid var(--border); text-align: left; font-size: 0.78rem; text-transform: uppercase; color: var(--muted);">
                  <th style="padding: 0.55rem 0.5rem;">Person</th>
                  <th style="padding: 0.55rem 0.5rem;">Organisation / Location</th>
                  <th style="padding: 0.55rem 0.5rem;">Last interaction</th>
                </tr>
              </thead>
              <tbody>
                ${people.map(p => `
                  <tr class="person-row-clickable" data-person="${p.name}" style="border-bottom: 1px solid var(--border); cursor: pointer;">
                    <td style="padding: 0.75rem 0.5rem;">
                      <div style="display: flex; align-items: center; gap: 0.6rem;">
                        <span style="display: inline-grid; place-items: center; width: 32px; height: 32px; border-radius: 50%; background: var(--highlight); font-weight: 700; font-size: 0.8rem; color: var(--content-on-highlight);">${p.initials}</span>
                        <div>
                          <strong style="display: block; color: var(--action);">${p.name}</strong>
                          <small style="color: var(--muted); font-size: 0.78rem;">${p.email}</small>
                        </div>
                      </div>
                    </td>
                    <td style="padding: 0.75rem 0.5rem; color: var(--muted);">${p.location}</td>
                    <td style="padding: 0.75rem 0.5rem; color: var(--muted);">${p.lastInteraction}</td>
                  </tr>
                `).join('')}
              </tbody>
            </table>
          </div>

          <!-- Pagination Footer -->
          <div style="display: flex; justify-content: space-between; align-items: center; margin-top: 1rem; padding-top: 0.75rem; border-top: 1px solid var(--border); font-size: 0.85rem; color: var(--muted); flex-wrap: wrap; gap: 0.5rem;">
            <span>Showing 1-6 of 214</span>
            <div style="display: flex; gap: 0.35rem; align-items: center;">
              <button class="secondary-button" type="button" style="min-height: 32px; padding: 0.2rem 0.5rem;">‹</button>
              <button class="primary-button" type="button" style="min-height: 32px; padding: 0.2rem 0.65rem;">1</button>
              <button class="secondary-button" type="button" style="min-height: 32px; padding: 0.2rem 0.65rem;">2</button>
              <button class="secondary-button" type="button" style="min-height: 32px; padding: 0.2rem 0.65rem;">3</button>
              <span>...</span>
              <button class="secondary-button" type="button" style="min-height: 32px; padding: 0.2rem 0.65rem;">36</button>
              <button class="secondary-button" type="button" style="min-height: 32px; padding: 0.2rem 0.5rem;">›</button>
            </div>
          </div>
        </div>

        <!-- Sidebar Batch Operations Card -->
        <div style="display: grid; gap: 1rem;">
          <div class="card" style="padding: 1.1rem;">
            <h3 style="font-size: 1rem; margin-top: 0;">Batch Operations</h3>
            <div style="display: grid; gap: 0.75rem; margin-top: 0.75rem;">
              <div style="border: 1px solid var(--border); padding: 0.75rem; border-radius: 0.35rem; background: var(--surface);">
                <strong style="display: block; font-size: 0.88rem;">Update existing records</strong>
                <p style="margin: 0.2rem 0 0 0; font-size: 0.8rem; color: var(--muted);">Update details for existing people.</p>
              </div>
              <div style="border: 1px solid var(--border); padding: 0.75rem; border-radius: 0.35rem; background: var(--surface);">
                <strong style="display: block; font-size: 0.88rem;">Join duplicates</strong>
                <p style="margin: 0.2rem 0 0 0; font-size: 0.8rem; color: var(--muted);">Combine records for the same person.</p>
              </div>
              <div style="border: 1px solid var(--border); padding: 0.75rem; border-radius: 0.35rem; background: var(--surface);">
                <strong style="display: block; font-size: 0.88rem;">Merge contacts</strong>
                <p style="margin: 0.2rem 0 0 0; font-size: 0.8rem; color: var(--muted);">Review and merge selected contacts.</p>
              </div>
            </div>
          </div>
        </div>

      </div>
    </section>
  `;
}
