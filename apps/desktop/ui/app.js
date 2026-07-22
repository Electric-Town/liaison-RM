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
  document.querySelectorAll("[data-route]").forEach((button) => {
    button.addEventListener("click", () => navigate(button.dataset.route));
  });

  document.querySelectorAll(".continue-readiness-btn").forEach((button) => {
    button.addEventListener("click", () => navigate("readiness"));
  });

  document.querySelectorAll(".person-row-clickable").forEach((row) => {
    row.addEventListener("click", () => {
      navigate("person");
    });
  });

  document.querySelectorAll(".attendee-row").forEach((row) => {
    row.addEventListener("click", () => {
      updateDrawer(row.dataset.attendee);
    });
  });

  const closeDrawerBtn = byId("close-drawer-btn");
  if (closeDrawerBtn) {
    closeDrawerBtn.addEventListener("click", () => {
      const drawer = byId("attendee-drawer");
      if (drawer) drawer.hidden = true;
    });
  }

  const recordResponseBtn = byId("drawer-record-response-btn");
  if (recordResponseBtn) {
    recordResponseBtn.addEventListener("click", () => {
      const dialog = byId("record-response-dialog");
      if (dialog && typeof dialog.showModal === "function") {
        dialog.showModal();
      }
    });
  }

  const cancelRecordBtn = byId("cancel-record-response");
  if (cancelRecordBtn) {
    cancelRecordBtn.addEventListener("click", () => {
      const dialog = byId("record-response-dialog");
      if (dialog && typeof dialog.close === "function") {
        dialog.close();
      }
    });
  }

  const recordResponseForm = byId("record-response-form");
  if (recordResponseForm) {
    recordResponseForm.addEventListener("submit", (e) => {
      e.preventDefault();
      const selectVal = byId("record-response-select")?.value || "Verified none";
      
      attendeeData.LL.dietary = selectVal;
      attendeeData.LL.readiness = "Ready";
      attendeeData.LL.warning = `Verified: ${selectVal}`;
      attendeeData.LL.resolution = "Recorded & ready";

      if (byId("ll-dietary-status")) byId("ll-dietary-status").textContent = selectVal;
      const chip = byId("ll-readiness-chip");
      if (chip) {
        chip.className = "chip good";
        chip.textContent = "✓ Ready";
      }

      if (byId("reconcile-gap-count")) {
        byId("reconcile-gap-count").textContent = "0 needs clarification";
        byId("reconcile-gap-count").style.color = "var(--success)";
      }

      updateDrawer("LL");
      status(`Recorded dietary response for Liam Lynch: ${selectVal}`);

      const dialog = byId("record-response-dialog");
      if (dialog && typeof dialog.close === "function") {
        dialog.close();
      }
    });
  }

  // Inline pencil edit buttons
  document.querySelectorAll(".inline-edit-btn").forEach((btn) => {
    btn.addEventListener("click", (e) => {
      e.stopPropagation();
      const field = btn.dataset.field;
      if (!field) return;
      state.currentEditingField = field;
      const targetEl = byId(`field-${field}`);
      const currentVal = targetEl ? targetEl.textContent : "";
      
      const titleEl = byId("edit-field-title");
      if (titleEl) titleEl.textContent = `Edit ${field.replace("_", " ")}`;
      const inputEl = byId("edit-field-input");
      if (inputEl) inputEl.value = currentVal;

      const dialog = byId("edit-field-dialog");
      if (dialog && typeof dialog.showModal === "function") {
        dialog.showModal();
      }
    });
  });

  const cancelEditField = byId("cancel-edit-field");
  if (cancelEditField) {
    cancelEditField.addEventListener("click", () => {
      const dialog = byId("edit-field-dialog");
      if (dialog && typeof dialog.close === "function") {
        dialog.close();
      }
    });
  }

  const editFieldForm = byId("edit-field-form");
  if (editFieldForm) {
    editFieldForm.addEventListener("submit", (e) => {
      e.preventDefault();
      const newVal = byId("edit-field-input")?.value || "";
      const field = state.currentEditingField;
      if (field) {
        const targetEl = byId(`field-${field}`);
        if (targetEl) targetEl.textContent = newVal;
        status(`Updated ${field}: ${newVal}`);
      }
      const dialog = byId("edit-field-dialog");
      if (dialog && typeof dialog.close === "function") {
        dialog.close();
      }
    });
  }

  // Desktop vs Narrow preview toggle in Edit Profile view
  const desktopPreviewBtn = byId("desktop-preview-btn");
  const narrowPreviewBtn = byId("narrow-preview-btn");
  const previewBox = byId("live-preview-box");

  if (desktopPreviewBtn && narrowPreviewBtn && previewBox) {
    desktopPreviewBtn.addEventListener("click", () => {
      previewBox.classList.remove("is-narrow");
      desktopPreviewBtn.style.background = "var(--ink)";
      desktopPreviewBtn.style.color = "#fff";
      narrowPreviewBtn.style.background = "var(--surface)";
      narrowPreviewBtn.style.color = "var(--ink)";
    });

    narrowPreviewBtn.addEventListener("click", () => {
      previewBox.classList.add("is-narrow");
      narrowPreviewBtn.style.background = "var(--ink)";
      narrowPreviewBtn.style.color = "#fff";
      desktopPreviewBtn.style.background = "var(--surface)";
      desktopPreviewBtn.style.color = "var(--ink)";
    });
  }

  document.querySelectorAll(".save-profile-customisation-btn").forEach((btn) => {
    btn.addEventListener("click", () => {
      status("Saved local profile customisation for Aisling Byrne.");
      navigate("person");
    });
  });

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

  document.querySelectorAll(".view-history-trigger").forEach((btn) => {
    btn.addEventListener("click", () => {
      const dialog = byId("source-history-dialog");
      if (dialog && typeof dialog.showModal === "function") {
        dialog.showModal();
      }
    });
  });

  const closeHistoryBtn = byId("close-history-dialog");
  if (closeHistoryBtn) {
    closeHistoryBtn.addEventListener("click", () => {
      const dialog = byId("source-history-dialog");
      if (dialog && typeof dialog.close === "function") {
        dialog.close();
      }
    });
  }
};

document.addEventListener("DOMContentLoaded", () => {
  applyTheme("light");
  bindEvents();
  updateDrawer("LL");
  renderTextNode(
    byId("live-status"),
    `Liaison RM ${state.productState.app.product_state} ready. ${state.productState.app.connection_state} · ${state.productState.app.release_evidence}`
  );
});
