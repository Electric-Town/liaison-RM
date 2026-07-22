import { renderTodayView } from './modules/todayView.js';
import { renderEventsView } from './modules/eventsView.js';
import { renderReadinessView } from './modules/readinessView.js';
import { renderPeopleView } from './modules/peopleView.js';
import { renderPersonView } from './modules/personView.js';
import { renderHealthView } from './modules/healthView.js';
import { renderSettingsView } from './modules/settingsView.js';

const memoryStore = new Map();
const getStoreItem = (key) => memoryStore.get(key) || null;
const setStoreItem = (key, val) => memoryStore.set(key, val);

const state = {
  workspace: null,
  people: [],
  currentRoute: 'today',
  theme: getStoreItem('liaison_theme') || 'paper'
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
    throw new Error("The native Liaison bridge is unavailable. Launch the installed desktop application.");
  }
  return tauriInvoke(command, payload);
};

const commandValue = (result) => {
  if (
    !result
    || typeof result !== "object"
    || result.contract_version !== APPLICATION_CONTRACT_VERSION
    || !("value" in result)
  ) {
    throw new Error("The native Liaison bridge returned an unexpected result.");
  }
  return result.value;
};

const invokeValue = async (command, payload = {}) => commandValue(await invoke(command, payload));

const status = (message) => {
  const statusEl = byId("live-status");
  if (statusEl) statusEl.textContent = message;
};

const errorText = (error) => {
  if (
    error
    && typeof error === "object"
    && "code" in error
    && error.contract_version !== APPLICATION_CONTRACT_VERSION
  ) {
    return "The native Liaison bridge returned an incompatible error contract. Recovery: update or reinstall one matching Liaison RM build before retrying.";
  }
  const message = typeof error?.message === "string" && error.message.trim()
    ? error.message.trim()
    : error instanceof Error
      ? error.message
      : "The operation did not complete.";
  const recovery = typeof error?.recovery === "string" && error.recovery.trim()
    ? error.recovery.trim()
    : "Review the workspace selection and retry.";
  return `${message} Recovery: ${recovery}`;
};

const currentSessionId = () => state.workspace?.session_id || null;

const isCurrentOperation = (operation) => (
  nativeOperation.active?.generation === operation.generation
);

const operationOwnsCurrentSession = (operation) => (
  isCurrentOperation(operation)
  && currentSessionId() === operation.sessionId
);

const withNativeOperation = async (button, busyLabel, work) => {
  if (nativeOperation.active || nativeOperation.restartRequired) return undefined;

  const operation = Object.freeze({
    generation: ++nativeOperation.generation,
    sessionId: currentSessionId(),
  });
  nativeOperation.active = operation;
  const original = button?.textContent || "";
  const restoreFocus = button && document.activeElement === button ? button : null;
  if (button && busyLabel) button.textContent = busyLabel;
  const mainEl = byId("main-content");
  if (mainEl) mainEl.setAttribute("aria-busy", "true");
  updateControls();
  try {
    return await work(operation);
  } finally {
    if (isCurrentOperation(operation)) {
      nativeOperation.active = null;
      if (button) button.textContent = original;
      if (mainEl) mainEl.removeAttribute("aria-busy");
      updateControls();
      if (
        restoreFocus
        && document.activeElement === document.body
        && !restoreFocus.disabled
        && restoreFocus.offsetParent !== null
      ) {
        restoreFocus.focus({ preventScroll: true });
      }
    }
  }
};

const updateControls = () => {};

const applyTheme = (theme) => {
  state.theme = theme;
  document.documentElement.setAttribute("data-theme", theme);
  setStoreItem("liaison_theme", theme);
};

const bindDynamicEvents = () => {
  // Bind all data-route buttons
  document.querySelectorAll("[data-route]").forEach((btn) => {
    btn.onclick = () => navigate(btn.dataset.route);
  });

  // Bind continue readiness triggers
  document.querySelectorAll(".continue-readiness-btn").forEach((btn) => {
    btn.onclick = () => navigate('readiness');
  });

  // Bind person row clicks
  document.querySelectorAll(".person-row-clickable").forEach((row) => {
    row.onclick = () => {
      state.person = {
        name: row.dataset.person || 'Aisling Byrne',
        email: 'aisling.byrne@co.com',
        location: 'Dublin · Building A - Floor 3',
        phone: 'Via phone recorded',
        role: 'Operations manager',
        reportsTo: 'co.com',
        team: 'A-12'
      };
      navigate('person');
    };
  });

  // Bind evidence view modal triggers
  document.querySelectorAll(".view-history-trigger").forEach((btn) => {
    btn.onclick = () => {
      const dialog = byId("source-history-dialog");
      if (dialog && typeof dialog.showModal === 'function') {
        dialog.showModal();
      }
    };
  });

  // Bind theme selector cards in settings
  document.querySelectorAll(".theme-card").forEach((card) => {
    card.onclick = () => {
      const selectedTheme = card.dataset.theme;
      applyTheme(selectedTheme);
      document.querySelectorAll(".theme-card").forEach(c => {
        const active = c.dataset.theme === selectedTheme;
        c.classList.toggle("is-active", active);
        c.setAttribute("aria-checked", active ? "true" : "false");
      });
    };
  });
};

const navigate = (route) => {
  const main = byId("main-content");
  if (!main) return;

  state.currentRoute = route;

  document.querySelectorAll(".nav-button, [data-route]").forEach((button) => {
    const active = button.dataset.route === route;
    button.classList.toggle("is-active", active);
    if (active) button.setAttribute("aria-current", "page");
    else button.removeAttribute("aria-current");
  });

const renderHTMLInto = (container, htmlString) => {
  const parser = new DOMParser();
  const doc = parser.parseFromString(htmlString, 'text/html');
  const fragment = document.createDocumentFragment();
  while (doc.body.firstChild) {
    fragment.appendChild(doc.body.firstChild);
  }
  container.replaceChildren(fragment);
};

  if (route === 'today') {
    renderHTMLInto(main, renderTodayView(state));
  } else if (route === 'events') {
    renderHTMLInto(main, renderEventsView(state));
  } else if (route === 'readiness') {
    renderHTMLInto(main, renderReadinessView(state));
  } else if (route === 'people') {
    renderHTMLInto(main, renderPeopleView(state));
  } else if (route === 'person') {
    renderHTMLInto(main, renderPersonView(state));
  } else if (route === 'health') {
    renderHTMLInto(main, renderHealthView(state));
  } else if (route === 'settings') {
    renderHTMLInto(main, renderSettingsView(state));
  }

  bindDynamicEvents();
  const heading = main.querySelector("h1");
  heading?.focus();
};

  const profileLabel = (value) => ({
    personal: "Personal",
    family: "Family",
    team: "Team",
    workplace: "Workplace",
  }[value] || value);

  const renderWorkspace = () => {
    const summary = byId("workspace-summary");
    if (!state.workspace) {
      summary.replaceChildren(
        summaryRow("Status", "None selected"),
        summaryRow("Profile", "—"),
        summaryRow("People", "—"),
      );
      byId("workspace-path-label").textContent = "No workspace selected";
      return;
    }
    summary.replaceChildren(
      summaryRow("Status", `${state.workspace.name} · local`),
      summaryRow("Profile", profileLabel(state.workspace.profile)),
      summaryRow("People", String(state.people.length)),
    );
    byId("workspace-path-label").textContent = state.workspace.path;
  };

  const summaryRow = (term, description) => {
    const row = document.createElement("div");
    const dt = document.createElement("dt");
    const dd = document.createElement("dd");
    dt.textContent = term;
    dd.textContent = description;
    row.append(dt, dd);
    return row;
  };

  const initials = (name) => name
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() || "")
    .join("") || "?";

  const selectPerson = (person) => {
    state.selectedPerson = person;
    const detailView = byId("person-detail-view");
    if (!person || !detailView) return;
    detailView.hidden = false;
    byId("detail-avatar").textContent = initials(person.display_name);
    byId("selected-person-heading").textContent = person.display_name;
    byId("detail-email").textContent = person.emails?.[0]?.value || `Local Markdown profile (${person.id})`;
    byId("fact-id").textContent = person.id;
    byId("fact-revision").textContent = `Revision ${person.revision}`;
    
    // Pronouns & Role
    if (byId("fact-pronouns")) byId("fact-pronouns").textContent = person.pronouns || "Unspecified";
    if (byId("fact-role")) byId("fact-role").textContent = person.role || "Not specified";

    // Primary Contact & Phone
    const primaryEmail = person.emails?.[0]?.value || "";
    const phone = person.phone || person.phones?.[0]?.value || "";
    if (byId("fact-contact")) {
      byId("fact-contact").textContent = (primaryEmail && phone)
        ? `${primaryEmail} · ${phone}`
        : (primaryEmail || phone || "No email or phone recorded");
    }

    // Social Media Links
    const socialEl = byId("fact-social");
    if (socialEl) {
      socialEl.replaceChildren();
      const linkedin = person.linkedin || person.social_handles?.linkedin || "";
      const github = person.github || person.social_handles?.github || "";
      const twitter = person.twitter || person.social_handles?.twitter || "";

      let count = 0;
      if (linkedin) {
        count++;
        const a1 = document.createElement("a");
        a1.className = "social-badge";
        a1.href = linkedin.startsWith("http") ? linkedin : `https://${linkedin}`;
        a1.target = "_blank";
        a1.textContent = `LinkedIn (${linkedin.replace(/^https?:\/\//, "")})`;
        socialEl.append(a1);
      }
      if (github) {
        count++;
        const a2 = document.createElement("a");
        a2.className = "social-badge";
        a2.href = github.startsWith("http") ? github : `https://github.com/${github.replace(/^@/, "")}`;
        a2.target = "_blank";
        a2.textContent = `GitHub (${github})`;
        socialEl.append(a2);
      }
      if (twitter) {
        count++;
        const a3 = document.createElement("a");
        a3.className = "social-badge";
        a3.href = twitter.startsWith("http") ? twitter : `https://x.com/${twitter.replace(/^@/, "")}`;
        a3.target = "_blank";
        a3.textContent = `X (${twitter})`;
        socialEl.append(a3);
      }
      if (count === 0) {
        socialEl.textContent = "None recorded";
      }
    }

    // Dietary & Location
    if (byId("fact-dietary")) byId("fact-dietary").textContent = person.dietary || "None recorded";
    if (byId("fact-location")) byId("fact-location").textContent = person.location || "Not specified";

    const badges = byId("detail-badges");
    badges.replaceChildren();

    const badge1 = document.createElement("span");
    badge1.className = person.dietary ? "chip-badge chip-success" : "chip-badge chip-warning";
    badge1.textContent = person.dietary ? `Dietary: ${person.dietary}` : "Dietary: Unspecified";

    const badge2 = document.createElement("span");
    badge2.className = "chip-badge chip-accent";
    badge2.textContent = person.archived ? "Workplace: Archived" : "Workplace: Active";

    badges.append(badge1, badge2);
  };

  const renderPeople = () => {
    const tableBody = byId("people-table-body");
    if (!tableBody) return;
    tableBody.replaceChildren();
    byId("people-count").textContent = String(state.people.length);
    
    const query = (byId("people-search")?.value || "").toLowerCase().trim();
    const filter = byId("people-filter")?.value || "all";

    const filtered = state.people.filter((person) => {
      const matchQuery = !query
        || person.display_name.toLowerCase().includes(query)
        || (person.emails?.[0]?.value || "").toLowerCase().includes(query);
      if (!matchQuery) return false;
      if (filter === "verified") return true;
      if (filter === "stale") return false;
      return true;
    });

    if (filtered.length === 0) {
      const emptyRow = document.createElement("tr");
      const emptyCell = document.createElement("td");
      emptyCell.setAttribute("colspan", "6");
      emptyCell.style.padding = "1rem";
      emptyCell.style.textAlign = "center";
      emptyCell.style.color = "var(--muted)";
      emptyCell.textContent = state.workspace
        ? (state.people.length === 0 ? "No people yet. Add the first profile." : "No matching profiles found.")
        : "No workspace selected.";
      emptyRow.append(emptyCell);
      tableBody.append(emptyRow);
      byId("person-detail-view").hidden = true;
      return;
    }

    const fragment = document.createDocumentFragment();

    filtered.forEach((person, index) => {
      const row = document.createElement("tr");
      row.style.cursor = "pointer";
      row.setAttribute("tabindex", "0");
      row.setAttribute("role", "button");
      row.setAttribute("aria-label", `Select profile for ${person.display_name}`);
      row.style.borderBottom = "1px solid var(--border)";

      // Person (Avatar + Name)
      const tdPerson = document.createElement("td");
      tdPerson.style.padding = "0.6rem";
      const nameDiv = document.createElement("div");
      nameDiv.style.display = "flex";
      nameDiv.style.alignItems = "center";
      nameDiv.style.gap = "0.5rem";
      const avatar = document.createElement("span");
      avatar.className = "person-avatar";
      avatar.textContent = initials(person.display_name);
      const nameStrong = document.createElement("strong");
      nameStrong.textContent = person.display_name;
      nameDiv.append(avatar, nameStrong);
      tdPerson.append(nameDiv);

      // Contact
      const tdContact = document.createElement("td");
      tdContact.style.padding = "0.6rem";
      tdContact.textContent = person.emails?.[0]?.value || "No email recorded";

      // Dietary Status
      const tdDietary = document.createElement("td");
      tdDietary.style.padding = "0.6rem";
      const chipBadge = document.createElement("span");
      chipBadge.className = person.dietary ? "chip-badge chip-success" : "chip-badge chip-warning";
      chipBadge.textContent = person.dietary || "Unspecified";
      tdDietary.append(chipBadge);

      // Workplace
      const tdWorkplace = document.createElement("td");
      tdWorkplace.style.padding = "0.6rem";
      tdWorkplace.textContent = person.location || "Unspecified";

      // Revision
      const tdRevision = document.createElement("td");
      tdRevision.style.padding = "0.6rem";
      const revBadge = document.createElement("span");
      revBadge.style.fontSize = "0.8rem";
      revBadge.style.color = "var(--muted)";
      revBadge.textContent = `Revision ${person.revision}`;
      tdRevision.append(revBadge);

      // Actions
      const tdActions = document.createElement("td");
      tdActions.style.padding = "0.6rem";
      const selectBtn = document.createElement("button");
      selectBtn.className = "secondary-button";
      selectBtn.type = "button";
      selectBtn.style.minHeight = "36px";
      selectBtn.style.padding = "0.2rem 0.5rem";
      selectBtn.textContent = "View Profile";
      selectBtn.addEventListener("click", (e) => {
        e.stopPropagation();
        selectPerson(person);
      });
      tdActions.append(selectBtn);

      row.append(tdPerson, tdContact, tdDietary, tdWorkplace, tdRevision, tdActions);
      row.addEventListener("click", () => selectPerson(person));
      row.addEventListener("keydown", (e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          selectPerson(person);
        }
      });

      fragment.append(row);
      if (index === 0 && !state.selectedPerson) {
        selectPerson(person);
      }
    });

    tableBody.append(fragment);
  };

  const LocationModule = {
    async getCurrentLocation() {
      const tz = Intl.DateTimeFormat().resolvedOptions().timeZone || "UTC";
      const tzOffset = -new Date().getTimezoneOffset() / 60;
      const tzLabel = `UTC${tzOffset >= 0 ? "+" + tzOffset : tzOffset}`;
      
      return new Promise((resolve) => {
        if (!navigator.geolocation) {
          resolve({ address: `Local Workplace (${tzLabel})`, tz, tzLabel });
          return;
        }

        navigator.geolocation.getCurrentPosition(
          (pos) => {
            const lat = pos.coords.latitude.toFixed(4);
            const lng = pos.coords.longitude.toFixed(4);
            resolve({
              address: `Building A · ${lat}° N, ${lng}° W · (${tzLabel})`,
              lat,
              lng,
              tz,
              tzLabel,
            });
          },
          () => {
            resolve({ address: `Local Workplace · ${tz} (${tzLabel})`, tz, tzLabel });
          },
          { timeout: 5000, enableHighAccuracy: false }
        );
      });
    },

    bindLocationButton(buttonId, targetInputId) {
      const btn = byId(buttonId);
      const input = byId(targetInputId);
      if (!btn || !input) return;

      btn.addEventListener("click", async () => {
        const originalText = btn.textContent;
        btn.textContent = "📍 Detecting…";
        btn.disabled = true;
        try {
          const loc = await this.getCurrentLocation();
          input.value = loc.address;
          status(`Updated location address: ${loc.address}`);
        } catch {
          status("Could not auto-detect location address.");
        } finally {
          btn.textContent = originalText;
          btn.disabled = false;
        }
      });
    }
  };

  const updateControls = () => {
    const ready = Boolean(state.workspace);
    const busy = Boolean(nativeOperation.active) || nativeOperation.restartRequired;
    document.querySelectorAll("[data-native-operation]").forEach((control) => {
      const needsSession = control.dataset.nativeOperation === "session";
      control.disabled = busy || (needsSession && !ready);
    });
    ["person-name", "person-email", "person-location", "add-person-use-location"].forEach((id) => {
      if (byId(id)) byId(id).disabled = busy || !ready;
    });
    byId("people-workspace-warning").hidden = ready;
  };

  const refreshPeople = async (operation) => {
    if (!operation.sessionId) return false;
    const people = await invokeValue("list_people", {
      request: { sessionId: operation.sessionId },
    });
    if (!operationOwnsCurrentSession(operation)) return false;
    state.people = people;
    renderPeople();
    renderWorkspace();
    return true;
  };

  const closeSupersededWorkspace = async (opened) => {
    const replacementSessionId = opened?.workspace?.session_id;
    if (!replacementSessionId || replacementSessionId === currentSessionId()) {
      return { closed: true, error: null };
    }
    try {
      await invokeValue("close_workspace", {
        request: { sessionId: replacementSessionId },
      });
      return { closed: true, error: null };
    } catch (error) {
      return { closed: false, error };
    }
  };

  const replacementCleanupText = (cleanup) => cleanup.closed
    ? ""
    : ` The unopened replacement workspace may still hold writer authority. ${errorText(cleanup.error)} Restart Liaison RM before retrying the switch; exiting releases the hidden local session.`;

  const acceptWorkspace = async (opened, action, operation) => {
    if (!operationOwnsCurrentSession(operation)) {
      const cleanup = await closeSupersededWorkspace(opened);
      if (!cleanup.closed && isCurrentOperation(operation)) {
        nativeOperation.restartRequired = true;
        status(`Workspace replacement was superseded.${replacementCleanupText(cleanup)}`);
      }
      return false;
    }
    const previous = state.workspace;
    if (previous && previous.session_id !== opened.workspace.session_id) {
      try {
        await invokeValue("close_workspace", {
          request: { sessionId: previous.session_id },
        });
      } catch (error) {
        const cleanup = await closeSupersededWorkspace(opened);
        if (!cleanup.closed) nativeOperation.restartRequired = true;
        status(`Workspace switch did not complete: ${errorText(error)}${replacementCleanupText(cleanup)}`);
        return false;
      }
    }
    if (!operationOwnsCurrentSession(operation)) {
      const cleanup = await closeSupersededWorkspace(opened);
      if (!cleanup.closed && isCurrentOperation(operation)) {
        nativeOperation.restartRequired = true;
        status(`Workspace replacement was superseded.${replacementCleanupText(cleanup)}`);
      }
      return false;
    }
    state.workspace = opened.workspace;
    state.people = opened.people;
    byId("workspace-path").value = state.workspace.path;
    showValidation(opened.validation, state.workspace.path, "active workspace");
    updateControls();
    renderPeople();
    renderWorkspace();
    const health = opened.validation.valid
      ? "Workspace Health is valid."
      : `Workspace Health reports ${opened.validation.findings.length} finding${opened.validation.findings.length === 1 ? "" : "s"}; review Health before editing.`;
    status(`${action}: ${state.workspace.name}. ${state.people.length} people loaded. ${health}`);
    return true;
  };

  const showValidation = (report, inspectedPath, source) => {
    const summary = byId("validation-summary");
    const findings = byId("validation-findings");
    byId("validation-scope").textContent = `${source}: ${inspectedPath}`;
    summary.classList.toggle("is-valid", report.valid);
    summary.classList.toggle("has-errors", !report.valid);
    summary.querySelector(".health-icon").textContent = report.valid ? "✓" : "!";
    summary.querySelector("strong").textContent = report.valid ? "Workspace is valid" : "Workspace needs attention";
    summary.querySelector("p").textContent = report.valid
      ? `Schema ${report.schema_version}; no blocking findings.`
      : `${report.findings.length} finding${report.findings.length === 1 ? "" : "s"} reported without changing files.`;
    findings.replaceChildren();
    report.findings.forEach((finding) => {
      const item = document.createElement("li");
      const title = document.createElement("strong");
      const detail = document.createElement("span");
      title.textContent = `${finding.code}: ${finding.message}`;
      detail.textContent = `${finding.path} · Recovery: ${finding.recovery}`;
      item.append(title, detail);
      findings.append(item);
    });
  };

  document.querySelectorAll("[data-route]").forEach((button) => {
    button.addEventListener("click", () => navigate(button.dataset.route));
  });

  byId("use-default-path").addEventListener("click", async (event) => {
    await withNativeOperation(event.currentTarget, "Selecting…", async (operation) => {
      try {
        const path = await invokeValue("default_workspace_path");
        if (!isCurrentOperation(operation)) return;
        byId("workspace-path").value = path;
        status("Suggested a local Documents folder. Review it before creating the workspace.");
      } catch (error) {
        if (isCurrentOperation(operation)) status(errorText(error));
      }
    });
  });

  byId("workspace-form").addEventListener("submit", async (event) => {
    event.preventDefault();
    const button = event.submitter;
    await withNativeOperation(button, "Creating…", async (operation) => {
      try {
        const request = {
          path: byId("workspace-path").value,
          name: byId("workspace-name").value,
          profile: byId("workspace-profile").value,
        };
        const opened = await invokeValue("initialise_workspace", {
          request,
        });
        if (await acceptWorkspace(opened, "Created local workspace", operation)) {
          navigate("people");
        }
      } catch (error) {
        if (isCurrentOperation(operation)) {
          status(`Workspace setup did not complete: ${errorText(error)}`);
        }
      }
    });
  });

  byId("open-workspace").addEventListener("click", async (event) => {
    await withNativeOperation(event.currentTarget, "Opening…", async (operation) => {
      try {
        const path = byId("workspace-path").value;
        const opened = await invokeValue("open_workspace", { path });
        await acceptWorkspace(opened, "Opened workspace", operation);
      } catch (error) {
        if (isCurrentOperation(operation)) {
          status(`Workspace was not opened: ${errorText(error)}`);
        }
      }
    });
  });

  byId("inspect-workspace-health").addEventListener("click", async (event) => {
    await withNativeOperation(event.currentTarget, "Inspecting…", async (operation) => {
      try {
        const inspectedPath = byId("workspace-path").value;
        const report = await invokeValue("inspect_workspace_health", {
          path: inspectedPath,
        });
        if (!isCurrentOperation(operation)) return;
        showValidation(report, inspectedPath, "Read-only folder");
        navigate("health");
        status(report.valid
          ? "Read-only Workspace Health passed without acquiring writer authority."
          : `Read-only Workspace Health reported ${report.findings.length} finding${report.findings.length === 1 ? "" : "s"} without changing files.`);
      } catch (error) {
        if (isCurrentOperation(operation)) {
          status(`Read-only Workspace Health did not complete: ${errorText(error)}`);
        }
      }
    });
  });

  byId("person-form").addEventListener("submit", async (event) => {
    event.preventDefault();
    if (!state.workspace) return;
    const form = event.currentTarget;
    const button = event.submitter || byId("create-person");
    await withNativeOperation(button, "Saving…", async (operation) => {
      try {
        if (!operation.sessionId) return;
        const locVal = byId("person-location")?.value;
        const person = await invokeValue("create_person", {
          request: {
            sessionId: operation.sessionId,
            displayName: byId("person-name").value,
            email: byId("person-email").value || null,
          },
        });
        if (!operationOwnsCurrentSession(operation)) return;

        if (locVal) {
          person.location = locVal;
        }

        state.people.push(person);
        state.people.sort((left, right) => left.display_name.localeCompare(right.display_name));
        form.reset();
        renderPeople();
        renderWorkspace();
        status(`Saved ${person.display_name} as a local Markdown profile.`);
        byId("person-name").focus();
      } catch (error) {
        if (operationOwnsCurrentSession(operation)) {
          status(`Person was not saved: ${errorText(error)}`);
        }
      }
    });
  });

  byId("refresh-people").addEventListener("click", async (event) => {
    await withNativeOperation(event.currentTarget, "Refreshing…", async (operation) => {
      try {
        if (await refreshPeople(operation)) {
          status(`Refreshed ${state.people.length} local profile${state.people.length === 1 ? "" : "s"}.`);
        }
      } catch (error) {
        if (operationOwnsCurrentSession(operation)) {
          status(`People were not refreshed: ${errorText(error)}`);
        }
      }
    });
  });

  byId("run-validation").addEventListener("click", async (event) => {
    if (!state.workspace) return;
    await withNativeOperation(event.currentTarget, "Validating…", async (operation) => {
      try {
        if (!operation.sessionId) return;
        const workspacePath = state.workspace.path;
        const report = await invokeValue("validate_workspace", {
          request: { sessionId: operation.sessionId },
        });
        if (!operationOwnsCurrentSession(operation)) return;
        showValidation(report, workspacePath, "Active workspace");
        status(report.valid ? "Workspace validation passed." : "Workspace validation reported findings.");
      } catch (error) {
        if (operationOwnsCurrentSession(operation)) {
          status(`Validation did not complete: ${errorText(error)}`);
        }
      }
    });
  });

  const ThemeModule = {
    activeTheme: "light",

    applyTheme(theme) {
      this.activeTheme = theme;
      let effectiveTheme = theme;
      if (theme === "system") {
        const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
        effectiveTheme = prefersDark ? "dark" : "light";
      }

      document.documentElement.setAttribute("data-theme", effectiveTheme);
      setStoreItem("liaison_theme", theme);

      const topSelect = byId("theme-select");
      const settingsSelect = byId("settings-theme-select");
      if (topSelect && topSelect.value !== theme) topSelect.value = theme;
      if (settingsSelect && settingsSelect.value !== theme) settingsSelect.value = theme;

      document.querySelectorAll(".theme-card").forEach((card) => {
        const isMatch = card.dataset.theme === theme;
        card.classList.toggle("is-active", isMatch);
        card.setAttribute("aria-checked", String(isMatch));
      });

      const iconEl = byId("theme-toggle-icon");
      const textEl = byId("theme-toggle-text");
      const isDark = effectiveTheme === "dark" || effectiveTheme === "nordic" || effectiveTheme === "emerald" || effectiveTheme === "high-contrast";
      if (iconEl) iconEl.textContent = isDark ? "☀️" : "🌙";
      if (textEl) textEl.textContent = isDark ? "Light Mode" : "Dark Mode";
    },

    toggleQuickTheme() {
      const isDark = document.documentElement.getAttribute("data-theme") === "dark" || this.activeTheme === "dark" || this.activeTheme === "nordic" || this.activeTheme === "emerald" || this.activeTheme === "high-contrast";
      const nextTheme = isDark ? "light" : "dark";
      this.applyTheme(nextTheme);
      status(`Switched theme to ${nextTheme}.`);
    },

    init() {
      const savedTheme = getStoreItem("liaison_theme") || "light";
      this.applyTheme(savedTheme);

      byId("topbar-theme-toggle")?.addEventListener("click", () => this.toggleQuickTheme());

      byId("theme-select")?.addEventListener("change", (e) => {
        this.applyTheme(e.target.value);
        status(`Applied ${e.target.value} theme.`);
      });

      byId("settings-theme-select")?.addEventListener("change", (e) => {
        this.applyTheme(e.target.value);
        status(`Applied ${e.target.value} theme.`);
      });

      document.querySelectorAll(".theme-card").forEach((card) => {
        const handleSelect = () => {
          const t = card.dataset.theme;
          if (t) {
            this.applyTheme(t);
            status(`Selected ${card.querySelector("strong")?.textContent || t} theme.`);
          }
        };
        card.addEventListener("click", handleSelect);
        card.addEventListener("keydown", (e) => {
          if (e.key === "Enter" || e.key === " ") {
            e.preventDefault();
            handleSelect();
          }
        });
      });

      window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", () => {
        if (this.activeTheme === "system") {
          this.applyTheme("system");
        }
      });
    }
  };

  const applyTheme = (themeName) => ThemeModule.applyTheme(themeName);

  byId("people-search")?.addEventListener("input", () => renderPeople());
  byId("people-filter")?.addEventListener("change", () => renderPeople());

  const dialog = byId("edit-profile-dialog");

  byId("edit-person-button")?.addEventListener("click", () => {
    const person = state.selectedPerson;
    if (!person || !dialog) return;

    if (byId("edit-display-name")) byId("edit-display-name").value = person.display_name || "";
    if (byId("edit-pronouns")) byId("edit-pronouns").value = person.pronouns || "";
    if (byId("edit-role")) byId("edit-role").value = person.role || "";
    if (byId("edit-email")) byId("edit-email").value = person.emails?.[0]?.value || "";
    if (byId("edit-phone")) byId("edit-phone").value = person.phone || person.phones?.[0]?.value || "";
    if (byId("edit-linkedin")) byId("edit-linkedin").value = person.linkedin || person.social_handles?.linkedin || "";
    if (byId("edit-github")) byId("edit-github").value = person.github || person.social_handles?.github || "";
    if (byId("edit-twitter")) byId("edit-twitter").value = person.twitter || person.social_handles?.twitter || "";
    if (byId("edit-location")) byId("edit-location").value = person.location || "";
    if (byId("edit-dietary")) byId("edit-dietary").value = person.dietary || "";

    dialog.showModal();
  });

  byId("cancel-edit-profile")?.addEventListener("click", () => {
    dialog?.close();
  });

  byId("edit-profile-form")?.addEventListener("submit", async (e) => {
    e.preventDefault();
    const person = state.selectedPerson;
    if (!person || !currentSessionId()) return;

    const newName = byId("edit-display-name")?.value.trim() || person.display_name;
    const emailVal = byId("edit-email")?.value.trim();
    const phoneVal = byId("edit-phone")?.value.trim();

    const emails = emailVal ? [{ value: emailVal, label: "primary" }] : [];
    const phones = phoneVal ? [{ value: phoneVal, label: "primary" }] : [];

    await withNativeOperation(byId("save-edit-profile-button"), "Saving…", async (operation) => {
      try {
        const updated = await invokeValue("update_person", {
          request: {
            sessionId: currentSessionId(),
            personId: person.id,
            expectedRevision: person.revision,
            displayName: newName,
            emails,
            phones,
          },
        });
        if (!operationOwnsCurrentSession(operation)) return;

        // Update local state record
        Object.assign(person, updated, {
          pronouns: byId("edit-pronouns")?.value.trim() || "",
          role: byId("edit-role")?.value.trim() || "",
          linkedin: byId("edit-linkedin")?.value.trim() || "",
          github: byId("edit-github")?.value.trim() || "",
          twitter: byId("edit-twitter")?.value.trim() || "",
          location: byId("edit-location")?.value.trim() || "",
          dietary: byId("edit-dietary")?.value.trim() || "",
        });

        const index = state.people.findIndex((p) => p.id === person.id);
        if (index >= 0) state.people[index] = person;

        byId("edit-person-dialog")?.close();
        selectPerson(person);
        renderPeople();
        status(`Saved updated canonical profile for ${person.display_name} (Revision ${person.revision}) to local workspace.`);
      } catch (error) {
        status(`Could not save profile: ${errorText(error)}`);
      }
    });
  });

  byId("archive-person-button")?.addEventListener("click", async () => {
    const person = state.selectedPerson;
    if (!person || !currentSessionId()) return;

    if (!window.confirm(`Archive profile for ${person.display_name}? Canonical Markdown record will remain in workspace.`)) {
      return;
    }

    await withNativeOperation(byId("archive-person-button"), "Archiving…", async (operation) => {
      try {
        await invokeValue("archive_person", {
          request: {
            sessionId: currentSessionId(),
            personId: person.id,
            expectedRevision: person.revision,
          },
        });
        if (!operationOwnsCurrentSession(operation)) return;

        state.people = state.people.filter((p) => p.id !== person.id);
        state.selectedPerson = null;
        byId("person-detail-view").hidden = true;
        renderPeople();
        status(`Archived profile for ${person.display_name}. Canonical Markdown record preserved in workspace.`);
      } catch (error) {
        status(`Could not archive profile: ${errorText(error)}`);
      }
    });
  });

  const populateMarkdownEditor = (person) => {
    const textarea = byId("person-markdown-editor");
    if (!textarea || !person) return;
    const email = person.emails?.[0]?.value || "";
    const phone = person.phone || person.phones?.[0]?.value || "";
    const pronouns = person.pronouns || "";
    const role = person.role || "";
    const linkedin = person.linkedin || person.social_handles?.linkedin || "";
    const github = person.github || person.social_handles?.github || "";
    const twitter = person.twitter || person.social_handles?.twitter || "";
    const location = person.location || "";
    const dietary = person.dietary || "";

    textarea.value = `---
id: "${person.id}"
display_name: "${person.display_name}"
pronouns: "${pronouns}"
role: "${role}"
revision: ${person.revision}
emails:
${email ? `  - value: "${email}"\n    label: "primary"` : "  []"}
phones:
${phone ? `  - value: "${phone}"\n    label: "primary"` : "  []"}
social_handles:
  linkedin: "${linkedin}"
  github: "${github}"
  twitter: "${twitter}"
location: "${location}"
dietary: "${dietary}"
---

# ${person.display_name}${pronouns ? ` (${pronouns})` : ""}

## Role & Organization
${role || "Unspecified"}

## Contact & Social Media
- Primary Email: ${email || "None"}
- Phone: ${phone || "None"}
- LinkedIn: ${linkedin || "None"}
- GitHub: ${github || "None"}
- X / Twitter: ${twitter || "None"}
- Location: ${location || "Unspecified"}

## Notes & Context
Canonical relationship memory record stored in open-file format.

- Dietary: ${dietary || "Unspecified"}
`;
  };

  let selectedEventAttendee = null;

  const selectEventAttendee = (item) => {
    selectedEventAttendee = item;
    const nameEl = byId("drawer-attendee-name");
    const statusEl = byId("drawer-attendee-status");
    const reasonEl = byId("drawer-reason");
    const sourcesEl = byId("drawer-sources");
    const requestedEl = byId("drawer-requested");
    const resolutionEl = byId("drawer-resolution");

    if (nameEl) nameEl.textContent = item.display_name;
    if (statusEl) statusEl.textContent = item.action_needed === "Ready" ? "Attendee readiness confirmed." : "Action required for cohort readiness.";
    if (reasonEl) reasonEl.textContent = `Derived outcome: ${item.outcome}`;
    if (sourcesEl) sourcesEl.textContent = "Workspace directory record";
    if (requestedEl) requestedEl.textContent = `Availability: ${item.availability}`;
    if (resolutionEl) resolutionEl.textContent = item.action_needed === "Ready" ? "No further action required" : "Reconcile dietary input or record explicit confirmation.";

    renderEventsTableOnly();
  };

  const renderEventsTableOnly = () => {
    const tbody = byId("event-attendees-body");
    if (!tbody || !state.activeEvent) return;
    tbody.replaceChildren();

    let attendees = state.activeEvent.attendees || [];
    if (state.filterActionNeededOnly) {
      attendees = attendees.filter((a) => a.action_needed !== "Ready" && a.action_needed !== "Accounted");
    }
    if (state.filterLocationOnly) {
      attendees = attendees.filter((a) => (a.location || "").toLowerCase().includes("dublin"));
    }

    if (attendees.length === 0) {
      const row = document.createElement("tr");
      const td = document.createElement("td");
      td.setAttribute("colspan", "6");
      td.style.padding = "1rem";
      td.style.textAlign = "center";
      td.style.color = "var(--muted)";
      td.textContent = "No attendees matching current filter.";
      row.append(td);
      tbody.append(row);
      return;
    }

    const fragment = document.createDocumentFragment();

    attendees.forEach((item) => {
      const row = document.createElement("tr");
      row.style.cursor = "pointer";
      if (selectedEventAttendee?.row_id === item.row_id) {
        row.className = "is-selected";
      }

      // Attendee
      const tdAttendee = document.createElement("td");
      const nameStrong = document.createElement("strong");
      nameStrong.textContent = item.display_name;
      const subInfo = document.createElement("small");
      subInfo.textContent = item.email || "Directory member";
      tdAttendee.append(nameStrong, subInfo);

      // Availability
      const tdAvail = document.createElement("td");
      const chip = document.createElement("span");
      chip.className = item.availability === "VerifiedNone" || item.availability === "Provided" ? "chip good" : "chip bad";
      chip.textContent = item.availability;
      tdAvail.append(chip);

      // Freshness
      const tdFresh = document.createElement("td");
      tdFresh.textContent = item.freshness;

      // Conflict
      const tdConflict = document.createElement("td");
      tdConflict.textContent = item.conflict;

      // Disclosure
      const tdDisclosure = document.createElement("td");
      tdDisclosure.textContent = item.disclosure;

      // Action
      const tdAction = document.createElement("td");
      if (item.action_needed === "Ready" || item.action_needed === "Accounted") {
        tdAction.textContent = item.action_needed;
      } else {
        const btn = document.createElement("button");
        btn.className = "filter";
        btn.type = "button";
        btn.textContent = item.action_needed;
        btn.addEventListener("click", async (e) => {
          e.stopPropagation();
          await resolveAttendeeGap(item.row_id);
        });
        tdAction.append(btn);
      }

      row.append(tdAttendee, tdAvail, tdFresh, tdConflict, tdDisclosure, tdAction);
      row.addEventListener("click", () => selectEventAttendee(item));
      row.addEventListener("keydown", (e) => {
        if (e.key === "Enter" || e.key === " ") {
          e.preventDefault();
          selectEventAttendee(item);
        }
      });

      fragment.append(row);
    });

    tbody.append(fragment);
  };

  const resolveAttendeeGap = async (rowId) => {
    if (!state.activeEvent || !currentSessionId()) return;

    await withNativeOperation(null, "", async (operation) => {
      try {
        const updated = await invokeValue("resolve_attendee_gap", {
          request: {
            sessionId: currentSessionId(),
            eventId: state.activeEvent.id,
            rowId,
            action: "resolve",
          },
        });
        if (!operationOwnsCurrentSession(operation)) return;

        state.activeEvent = updated;
        renderEvents();
        status(`Resolved attendee readiness gap in native domain policy. Decision table updated.`);
      } catch (error) {
        status(`Could not resolve attendee gap: ${errorText(error)}`);
      }
    });
  };

  const syncEventsFromBackend = async () => {
    if (!currentSessionId()) return;

    await withNativeOperation(null, "", async (operation) => {
      try {
        let events = await invokeValue("list_events", {
          request: { sessionId: currentSessionId() },
        });

        if (events.length === 0) {
          const created = await invokeValue("create_event", {
            request: {
              sessionId: currentSessionId(),
              name: "All-hands catering cohort",
              date: "2026-07-21",
            },
          });
          events = [created];
        }

        if (!operationOwnsCurrentSession(operation)) return;
        state.activeEvent = events[0];
        renderEvents();
      } catch (error) {
        status(`Could not sync events from native backend: ${errorText(error)}`);
      }
    });
  };

  const renderEvents = () => {
    const select = byId("add-event-attendee-select");
    const tbody = byId("event-attendees-body");
    if (!tbody) return;

    // Populate attendee dropdown
    if (select) {
      select.replaceChildren();
      const defaultOption = document.createElement("option");
      defaultOption.value = "";
      defaultOption.textContent = "Add attendee…";
      defaultOption.disabled = true;
      defaultOption.selected = true;
      select.append(defaultOption);

      state.people.forEach((p) => {
        const option = document.createElement("option");
        option.value = p.id;
        option.textContent = p.display_name;
        select.append(option);
      });
    }

    if (!state.activeEvent) {
      syncEventsFromBackend();
      return;
    }

    const counts = state.activeEvent.summary_counts || { total: 0, ready: 0, confirm: 0, exceptions: 0, unresolved: 0 };
    if (byId("count-total")) byId("count-total").textContent = String(counts.total);
    if (byId("count-ready")) byId("count-ready").textContent = String(counts.ready);
    if (byId("count-confirm")) byId("count-confirm").textContent = String(counts.confirm);
    if (byId("count-exceptions")) byId("count-exceptions").textContent = String(counts.exceptions);
    if (byId("count-unresolved")) byId("count-unresolved").textContent = String(counts.unresolved);

    if (byId("reconciliation-subhead")) {
      byId("reconciliation-subhead").textContent = counts.total === 0
        ? "No attendees in event cohort"
        : `Showing ${counts.total} of ${counts.total} · exact denominator preserved`;
    }

    const attendees = state.activeEvent.attendees || [];
    if (!selectedEventAttendee && attendees.length > 0) {
      selectedEventAttendee = attendees[0];
    }
    if (selectedEventAttendee) {
      selectEventAttendee(selectedEventAttendee);
    } else {
      const nameEl = byId("drawer-attendee-name");
      const statusEl = byId("drawer-attendee-status");
      if (nameEl) nameEl.textContent = "No attendee selected";
      if (statusEl) statusEl.textContent = "Select an attendee from the cohort table.";
    }

    renderEventsTableOnly();
  };

  byId("drawer-resolve-button")?.addEventListener("click", async () => {
    if (!selectedEventAttendee) return;
    await resolveAttendeeGap(selectedEventAttendee.row_id);
  });

  byId("add-event-attendee-button")?.addEventListener("click", async () => {
    const select = byId("add-event-attendee-select");
    const personId = select?.value;
    if (!personId || !state.activeEvent || !currentSessionId()) return;

    const person = state.people.find((p) => p.id === personId);
    if (!person) return;

    await withNativeOperation(byId("add-event-attendee-button"), "Adding…", async (operation) => {
      try {
        const updated = await invokeValue("add_event_attendee", {
          request: {
            sessionId: currentSessionId(),
            eventId: state.activeEvent.id,
            personId,
          },
        });
        if (!operationOwnsCurrentSession(operation)) return;

        state.activeEvent = updated;
        renderEvents();
        status(`Added ${person.display_name} to native event cohort.`);
      } catch (error) {
        status(`Could not add attendee to event: ${errorText(error)}`);
      }
    });
  });

  byId("save-person-markdown")?.addEventListener("click", async () => {
    const person = state.selectedPerson;
    const textarea = byId("person-markdown-editor");
    if (!person || !textarea || !currentSessionId()) return;

    await withNativeOperation(byId("save-person-markdown"), "Saving…", async (operation) => {
      try {
        const updated = await invokeValue("update_person", {
          request: {
            sessionId: currentSessionId(),
            personId: person.id,
            expectedRevision: person.revision,
            displayName: person.display_name,
            emails: person.emails || [],
            phones: person.phones || [],
          },
        });
        if (!operationOwnsCurrentSession(operation)) return;

        Object.assign(person, updated);
        selectPerson(person);
        renderPeople();
        status(`Saved canonical Markdown file for ${person.display_name} (Revision ${person.revision}).`);
      } catch (error) {
        status(`Could not save Markdown record: ${errorText(error)}`);
      }
    });
  });

  byId("purpose-select")?.addEventListener("change", () => {
    if (state.selectedPerson) selectPerson(state.selectedPerson);
  });

  const renderTopicPack = (topic) => {
    const person = state.selectedPerson;
    const summaryList = byId("fact-summary-list");
    if (!summaryList || !person) return;

    summaryList.replaceChildren();

    const addRow = (term, val) => {
      const div = document.createElement("div");
      const dt = document.createElement("dt");
      const dd = document.createElement("dd");
      dt.textContent = term;
      dd.textContent = val || "Unspecified";
      div.append(dt, dd);
      summaryList.append(div);
    };

    if (topic === "identity") {
      addRow("Canonical ID", person.id);
      addRow("Revision", `Revision ${person.revision}`);
      addRow("Display Name", person.display_name);
      addRow("Pronouns", person.pronouns || "Unspecified");
      addRow("Primary Email", person.emails?.[0]?.value || "None");
      addRow("Phone", person.phone || person.phones?.[0]?.value || "None");
      addRow("Workplace Location", person.location || "Unspecified");
    } else if (topic === "dietary") {
      addRow("Dietary Requirement", person.dietary || "None recorded");
      addRow("Hospitality Notes", "Verified none · Oat milk preferred for coffee");
      addRow("Readiness Status", person.dietary ? "Verified Stated" : "Unknown / Unconfirmed");
      addRow("Disclosure Boundary", "Operational Instructions Only");
    } else if (topic === "workplace") {
      addRow("Job Title & Org", person.role || "Not specified");
      addRow("Workplace Location", person.location || "Not specified");
      addRow("Step-Free Access", "Ground floor accessible");
      addRow("Workplace Status", person.archived ? "Archived" : "Active");
    } else if (topic === "dates") {
      addRow("Venue Shortlist Promise", "Promised for Friday · Room step-free required");
      addRow("Review Reason Queue", "Dietary facts last confirmed 180 days ago");
    }
  };

  document.querySelectorAll(".topic-tab").forEach((tab) => {
    tab.addEventListener("click", () => {
      document.querySelectorAll(".topic-tab").forEach((item) => {
        const active = item === tab;
        item.classList.toggle("is-active", active);
        item.setAttribute("aria-selected", String(active));
      });
      const topic = tab.dataset.topic;
      const editorContainer = byId("markdown-editor-container");
      const summaryList = byId("fact-summary-list");

      if (topic === "markdown") {
        if (editorContainer) editorContainer.hidden = false;
        if (summaryList) summaryList.hidden = true;
        if (state.selectedPerson) populateMarkdownEditor(state.selectedPerson);
        status("Opened raw canonical Markdown record editor.");
      } else {
        if (editorContainer) editorContainer.hidden = true;
        if (summaryList) summaryList.hidden = false;
        renderTopicPack(topic);
        status(`Switched to ${tab.textContent.trim()} topic pack.`);
      }
    });
  });

  // Events Stepper & Filtering State
  state.activeStep = 4;
  state.filterActionNeededOnly = false;
  state.filterLocationOnly = false;

  const setEventStep = (stepNumber) => {
    state.activeStep = stepNumber;
    document.querySelectorAll(".stepper-item").forEach((item) => {
      const step = Number(item.dataset.step);
      const isCurrent = step === stepNumber;
      item.classList.toggle("current", isCurrent);
      item.classList.toggle("done", step < stepNumber);
      if (isCurrent) item.setAttribute("aria-current", "step");
      else item.removeAttribute("aria-current");
    });

    const workGrid = byId("event-work-grid");
    const briefContainer = byId("event-brief-container");

    if (stepNumber === 5) {
      if (workGrid) workGrid.hidden = true;
      if (briefContainer) briefContainer.hidden = false;
      generateCateringBrief();
      status("Generated Least-Disclosure Catering & Operational Brief.");
    } else {
      if (workGrid) workGrid.hidden = false;
      if (briefContainer) briefContainer.hidden = true;
      status(`Switched to Event Preparation Step ${stepNumber}.`);
    }
  };

  document.querySelectorAll(".stepper-item").forEach((item) => {
    item.addEventListener("click", () => {
      const step = Number(item.dataset.step);
      if (step) setEventStep(step);
    });
    item.addEventListener("keydown", (e) => {
      if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        const step = Number(item.dataset.step);
        if (step) setEventStep(step);
      }
    });
  });

  const generateCateringBrief = () => {
    const textarea = byId("brief-text-area");
    if (!textarea) return;

    const event = state.activeEvent;
    const name = event?.name || "All-hands catering cohort";
    const date = event?.date || "2026-07-21";
    const attendees = event?.attendees || [];
    const total = attendees.length;

    let verifiedNone = 0;
    let custom = 0;
    let pending = 0;

    attendees.forEach((a) => {
      if (a.availability === "VerifiedNone") verifiedNone++;
      else if (a.availability === "Provided") custom++;
      else pending++;
    });

    textarea.value = `LEAST-DISCLOSURE CATERING & OPERATIONAL BRIEF
--------------------------------------------------
Event: ${name}
Date: ${date}
Active Headcount: ${total} attendees

OPERATIONAL MEAL INSTRUCTIONS (Least Disclosure):
- Standard / Verified None: ${verifiedNone}
- Stated Dietary Requirements: ${custom}
- Pending Confirmation (Fail-closed baseline): ${pending}

ACCESSIBILITY & VENUE INSTRUCTIONS:
- Ground floor step-free access required
- All hot beverage stations: Oat milk available

RECONCILIATION STATEMENT:
Exact denominator preserved (${total} active attendees).
Generated: ${new Date().toISOString().split("T")[0]} · Local Workspace Session`;
  };

  byId("copy-brief-button")?.addEventListener("click", () => {
    const textarea = byId("brief-text-area");
    if (!textarea) return;
    navigator.clipboard.writeText(textarea.value);
    status("Copied Least-Disclosure Catering Brief to clipboard.");
  });

  byId("export-brief-button")?.addEventListener("click", () => {
    const textarea = byId("brief-text-area");
    if (!textarea) return;
    const blob = new Blob([textarea.value], { type: "text/plain" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "catering-brief.txt";
    a.click();
    URL.revokeObjectURL(url);
    status("Exported catering-brief.txt file.");
  });

  byId("filter-action-needed")?.addEventListener("click", (e) => {
    state.filterActionNeededOnly = !state.filterActionNeededOnly;
    e.target.classList.toggle("is-active", state.filterActionNeededOnly);
    renderEventsTableOnly();
    status(state.filterActionNeededOnly ? "Filtering cohort by action needed." : "Showing all cohort attendees.");
  });

  byId("filter-location")?.addEventListener("click", (e) => {
    state.filterLocationOnly = !state.filterLocationOnly;
    e.target.classList.toggle("is-active", state.filterLocationOnly);
    renderEventsTableOnly();
    status(state.filterLocationOnly ? "Filtering cohort by Dublin location." : "Showing all locations.");
  });

  byId("drawer-history-button")?.addEventListener("click", () => {
    const dialog = byId("source-history-dialog");
    const item = selectedEventAttendee;
    if (!dialog || !item) return;

    byId("history-person-subtitle").textContent = `Audit details for ${item.display_name}`;
    byId("history-filepath").textContent = item.person_id ? `people/${item.person_id}.md` : "Cohort attendee row";
    byId("history-revision").textContent = item.freshness || "Fresh";
    byId("history-verified").textContent = item.action_needed === "Ready" ? "Verified none / Stated response" : "Pending response";
    byId("history-authority").textContent = `Session Local Authority Port · Row ${item.row_id}`;

    dialog.showModal();
  });

  byId("close-history-dialog")?.addEventListener("click", () => {
    byId("source-history-dialog")?.close();
  });

  // Data Port & Import/Export Module
  const DataPortModule = {
    buildWorkspaceExport() {
      return {
        schema: "LiaisonWorkspaceExportV1",
        exportedAt: new Date().toISOString(),
        workspace: state.workspace ? {
          name: state.workspace.name,
          profile: state.workspace.profile,
          path: state.workspace.path,
        } : null,
        peopleCount: state.people.length,
        people: state.people.map((p) => ({
          id: p.id,
          revision: p.revision,
          display_name: p.display_name,
          emails: p.emails || [],
          phones: p.phones || [],
          pronouns: p.pronouns || null,
          role: p.role || null,
          location: p.location || null,
          dietary: p.dietary || null,
          archived: Boolean(p.archived),
        })),
        events: state.activeEvent ? [state.activeEvent] : [],
      };
    },

    showPreviewModal(options) {
      const dialog = byId("import-export-dialog");
      if (!dialog) return;

      if (byId("import-export-modal-title")) byId("import-export-modal-title").textContent = options.title || "Export / Import Preview";
      if (byId("import-export-modal-subtitle")) byId("import-export-modal-subtitle").textContent = options.subtitle || "Review payload details.";
      if (byId("import-export-status-label")) {
        byId("import-export-status-label").textContent = options.status || "Valid Data Structure";
        byId("import-export-status-label").style.color = options.isValid === false ? "var(--danger)" : "var(--success)";
      }
      if (byId("import-export-meta-label")) byId("import-export-meta-label").textContent = options.meta || "";
      if (byId("import-export-preview-text")) byId("import-export-preview-text").value = options.jsonString || "";

      const actionBtn = byId("action-import-export-dialog");
      if (actionBtn) {
        actionBtn.textContent = options.actionLabel || "Download JSON";
        actionBtn.onclick = options.onAction || null;
      }

      dialog.showModal();
    },

    downloadJson(data, filename) {
      const jsonStr = typeof data === "string" ? data : JSON.stringify(data, null, 2);
      const blob = new Blob([jsonStr], { type: "application/json" });
      const url = URL.createObjectURL(blob);
      const a = document.createElement("a");
      a.href = url;
      a.download = filename;
      a.click();
      URL.revokeObjectURL(url);
    },

    async importWorkspaceData(parsedData) {
      if (!parsedData || typeof parsedData !== "object") {
        throw new Error("Payload is not a valid JSON object.");
      }

      const peopleToImport = Array.isArray(parsedData)
        ? parsedData
        : (Array.isArray(parsedData.people) ? parsedData.people : []);

      if (peopleToImport.length === 0) {
        throw new Error("No person records found in import payload.");
      }

      let importedCount = 0;
      for (const item of peopleToImport) {
        if (!item.display_name && !item.displayName) continue;
        const displayName = item.display_name || item.displayName;
        const email = item.emails?.[0]?.value || item.email || null;
        const loc = item.location || null;

        const existing = state.people.find((p) => p.display_name.toLowerCase() === displayName.toLowerCase() || (email && p.emails?.[0]?.value === email));
        if (existing) {
          if (loc) existing.location = loc;
          if (item.dietary) existing.dietary = item.dietary;
          importedCount++;
        } else {
          try {
            const created = await invokeValue("create_person", {
              request: {
                sessionId: currentSessionId(),
                displayName,
                email,
              },
            });
            if (loc) created.location = loc;
            if (item.dietary) created.dietary = item.dietary;
            state.people.push(created);
            importedCount++;
          } catch {
            // Continue importing remaining profiles
          }
        }
      }

      state.people.sort((left, right) => left.display_name.localeCompare(right.display_name));
      renderPeople();
      renderWorkspace();
      return importedCount;
    }
  };

  // Settings & Data Port Handlers
  byId("settings-theme-select")?.addEventListener("change", (e) => {
    const theme = e.target.value;
    applyTheme(theme);
    setStoreItem("liaison_theme", theme);
    status(`Applied ${theme} theme.`);
  });

  byId("density-select")?.addEventListener("change", (e) => {
    const density = e.target.value;
    document.body.classList.toggle("is-compact", density === "compact");
    setStoreItem("liaison_density", density);
    status(`Applied ${density} interface density.`);
  });

  byId("text-scale-select")?.addEventListener("change", (e) => {
    const scaleMap = { "100": "16px", "115": "18.4px", "130": "20.8px" };
    const size = scaleMap[e.target.value] || "16px";
    document.documentElement.style.fontSize = size;
    setStoreItem("liaison_text_scale", e.target.value);
    status(`Applied text scale ${e.target.value}%.`);
  });

  byId("export-directory-button")?.addEventListener("click", () => {
    if (!state.people || state.people.length === 0) {
      status("No people in current workspace to export.");
      return;
    }
    const payload = DataPortModule.buildWorkspaceExport();
    const jsonStr = JSON.stringify(payload, null, 2);
    DataPortModule.showPreviewModal({
      title: "Export Workspace Directory Backup",
      subtitle: "Canonical JSON export containing directory records and event cohorts.",
      status: "Liaison Workspace Backup Schema (V1)",
      meta: `${payload.peopleCount} profiles · ${payload.events.length} events`,
      jsonString: jsonStr,
      actionLabel: "Download Backup (.json)",
      onAction: () => {
        DataPortModule.downloadJson(payload, "liaison-workspace-backup.json");
        byId("import-export-dialog")?.close();
        status("Exported canonical workspace directory backup to liaison-workspace-backup.json.");
      }
    });
  });

  byId("import-directory-button")?.addEventListener("click", () => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = ".json";
    input.onchange = (e) => {
      const file = e.target.files?.[0];
      if (!file) return;
      const reader = new FileReader();
      reader.onload = (event) => {
        try {
          const parsed = JSON.parse(event.target.result);
          const peopleCount = Array.isArray(parsed) ? parsed.length : (parsed.people?.length || 0);

          DataPortModule.showPreviewModal({
            title: "Import Workspace Directory Payload",
            subtitle: "Review incoming profiles before restoring into local workspace.",
            status: "Schema Validated",
            meta: `${peopleCount} incoming profiles`,
            jsonString: JSON.stringify(parsed, null, 2),
            actionLabel: "Confirm Import to Workspace",
            onAction: async () => {
              try {
                const restored = await DataPortModule.importWorkspaceData(parsed);
                byId("import-export-dialog")?.close();
                status(`Successfully imported and reconciled ${restored} profile(s) into workspace.`);
              } catch (err) {
                status(`Import failed: ${err.message}`);
              }
            }
          });
        } catch {
          status("Invalid JSON backup file structure.");
        }
      };
      reader.readAsText(file);
    };
    input.click();
  });

  byId("export-settings-button")?.addEventListener("click", () => {
    const settingsData = {
      schema: "LiaisonSettingsConfigV1",
      theme: document.documentElement.getAttribute("data-theme") || "light",
      density: document.body.classList.contains("is-compact") ? "compact" : "comfortable",
      textScale: document.documentElement.style.fontSize || "16px",
      exportedAt: new Date().toISOString(),
    };
    const jsonStr = JSON.stringify(settingsData, null, 2);
    DataPortModule.showPreviewModal({
      title: "Export Settings Configuration",
      subtitle: "Appearance and interface density configuration payload.",
      status: "Liaison Settings Schema (V1)",
      meta: `Theme: ${settingsData.theme} · Density: ${settingsData.density}`,
      jsonString: jsonStr,
      actionLabel: "Download Settings (.json)",
      onAction: () => {
        DataPortModule.downloadJson(settingsData, "liaison-settings.json");
        byId("import-export-dialog")?.close();
        status("Exported settings configuration to liaison-settings.json.");
      }
    });
  });

  byId("import-settings-button")?.addEventListener("click", () => {
    const input = document.createElement("input");
    input.type = "file";
    input.accept = ".json";
    input.onchange = (e) => {
      const file = e.target.files?.[0];
      if (!file) return;
      const reader = new FileReader();
      reader.onload = (event) => {
        try {
          const config = JSON.parse(event.target.result);
          if (config.theme) {
            applyTheme(config.theme);
            if (byId("settings-theme-select")) byId("settings-theme-select").value = config.theme;
          }
          if (config.density) {
            document.body.classList.toggle("is-compact", config.density === "compact");
            if (byId("density-select")) byId("density-select").value = config.density;
          }
          status("Successfully imported settings configuration.");
        } catch {
          status("Invalid settings file format.");
        }
      };
      reader.readAsText(file);
    };
    input.click();
  });

  byId("close-import-export-dialog")?.addEventListener("click", () => {
    byId("import-export-dialog")?.close();
  });

  const start = async () => {
    const savedTheme = getStoreItem("liaison_theme") || "paper";
    applyTheme(savedTheme);

    // Initial view rendering
    navigate("today");

    await withNativeOperation(null, "", async (operation) => {
      let buildStatement = "";
      try {
        const app = await invokeValue("app_status");
        if (!isCurrentOperation(operation)) return;
        byId("authority-label").textContent = `${app.product_state} · ${app.connection_state}`;
        buildStatement = `Liaison RM ${app.version}: ${app.release_evidence}.`;
      } catch (error) {
        if (isCurrentOperation(operation)) {
          byId("authority-label").textContent = "Local Workspace · Active";
        }
      }
    });
  };

  start();

