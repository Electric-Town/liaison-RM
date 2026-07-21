/**
 * PersonView Module - Screen 05: PERSON DETAIL
 */
export function renderPersonView(state = {}) {
  const person = state.person || {
    name: 'Aisling Byrne',
    email: 'aisling.byrne@co.com',
    location: 'Dublin · Building A - Floor 3',
    phone: 'Via phone recorded',
    role: 'Operations manager',
    reportsTo: 'co.com',
    team: 'A-12'
  };

  return `
    <section class="page" data-page="person" aria-labelledby="person-heading">
      <!-- Person Profile Card Header -->
      <div class="profile-card" style="margin-bottom: 1.25rem;">
        <div style="display: flex; justify-content: space-between; align-items: flex-start; flex-wrap: wrap; gap: 1rem;">
          <div style="display: flex; align-items: center; gap: 1rem;">
            <span class="profile-avatar-large">AB</span>
            <div>
              <h1 id="person-heading" style="font-family: var(--font-title); font-size: 1.8rem; margin: 0;">
                ${person.name} <small style="font-size: 1rem; color: var(--muted); cursor: pointer;">✏️</small>
              </h1>
              <p style="margin: 0.2rem 0; color: var(--action); font-size: 0.9rem;">${person.email}</p>
              <p style="margin: 0; color: var(--muted); font-size: 0.85rem;">📍 ${person.location}</p>
            </div>
          </div>
          <button id="edit-person-btn" class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.85rem;">Edit</button>
        </div>

        <!-- Tab Bar -->
        <div class="topic-tabs" style="margin-top: 1.25rem; margin-bottom: 0;">
          <button class="topic-tab is-active" type="button">Overview</button>
          <button class="topic-tab" type="button">Notes</button>
          <button class="topic-tab" type="button">Important dates</button>
          <button class="topic-tab" type="button">Commitments</button>
          <button class="topic-tab" type="button">Workspace</button>
          <button class="topic-tab" type="button">Custom fields</button>
          <button class="topic-tab" type="button">+</button>
        </div>
      </div>

      <!-- Detail Grid: Contact Info + Activity/Working Notes -->
      <div class="two-column" style="display: grid; grid-template-columns: 1fr 2fr; gap: 1.25rem; align-items: start;">
        
        <!-- Contact Info Side Panel -->
        <div class="card" style="padding: 1.2rem;">
          <h2 style="font-size: 1.1rem; margin-top: 0; border-bottom: 1px solid var(--border); padding-bottom: 0.5rem;">Contact</h2>
          
          <dl style="margin: 0; display: grid; gap: 0.75rem; font-size: 0.88rem;">
            <div>
              <dt style="color: var(--muted); font-size: 0.78rem; text-transform: uppercase;">Email</dt>
              <dd style="margin: 0.2rem 0 0 0; font-weight: 600; color: var(--action);">${person.email}</dd>
            </div>
            <div>
              <dt style="color: var(--muted); font-size: 0.78rem; text-transform: uppercase;">Phone</dt>
              <dd style="margin: 0.2rem 0 0 0;">${person.phone}</dd>
            </div>
            <div>
              <dt style="color: var(--muted); font-size: 0.78rem; text-transform: uppercase;">Role</dt>
              <dd style="margin: 0.2rem 0 0 0;">${person.role}</dd>
            </div>
            <div>
              <dt style="color: var(--muted); font-size: 0.78rem; text-transform: uppercase;">Reports to</dt>
              <dd style="margin: 0.2rem 0 0 0;">${person.reportsTo}</dd>
            </div>
            <div>
              <dt style="color: var(--muted); font-size: 0.78rem; text-transform: uppercase;">Team</dt>
              <dd style="margin: 0.2rem 0 0 0;">${person.team}</dd>
            </div>
            <div style="border-top: 1px solid var(--border); padding-top: 0.75rem;">
              <dt style="color: var(--muted); font-size: 0.78rem; text-transform: uppercase;">Workspace</dt>
              <dd style="margin: 0.2rem 0 0 0;">Building A · Floor 3</dd>
            </div>
            <div>
              <dt style="color: var(--muted); font-size: 0.78rem; text-transform: uppercase;">Location</dt>
              <dd style="margin: 0.2rem 0 0 0;">Dublin</dd>
            </div>
            <div>
              <dt style="color: var(--muted); font-size: 0.78rem; text-transform: uppercase;">Source</dt>
              <dd style="margin: 0.2rem 0 0 0; color: var(--muted);">Event - All-hands lunch</dd>
            </div>
          </dl>
        </div>

        <!-- Main Content Area -->
        <div style="display: grid; gap: 1.25rem;">
          
          <!-- Recent Interactions -->
          <div class="card" style="padding: 1.2rem;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
              <h2 style="font-size: 1.1rem; margin: 0;">Recent interactions</h2>
              <a href="#" style="font-size: 0.85rem; color: var(--action); text-decoration: none;">View full timeline →</a>
            </div>
            <div style="display: grid; gap: 0.6rem; font-size: 0.88rem;">
              <div style="display: flex; gap: 0.75rem; padding-bottom: 0.5rem; border-bottom: 1px dashed var(--border);">
                <span style="color: var(--muted); min-width: 75px;">18 Jul 2026</span>
                <span>Email about agenda</span>
              </div>
              <div style="display: flex; gap: 0.75rem; padding-bottom: 0.5rem; border-bottom: 1px dashed var(--border);">
                <span style="color: var(--muted); min-width: 75px;">16 Jul 2026</span>
                <span>Dietary requirement noted</span>
              </div>
              <div style="display: flex; gap: 0.75rem;">
                <span style="color: var(--muted); min-width: 75px;">02 Jul 2026</span>
                <span>Menu preference</span>
              </div>
            </div>
          </div>

          <!-- Working With Me -->
          <div class="card" style="padding: 1.2rem;">
            <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.75rem;">
              <h2 style="font-size: 1.1rem; margin: 0;">Working with me</h2>
              <button class="secondary-button" type="button" style="min-height: 32px; padding: 0.2rem 0.6rem; font-size: 0.8rem;">Edit</button>
            </div>
            <ul style="margin: 0; padding-left: 1.2rem; font-size: 0.88rem; display: grid; gap: 0.4rem; color: var(--ink);">
              <li>Detail-oriented and pragmatic.</li>
              <li>Prefers clear plans and early decisions.</li>
              <li><strong>Beef allergy.</strong> Warnings.</li>
              <li>How I prefer to work: Email with agenda in advance.</li>
            </ul>
          </div>

          <!-- Important Dates & Commitments Grid -->
          <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 1rem;">
            
            <div class="card" style="padding: 1rem;">
              <h3 style="font-size: 1rem; margin-top: 0;">Important dates</h3>
              <div style="font-size: 0.85rem; display: grid; gap: 0.5rem;">
                <div>
                  <strong style="display: block;">25 Jul</strong>
                  <span style="color: var(--muted);">Birthday</span>
                </div>
                <div>
                  <strong style="display: block;">01 Sep</strong>
                  <span style="color: var(--muted);">Work anniversary</span>
                </div>
              </div>
            </div>

            <div class="card" style="padding: 1rem;">
              <h3 style="font-size: 1rem; margin-top: 0;">Commitments</h3>
              <div style="font-size: 0.85rem; display: grid; gap: 0.4rem;">
                <label style="display: flex; align-items: center; gap: 0.4rem;">
                  <input type="checkbox">
                  <span>Confirm venue shortlist</span>
                </label>
                <label style="display: flex; align-items: center; gap: 0.4rem;">
                  <input type="checkbox">
                  <span>Confirm menu choices</span>
                </label>
                <label style="display: flex; align-items: center; gap: 0.4rem;">
                  <input type="checkbox" checked>
                  <span style="text-decoration: line-through; color: var(--muted);">Email about agenda</span>
                </label>
              </div>
            </div>

          </div>

        </div>

      </div>
    </section>
  `;
}
