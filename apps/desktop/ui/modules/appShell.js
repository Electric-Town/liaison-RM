/**
 * AppShell Module - Renders Navigation Rail & Brand Header
 */
export function renderAppShell(activeRoute = 'today', workspaceName = 'All-hands lunch 24 Jul 2026') {
  const navItems = [
    { key: 'today', label: 'Today', icon: '1' },
    { key: 'people', label: 'People', icon: '2' },
    { key: 'events', label: 'Events', icon: '3' },
    { key: 'health', label: 'Health', icon: '4' },
    { key: 'settings', label: 'Settings', icon: '5' }
  ];

  return `
    <header class="topbar">
      <div class="brand" aria-label="Liaison RM">
        <span class="brand-mark" aria-hidden="true">LR</span>
        <span>
          <strong>Liaison RM</strong>
          <small>Local relationship memory</small>
        </span>
      </div>
      <div style="display:flex; align-items:center; gap:0.75rem;">
        <button id="topbar-theme-toggle" class="secondary-button" type="button" title="Quick toggle Light/Dark Theme" style="min-height:38px; padding:0.3rem 0.65rem; display:flex; align-items:center; gap:0.4rem;">
          <span id="theme-toggle-icon">🌙</span> <span id="theme-toggle-text">Theme</span>
        </button>
        <div class="local-status" aria-label="Storage status">
          <span class="status-dot" aria-hidden="true"></span>
          <span id="authority-label">Local Workspace · Live</span>
        </div>
      </div>
    </header>

    <aside class="sidebar" aria-label="Primary navigation">
      <nav>
        ${navItems.map(item => `
          <button class="nav-button ${activeRoute === item.key ? 'is-active' : ''}" type="button" data-route="${item.key}" aria-current="${activeRoute === item.key ? 'page' : 'false'}">
            <span aria-hidden="true">${item.icon}</span> ${item.label}
          </button>
        `).join('')}
      </nav>
      
      <div class="sidebar-note" style="margin-top: auto; padding: 0.85rem; border: 1px solid var(--border); border-radius: 0.35rem; background: var(--surface); font-size: 0.82rem;">
        <div style="display: flex; align-items: center; gap: 0.4rem; color: var(--action); margin-bottom: 0.2rem;">
          <span style="font-size: 0.9rem;">📍</span> <strong>Local workspace</strong>
        </div>
        <p style="margin: 0; color: var(--muted); font-size: 0.78rem; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">${workspaceName}</p>
        <div style="display: flex; align-items: center; gap: 0.35rem; margin-top: 0.4rem; color: var(--success); font-size: 0.75rem; font-weight: 700;">
          <span style="width: 6px; height: 6px; border-radius: 50%; background: currentColor;"></span> Live
        </div>
      </div>
    </aside>
  `;
}
