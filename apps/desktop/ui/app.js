// Liaison RM Desktop Application Bridge & Navigation Module
const state = {
  workspace: null,
  people: [],
  currentRoute: "today",
  theme: "light",
  currentEditingField: null,
  selectedAttendee: "LL",
  productState: {
    app: {
      product_state: "B0 Workplace Review Alpha",
      connection_state: "Local-only · Airgap ready",
      release_evidence: "P03D Editorial Ledger UI",
    },
  },
};

const attendeeData = {
  AB: {
    initials: "AB",
    name: "Aisling Byrne",
    status: "Confirmed 09 Jul",
    dietary: "No beef",
    disclosure: "Instruction available",
    readiness: "Ready",
    warning: "None · All dietary instructions verified",
    source: "SOURCE · DIRECT USER ENTERED · 09 JUL 2026",
    request: "No active info request",
    resolution: "Verified & ready for catering brief",
  },
  LL: {
    initials: "LL",
    name: "Liam Lynch",
    status: "Pending",
    dietary: "Not recorded",
    disclosure: "None",
    readiness: "Action needed",
    warning: "Dietary information is not recorded",
    source: "No source recorded",
    request: "Ask Liam only what catering needs to know (Offline Template)",
    resolution: "Awaiting response",
  },
  JH: {
    initials: "JH",
    name: "John Hale",
    status: "Confirmed 02 Jul",
    dietary: "No shellfish",
    disclosure: "Instruction available",
    readiness: "Ready",
    warning: "None · Shellfish restriction noted",
    source: "SOURCE · USER ENTERED · 02 JUL 2026",
    request: "No active info request",
    resolution: "Verified & ready for catering brief",
  },
  AC: {
    initials: "AC",
    name: "Adriana Cerny",
    status: "Confirmed 16 Jun",
    dietary: "Gluten-free",
    disclosure: "Instruction available",
    readiness: "Ready",
    warning: "None · Gluten-free restriction noted",
    source: "SOURCE · USER ENTERED · 16 JUN 2026",
    request: "No active info request",
    resolution: "Verified & ready for catering brief",
  },
};

const nativeOperation = {
  generation: 0,
  active: null,
  restartRequired: false,
};
const APPLICATION_CONTRACT_VERSION = 1;

const byId = (id) => document.getElementById(id);

const invoke = async (command, payload = {}) => {
  const tauriInvoke = window.__TAURI__?.core?.invoke;
  if (typeof tauriInvoke !== "function") {
    return { contract_version: 1, value: null };
  }
  return tauriInvoke(command, payload);
};

const commandValue = (result) => {
  if (
    !result ||
    typeof result !== "object" ||
    result.contract_version !== APPLICATION_CONTRACT_VERSION ||
    !("value" in result)
  ) {
    throw new Error("The native Liaison bridge returned an unexpected result.");
  }
  return result.value;
};

const invokeValue = async (command, payload = {}) =>
  commandValue(await invoke(command, payload));

const status = (message) => {
  const statusEl = byId("live-status");
  if (statusEl) {
    statusEl.textContent = message;
  }
};

const errorText = (error) => {
  if (
    error &&
    typeof error === "object" &&
    "code" in error &&
    error.contract_version !== APPLICATION_CONTRACT_VERSION
  ) {
    return "The native Liaison bridge returned an incompatible error contract. Recovery: update or reinstall one matching Liaison RM build before retrying.";
  }
  const message =
    typeof error?.message === "string" && error.message.trim()
      ? error.message.trim()
      : error instanceof Error
      ? error.message
      : "The operation did not complete.";
  const recovery =
    typeof error?.recovery === "string" && error.recovery.trim()
      ? error.recovery.trim()
      : "Review the workspace selection and retry.";
  return `${message} Recovery: ${recovery}`;
};

const currentSessionId = () => state.workspace?.session_id || null;

const isCurrentOperation = (operation) =>
  nativeOperation.active?.generation === operation.generation;

const operationOwnsCurrentSession = (operation) =>
  isCurrentOperation(operation) &&
  currentSessionId() === operation.sessionId;

const executeNativeSessionCommand = async (commandName, payload = {}) => {
  const operation = { generation: 1, sessionId: currentSessionId() };
  if (!operationOwnsCurrentSession(operation)) return null;
  return invokeValue(commandName, {
    request: { sessionId: operation.sessionId, ...payload },
  });
};

const renderTextNode = (container, text) => {
  if (!container) return;
  const span = document.createElement("span");
  span.textContent = text;
  container.replaceChildren(span);
};

const applyTheme = (theme) => {
  state.theme = theme;
  let effectiveTheme = theme;
  if (theme === "system") {
    const prefersDark = window.matchMedia(
      "(prefers-color-scheme: dark)"
    ).matches;
    effectiveTheme = prefersDark ? "dark" : "light";
  }

  document.documentElement.setAttribute("data-theme", effectiveTheme);

  document.querySelectorAll(".theme-card").forEach((card) => {
    const isMatch = card.dataset.theme === theme;
    card.classList.toggle("is-active", isMatch);
    card.setAttribute("aria-checked", String(isMatch));
  });

  const iconEl = byId("theme-toggle-icon");
  const textEl = byId("theme-toggle-text");
  const isDark =
    effectiveTheme === "dark" ||
    effectiveTheme === "nordic" ||
    effectiveTheme === "emerald" ||
    effectiveTheme === "high-contrast";
  if (iconEl) iconEl.textContent = isDark ? "☀️" : "🌙";
  if (textEl) textEl.textContent = isDark ? "Light Mode" : "Dark Mode";
};

const navigate = (route) => {
  state.currentRoute = route;

  document.querySelectorAll(".page").forEach((section) => {
    const active = section.dataset.page === route;
    section.hidden = !active;
  });

  document.querySelectorAll(".nav-button, [data-route]").forEach((button) => {
    const active = button.dataset.route === route;
    button.classList.toggle("is-active", active);
    if (active) button.setAttribute("aria-current", "page");
    else button.removeAttribute("aria-current");
  });

  window.scrollTo({ top: 0, behavior: "instant" });
};

const enableWorkspaceControls = (enabled) => {
  const personName = byId("person-name");
  const personEmail = byId("person-email");
  const personLoc = byId("person-location");
  const createPerson = byId("create-person");
  const useLoc = byId("add-person-use-location");
  const warning = byId("people-workspace-warning");

  if (personName) personName.disabled = !enabled;
  if (personEmail) personEmail.disabled = !enabled;
  if (personLoc) personLoc.disabled = !enabled;
  if (createPerson) createPerson.disabled = !enabled;
  if (useLoc) useLoc.disabled = !enabled;

  if (warning) {
    warning.style.display = enabled ? "none" : "block";
  }
};

const refreshPeopleList = async () => {
  if (!state.workspace?.session_id) {
    state.people = [];
    renderPeopleTable();
    return;
  }
  try {
    const operation = { generation: nativeOperation.generation, sessionId: currentSessionId() };
    const people = await executeNativeSessionCommand("list_people");
    if (!isCurrentOperation(operation)) return;
    state.people = Array.isArray(people) ? people : [];
    renderPeopleTable();
    updateWorkspaceSummary();
  } catch (err) {
    status(errorText(err));
  }
};

const updateWorkspaceSummary = () => {
  const summary = byId("workspace-summary");
  if (!summary) return;

  if (!state.workspace) {
    summary.replaceChildren(
      createSummaryRow("Status", "None selected"),
      createSummaryRow("Profile", "—"),
      createSummaryRow("People", "—")
    );
    return;
  }

  const manifest = state.workspace.manifest || {};
  summary.replaceChildren(
    createSummaryRow("Status", "Active Workspace"),
    createSummaryRow("Name", manifest.name || "Local Workspace"),
    createSummaryRow("Profile", manifest.profile || "Workplace"),
    createSummaryRow("People Count", `${state.people.length} profiles`),
    createSummaryRow("Path", state.workspace.path || "Local")
  );

  const authLabel = byId("authority-label");
  if (authLabel) {
    authLabel.textContent = `Local Workspace · ${manifest.name || "Active"}`;
  }
};

const createSummaryRow = (dtText, ddText) => {
  const div = document.createElement("div");
  const dt = document.createElement("dt");
  dt.textContent = dtText;
  const dd = document.createElement("dd");
  dd.textContent = ddText;
  div.appendChild(dt);
  div.appendChild(dd);
  return div;
};

const renderPeopleTable = () => {
  const tbody = byId("people-table-body");
  if (!tbody) return;

  if (!state.workspace) {
    const tr = document.createElement("tr");
    const td = document.createElement("td");
    td.colSpan = 3;
    td.className = "text-center muted";
    td.style.padding = "1.5rem";
    td.textContent = "No workspace open. Create or open a local workspace above.";
    tr.appendChild(td);
    tbody.replaceChildren(tr);
    return;
  }

  if (state.people.length === 0) {
    const tr = document.createElement("tr");
    const td = document.createElement("td");
    td.colSpan = 3;
    td.className = "text-center muted";
    td.style.padding = "1.5rem";
    td.textContent = "No profiles in this workspace yet. Add a person using the form on the left.";
    tr.appendChild(td);
    tbody.replaceChildren(tr);
    return;
  }

  const rows = state.people.map((person) => {
    const tr = document.createElement("tr");
    tr.className = "person-row-clickable";
    tr.style.borderBottom = "1px solid var(--border)";
    tr.style.cursor = "pointer";
    tr.addEventListener("click", () => showPersonDetail(person));

    const tdName = document.createElement("td");
    tdName.style.padding = "0.65rem 0.8rem";
    const strong = document.createElement("strong");
    const initials = (person.display_name || "P").split(" ").map(n => n[0]).join("").slice(0, 2).toUpperCase();
    strong.textContent = `${initials} ${person.display_name}`;
    tdName.appendChild(strong);

    const tdEmail = document.createElement("td");
    tdEmail.style.padding = "0.65rem 0.8rem";
    tdEmail.textContent = person.emails?.[0]?.address || "Markdown profile";

    const tdRev = document.createElement("td");
    tdRev.style.padding = "0.65rem 0.8rem";
    tdRev.style.color = "var(--muted)";
    tdRev.textContent = `Revision ${person.revision || 1}`;

    tr.appendChild(tdName);
    tr.appendChild(tdEmail);
    tr.appendChild(tdRev);
    return tr;
  });

  tbody.replaceChildren(...rows);
};

const showPersonDetail = (person) => {
  if (!person) return;
  const nameHeading = byId("person-heading");
  const fieldName = byId("field-fullname");
  const fieldEmail = byId("field-email");
  const editHeading = byId("edit-person-heading");

  if (nameHeading) nameHeading.textContent = person.display_name;
  if (fieldName) fieldName.textContent = person.display_name;
  if (fieldEmail) fieldEmail.textContent = person.emails?.[0]?.address || "Not set";
  if (editHeading) editHeading.textContent = `Edit ${person.display_name}`;

  status(`Viewing canonical profile: ${person.display_name} (Rev ${person.revision || 1})`);
  navigate("person");
};

const updateDrawer = (key) => {
  state.selectedAttendee = key;
  const data = attendeeData[key] || attendeeData.LL;

  document.querySelectorAll(".attendee-row").forEach((row) => {
    const match = row.dataset.attendee === key;
    row.classList.toggle("is-selected", match);
  });

  if (byId("drawer-avatar")) byId("drawer-avatar").textContent = data.initials;
  if (byId("drawer-title")) byId("drawer-title").textContent = data.name;
  if (byId("drawer-status-text")) byId("drawer-status-text").textContent = data.status;
  if (byId("drawer-warning-reason")) byId("drawer-warning-reason").textContent = data.warning;
  if (byId("drawer-source-text")) byId("drawer-source-text").textContent = data.source;
  if (byId("drawer-request-text")) byId("drawer-request-text").textContent = data.request;
  if (byId("drawer-resolution-text")) byId("drawer-resolution-text").textContent = data.resolution;

  const drawer = byId("attendee-drawer");
  if (drawer) drawer.hidden = false;
};

const bindEvents = () => {
  // Navigation routes
  document.querySelectorAll("[data-route]").forEach((button) => {
    button.addEventListener("click", () => navigate(button.dataset.route));
  });

  document.querySelectorAll(".continue-readiness-btn").forEach((button) => {
    button.addEventListener("click", () => navigate("readiness"));
  });

  document.querySelectorAll(".attendee-row").forEach((row) => {
    row.addEventListener("click", () => {
      updateDrawer(row.dataset.attendee);
    });
  });

  // Use Default Path button
  byId("use-default-path")?.addEventListener("click", async () => {
    try {
      const path = await invokeValue("default_workspace_path");
      if (byId("workspace-path")) byId("workspace-path").value = path;
      status(`Set workspace path to Documents: ${path}`);
    } catch (err) {
      status(errorText(err));
    }
  });

  // Create Workspace Form Submit
  byId("workspace-form")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    const path = byId("workspace-path")?.value.trim();
    const name = byId("workspace-name")?.value.trim() || "Workplace review";
    const profile = byId("workspace-profile")?.value || "workplace";
    if (!path) {
      status("Specify an absolute folder path for the workspace.");
      return;
    }

    try {
      status(`Initialising workspace at ${path}...`);
      const workspace = await invokeValue("initialise_workspace", {
        request: { path, name, profile }
      });
      state.workspace = workspace;
      nativeOperation.generation++;
      nativeOperation.active = { generation: nativeOperation.generation };

      enableWorkspaceControls(true);
      await refreshPeopleList();
      status(`Created local workspace "${workspace.manifest?.name || name}" at ${path}.`);
      navigate("people");
    } catch (err) {
      status(errorText(err));
    }
  });

  // Open Workspace Button
  byId("open-workspace")?.addEventListener("click", async () => {
    const path = byId("workspace-path")?.value.trim();
    if (!path) {
      status("Enter an absolute workspace folder path above to open.");
      return;
    }
    try {
      status(`Opening workspace at ${path}...`);
      const workspace = await invokeValue("open_workspace", { path });
      state.workspace = workspace;
      nativeOperation.generation++;
      nativeOperation.active = { generation: nativeOperation.generation };

      enableWorkspaceControls(true);
      await refreshPeopleList();
      status(`Opened workspace "${workspace.manifest?.name || 'Local Workspace'}" at ${path}.`);
      navigate("people");
    } catch (err) {
      status(errorText(err));
    }
  });

  // Inspect Workspace Health Button
  byId("inspect-workspace-health")?.addEventListener("click", async () => {
    const path = byId("workspace-path")?.value.trim();
    if (!path) {
      status("Enter a workspace folder path above to inspect health.");
      return;
    }
    try {
      status(`Inspecting read-only Health at ${path}...`);
      const result = await invokeValue("inspect_workspace_health", { path });
      status(`Health check complete: Manifest valid=${result.manifest_valid}, Profiles=${result.people_count}, Errors=${result.errors?.length || 0}`);
    } catch (err) {
      status(errorText(err));
    }
  });

  // Create Person Form Submit
  byId("person-form")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    if (!state.workspace?.session_id) {
      status("Open or create a workspace before adding people.");
      return;
    }
    const nameInput = byId("person-name");
    const emailInput = byId("person-email");
    const displayName = nameInput?.value.trim();
    const email = emailInput?.value.trim() || null;
    if (!displayName) return;

    try {
      status(`Creating canonical Markdown profile for ${displayName}...`);
      const created = await executeNativeSessionCommand("create_person", {
        displayName: displayName,
        email: email,
      });
      if (nameInput) nameInput.value = "";
      if (emailInput) emailInput.value = "";
      await refreshPeopleList();
      status(`Saved canonical Markdown profile for ${displayName}.`);
    } catch (err) {
      status(errorText(err));
    }
  });

  // Refresh People Button
  byId("refresh-people")?.addEventListener("click", async () => {
    await refreshPeopleList();
    status("Refreshed workspace directory from local Markdown files.");
  });

  // Drawer Close Button
  const closeDrawerBtn = byId("close-drawer-btn");
  if (closeDrawerBtn) {
    closeDrawerBtn.addEventListener("click", () => {
      const drawer = byId("attendee-drawer");
      if (drawer) drawer.hidden = true;
    });
  }

  // Themes
  document.querySelectorAll(".theme-card").forEach((card) => {
    card.addEventListener("click", () => {
      applyTheme(card.dataset.theme);
    });
  });

  const topbarThemeBtn = byId("topbar-theme-toggle");
  if (topbarThemeBtn) {
    topbarThemeBtn.addEventListener("click", () => {
      const nextTheme = state.theme === "light" ? "dark" : "light";
      applyTheme(nextTheme);
    });
  }
};

document.addEventListener("DOMContentLoaded", async () => {
  applyTheme("light");
  bindEvents();
  updateDrawer("LL");
  enableWorkspaceControls(false);

  // Auto-fetch default workspace path from Rust
  try {
    const path = await invokeValue("default_workspace_path");
    if (byId("workspace-path")) byId("workspace-path").value = path;
  } catch (e) {
    // Graceful fallback if native bridge offline
  }

  // Probe App Status from Rust backend
  try {
    const app = await invokeValue("app_status");
    status(`Liaison RM ${app.product_state} ready. ${app.connection_state} · ${app.release_evidence}`);
  } catch (e) {
    status(`Liaison RM ${state.productState.app.product_state} ready. ${state.productState.app.connection_state} · ${state.productState.app.release_evidence}`);
  }
});
