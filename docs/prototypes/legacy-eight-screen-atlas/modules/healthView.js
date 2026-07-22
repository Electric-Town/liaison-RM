/**
 * HealthView Module - Screen 06: HEALTH / RECOVERY
 */
export function renderHealthView(state = {}) {
  return `
    <section class="page" data-page="health" aria-labelledby="health-heading">
      <!-- Header -->
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 1.25rem;">
        <h1 id="health-heading" style="font-family: var(--font-title); font-size: 1.8rem; margin: 0;">Local workspace health</h1>
        <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.75rem;">•••</button>
      </div>

      <!-- Main Health Grid -->
      <div class="two-column" style="display: grid; grid-template-columns: 2fr 1fr; gap: 1.25rem; align-items: start;">
        
        <!-- Main Column: Checkpoints & Banners -->
        <div style="display: grid; gap: 1.25rem;">
          
          <!-- Card 1: Last Safe Checkpoint -->
          <div class="card" style="padding: 1.25rem;">
            <div style="display: flex; align-items: flex-start; gap: 1rem;">
              <span style="display: inline-grid; place-items: center; width: 42px; height: 42px; border-radius: 50%; background: var(--success-soft); color: var(--success); font-size: 1.2rem; font-weight: 700;">✓</span>
              <div>
                <strong style="display: block; font-size: 1.1rem;">Last safe checkpoint</strong>
                <small style="color: var(--muted); display: block; margin: 0.2rem 0 0.5rem 0;">Today, 24 Jul 2026 · 09:42</small>
                <p style="margin: 0; font-size: 0.9rem;">All local records are consistent and complete.</p>
              </div>
            </div>
          </div>

          <!-- Card 2: Recoverable Change Banner -->
          <div class="card" style="padding: 1.25rem; border-left: 4px solid var(--warning); background: var(--warning-soft);">
            <div style="display: flex; align-items: flex-start; gap: 1rem;">
              <span style="display: inline-grid; place-items: center; width: 42px; height: 42px; border-radius: 50%; background: var(--surface); color: var(--warning); font-size: 1.2rem; font-weight: 700;">⚠️</span>
              <div style="flex: 1;">
                <strong style="display: block; font-size: 1.1rem; color: var(--warning);">Recoverable change</strong>
                <small style="display: block; margin: 0.2rem 0 0.75rem 0; color: var(--ink);">Restored data available from 20 Jul 2026 · 14:18</small>
                <button id="review-recover-btn" class="primary-button" type="button" style="min-height: 40px; padding: 0.4rem 1rem; background: var(--warning); border-color: var(--warning);">Review and recover</button>
              </div>
            </div>
          </div>

          <!-- Card 3: All Good & Safety Export -->
          <div class="card" style="padding: 1.25rem;">
            <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 1rem;">
              <div style="display: flex; align-items: center; gap: 0.85rem;">
                <span style="display: inline-grid; place-items: center; width: 38px; height: 38px; border-radius: 50%; background: var(--accent-soft); color: var(--action); font-size: 1.1rem; font-weight: 700;">🔒</span>
                <div>
                  <strong style="display: block; font-size: 1.05rem;">All good</strong>
                  <p style="margin: 0.2rem 0 0 0; color: var(--muted); font-size: 0.85rem;">Your data is local and private to this device.</p>
                </div>
              </div>
              <div style="display: flex; gap: 0.6rem; flex-wrap: wrap;">
                <button id="export-safety-btn" class="secondary-button" type="button" style="min-height: 40px; font-size: 0.85rem;">Export safety copy</button>
                <button id="export-json-btn" class="secondary-button" type="button" style="min-height: 40px; font-size: 0.85rem;">Export data (JSON)</button>
              </div>
            </div>
          </div>

        </div>

        <!-- Sidebar: Audit Evidence Timeline -->
        <div class="card" style="padding: 1.2rem;">
          <h2 style="font-size: 1.05rem; margin-top: 0; border-bottom: 1px solid var(--border); padding-bottom: 0.5rem;">Audit evidence</h2>
          
          <div style="display: grid; gap: 0.75rem; font-size: 0.85rem; margin-top: 0.75rem;">
            <div style="padding-bottom: 0.6rem; border-bottom: 1px dashed var(--border);">
              <small style="color: var(--muted); display: block;">24 Jul 2026, 10:19</small>
              <strong>Integrity check</strong>
            </div>
            <div style="padding-bottom: 0.6rem; border-bottom: 1px dashed var(--border);">
              <small style="color: var(--muted); display: block;">24 Jul 2026, 10:19</small>
              <strong>Change detected</strong>
            </div>
            <div>
              <small style="color: var(--muted); display: block;">23 Jul 2026, 18:05</small>
              <strong>Backup check</strong>
            </div>
          </div>
        </div>

      </div>
    </section>
  `;
}
