/**
 * SettingsView Module - Screen 07: SETTINGS
 */
export function renderSettingsView(state = {}) {
  const currentTheme = state.theme || 'paper';

  return `
    <section class="page" data-page="settings" aria-labelledby="settings-heading">
      <h1 id="settings-heading" style="font-family: var(--font-title); font-size: 1.8rem; margin: 0 0 1rem 0;">Settings</h1>

      <!-- Settings Sub-Navigation Tabs -->
      <div class="topic-tabs" style="margin-bottom: 1.25rem;">
        <button class="topic-tab is-active" type="button">Appearance</button>
        <button class="topic-tab" type="button">Profile tabs</button>
        <button class="topic-tab" type="button">Custom fields</button>
        <button class="topic-tab" type="button">Portable</button>
        <button class="topic-tab" type="button">Data & backup</button>
        <button class="topic-tab" type="button">Accessibility</button>
      </div>

      <!-- Settings Content Layout -->
      <div class="two-column" style="display: grid; grid-template-columns: 2fr 1fr; gap: 1.25rem; align-items: start;">
        
        <!-- Main Panel: Theme & Typography Preferences -->
        <div class="card" style="padding: 1.25rem;">
          
          <!-- Theme Picker Section -->
          <div style="margin-bottom: 1.5rem;">
            <h2 style="font-size: 1.15rem; margin-top: 0;">Theme</h2>
            <div class="theme-grid">
              <div class="theme-card ${currentTheme === 'paper' || currentTheme === 'light' ? 'is-active' : ''}" data-theme="paper" tabindex="0" role="radio" aria-checked="${currentTheme === 'paper' || currentTheme === 'light'}">
                <div class="theme-swatch">
                  <span style="background: #eee8dc;"></span>
                  <span style="background: #fffefb;"></span>
                  <span style="background: #1e5d73;"></span>
                  <span style="background: #f2d98d;"></span>
                </div>
                <strong>Paper</strong>
              </div>

              <div class="theme-card ${currentTheme === 'high-contrast' ? 'is-active' : ''}" data-theme="high-contrast" tabindex="0" role="radio" aria-checked="${currentTheme === 'high-contrast'}">
                <div class="theme-swatch">
                  <span style="background: #000000;"></span>
                  <span style="background: #ffffff;"></span>
                  <span style="background: #00d4ff;"></span>
                  <span style="background: #ffd400;"></span>
                </div>
                <strong>High contrast</strong>
              </div>

              <div class="theme-card ${currentTheme === 'night' || currentTheme === 'dark' ? 'is-active' : ''}" data-theme="night" tabindex="0" role="radio" aria-checked="${currentTheme === 'night' || currentTheme === 'dark'}">
                <div class="theme-swatch">
                  <span style="background: #0e1714;"></span>
                  <span style="background: #16231e;"></span>
                  <span style="background: #78c8e0;"></span>
                  <span style="background: #514819;"></span>
                </div>
                <strong>Night</strong>
              </div>
            </div>
          </div>

          <!-- Text Size Section -->
          <div style="margin-bottom: 1.5rem; border-top: 1px solid var(--border); padding-top: 1.25rem;">
            <h2 style="font-size: 1.15rem; margin-top: 0;">Text size</h2>
            <div style="display: flex; gap: 0.75rem; flex-wrap: wrap;">
              <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.85rem; font-size: 0.9rem;">Aa Standard</button>
              <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.85rem; font-size: 1rem;">Aa Large</button>
              <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.85rem; font-size: 1.1rem;">Aa Extra large</button>
            </div>
          </div>

          <!-- Reduce Motion Switch -->
          <div style="margin-bottom: 1.5rem; border-top: 1px solid var(--border); padding-top: 1.25rem;">
            <div style="display: flex; justify-content: space-between; align-items: center; flex-wrap: wrap; gap: 0.5rem;">
              <div>
                <strong style="display: block; font-size: 1rem;">Reduce motion</strong>
                <small style="color: var(--muted);">Minimise animations and transitions.</small>
              </div>
              <input type="checkbox" id="reduce-motion-checkbox" style="width: 24px; height: 24px; cursor: pointer;">
            </div>
          </div>

          <!-- Interface Density Section -->
          <div style="border-top: 1px solid var(--border); padding-top: 1.25rem;">
            <h2 style="font-size: 1.15rem; margin-top: 0;">Density</h2>
            <p style="margin: 0 0 0.75rem 0; font-size: 0.85rem; color: var(--muted);">Choose how much information to show.</p>
            <div style="display: flex; gap: 0.75rem; flex-wrap: wrap;">
              <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.85rem;">Comfortable</button>
              <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.85rem;">Compact</button>
              <button class="secondary-button" type="button" style="min-height: 40px; padding: 0.4rem 0.85rem;">Dense</button>
            </div>
          </div>

        </div>

        <!-- Sidebar: Local Backup & Export Card -->
        <div class="card" style="padding: 1.25rem;">
          <h2 style="font-size: 1.1rem; margin-top: 0;">Local backup</h2>
          <p style="margin: 0 0 1rem 0; font-size: 0.85rem; color: var(--muted);">
            Your data stays on this device. On another device:
          </p>

          <div style="display: grid; gap: 0.75rem;">
            <button id="export-settings-bundle-btn" class="primary-button full-width" type="button" style="min-height: 42px; font-size: 0.88rem;">
              Export settings bundle
            </button>
            <button id="import-settings-bundle-btn" class="secondary-button full-width" type="button" style="min-height: 42px; font-size: 0.88rem;">
              Import settings bundle
            </button>
          </div>

          <div style="margin-top: 1.25rem; border-top: 1px solid var(--border); padding-top: 0.85rem; font-size: 0.8rem; color: var(--muted);">
            <span>Optional provider sync: Sync with a provider is optional.</span>
          </div>
        </div>

      </div>
    </section>
  `;
}
