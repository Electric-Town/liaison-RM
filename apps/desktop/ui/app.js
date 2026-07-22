(() => {
  "use strict";

  const APPLICATION_CONTRACT_VERSION = 1;
  const THEME_STORAGE_KEY = "liaison.rm.appearance.v1";
  const THEMES = ["system", "light", "dark", "high_contrast"];
  const THEME_LABELS = {
    system: "System",
    light: "Light",
    dark: "Dark",
    high_contrast: "High contrast",
  };
  const ROUTE_LABELS = {
    setup: "Workspace",
    people: "People",
    health: "Health",
  };

  const state = {
    workspace: null,
    people: [],
    peopleLoading: false,
    peopleQuery: "",
    theme: "system",
    personDialogMode: null,
    selectedPersonId: null,
  };
  const nativeOperation = {
    generation: 0,
    active: null,
    restartRequired: false,
  };
  let dialogReturnFocus = null;
  let mobileNavigationOpen = false;

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
    byId("live-status").textContent = message;
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
    byId("main-content").setAttribute("aria-busy", "true");
    updateControls();
    try {
      return await work(operation);
    } finally {
      if (isCurrentOperation(operation)) {
        nativeOperation.active = null;
        if (button) button.textContent = original;
        byId("main-content").removeAttribute("aria-busy");
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

  const storedTheme = () => {
    try {
      const saved = window.localStorage.getItem(THEME_STORAGE_KEY);
      return THEMES.includes(saved) ? saved : "system";
    } catch (_error) {
      return "system";
    }
  };

  const applyTheme = (theme, { persist = false, announce = false } = {}) => {
    const nextTheme = THEMES.includes(theme) ? theme : "system";
    state.theme = nextTheme;
    document.documentElement.dataset.theme = nextTheme;
    byId("theme-label").textContent = THEME_LABELS[nextTheme];
    byId("theme-toggle").setAttribute(
      "aria-label",
      `Change colour theme. Current theme: ${THEME_LABELS[nextTheme]}`,
    );

    if (!persist) return;
    try {
      window.localStorage.setItem(THEME_STORAGE_KEY, nextTheme);
      if (announce) {
        status(`Appearance changed to ${THEME_LABELS[nextTheme]} and will be reused on this device.`);
      }
    } catch (_error) {
      if (announce) {
        status(`Appearance changed to ${THEME_LABELS[nextTheme]} for this session. This device could not retain the choice.`);
      }
    }
  };

  const narrowNavigation = window.matchMedia("(max-width: 760px)");

  const syncNavigationForViewport = () => {
    const toggle = byId("sections-toggle");
    const navigation = byId("primary-navigation");
    if (narrowNavigation.matches) {
      toggle.hidden = false;
      navigation.hidden = !mobileNavigationOpen;
      toggle.setAttribute("aria-expanded", String(mobileNavigationOpen));
    } else {
      toggle.hidden = true;
      navigation.hidden = false;
      toggle.setAttribute("aria-expanded", "false");
      mobileNavigationOpen = false;
    }
  };

  const navigate = (route) => {
    const destination = document.querySelector(`[data-page="${route}"]`);
    if (!destination || !(route in ROUTE_LABELS)) return;
    document.querySelectorAll("[data-page]").forEach((page) => {
      page.hidden = page !== destination;
    });
    document.querySelectorAll("[data-route]").forEach((button) => {
      const active = button.dataset.route === route;
      button.classList.toggle("is-active", active);
      if (active) button.setAttribute("aria-current", "page");
      else button.removeAttribute("aria-current");
    });
    byId("current-section-label").textContent = ROUTE_LABELS[route];
    if (narrowNavigation.matches) {
      mobileNavigationOpen = false;
      syncNavigationForViewport();
    }
    destination.querySelector("h1")?.focus();
  };

  const profileLabel = (value) => ({
    personal: "Personal",
    family: "Family",
    team: "Team",
    workplace: "Workplace",
  }[value] || value);

  const summaryRow = (term, description) => {
    const row = document.createElement("div");
    const dt = document.createElement("dt");
    const dd = document.createElement("dd");
    dt.textContent = term;
    dd.textContent = description;
    row.append(dt, dd);
    return row;
  };

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

  const initials = (name) => name
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() || "")
    .join("") || "?";

  const primaryContact = (items) => items?.[0]?.value || "Not recorded";

  const matchingPeople = () => {
    const query = state.peopleQuery.trim().toLocaleLowerCase();
    if (!query) return state.people;
    return state.people.filter((person) => {
      const values = [
        person.display_name,
        ...(person.aliases || []),
        ...(person.emails || []).map((email) => email.value),
        ...(person.phones || []).map((phone) => phone.value),
      ];
      return values.some((value) => String(value).toLocaleLowerCase().includes(query));
    });
  };

  const emptyPeopleRow = (message) => {
    const row = document.createElement("tr");
    const cell = document.createElement("td");
    cell.className = "empty-state";
    cell.colSpan = 5;
    cell.textContent = message;
    row.append(cell);
    return row;
  };

  const personCell = (label, text, className = "") => {
    const cell = document.createElement("td");
    cell.dataset.label = label;
    if (className) cell.className = className;
    cell.textContent = text;
    return cell;
  };

  const renderPersonRow = (person) => {
    const row = document.createElement("tr");
    row.className = "person-row";
    row.dataset.personId = person.id;

    const identity = document.createElement("td");
    identity.dataset.label = "Person";
    const identityLayout = document.createElement("span");
    identityLayout.className = "person-identity";
    const avatar = document.createElement("span");
    avatar.className = "person-avatar";
    avatar.setAttribute("aria-hidden", "true");
    avatar.textContent = initials(person.display_name);
    const details = document.createElement("span");
    const name = document.createElement("strong");
    name.textContent = person.display_name;
    details.append(name);
    if (person.aliases?.length) {
      const alias = document.createElement("small");
      alias.textContent = person.aliases.join(", ");
      details.append(alias);
    }
    identityLayout.append(avatar, details);
    identity.append(identityLayout);

    const revision = personCell("Revision", String(person.revision), "revision");
    const action = document.createElement("td");
    action.dataset.label = "Action";
    action.className = "row-action";
    const edit = document.createElement("button");
    edit.type = "button";
    edit.className = "quiet-button edit-person";
    edit.dataset.personId = person.id;
    edit.dataset.nativeOperation = "session";
    edit.textContent = "Edit";
    edit.setAttribute("aria-label", `Edit ${person.display_name}`);
    action.append(edit);

    row.append(
      identity,
      personCell("Email", primaryContact(person.emails)),
      personCell("Phone", primaryContact(person.phones)),
      revision,
      action,
    );
    return row;
  };

  const renderPeople = () => {
    const list = byId("people-list");
    const count = byId("people-count-summary");
    const visiblePeople = matchingPeople();
    list.replaceChildren();

    if (state.peopleLoading) {
      count.textContent = "Loading people…";
      list.append(emptyPeopleRow("Loading profiles from the open workspace…"));
    } else if (!state.workspace) {
      count.textContent = "No people loaded.";
      list.append(emptyPeopleRow("Open a workspace to load its people."));
    } else if (state.people.length === 0) {
      count.textContent = "0 people in this workspace.";
      list.append(emptyPeopleRow("No people yet. Add the first local profile."));
    } else if (visiblePeople.length === 0) {
      count.textContent = `Showing 0 of ${state.people.length} people.`;
      list.append(emptyPeopleRow(`No people match “${state.peopleQuery.trim()}”.`));
    } else {
      count.textContent = state.peopleQuery.trim()
        ? `Showing ${visiblePeople.length} of ${state.people.length} people.`
        : `${state.people.length} ${state.people.length === 1 ? "person" : "people"} in this workspace.`;
      visiblePeople.forEach((person) => list.append(renderPersonRow(person)));
    }
    byId("clear-people-search").disabled = state.peopleQuery.length === 0;
    updateControls();
  };

  const updateControls = () => {
    const ready = Boolean(state.workspace);
    const busy = Boolean(nativeOperation.active) || nativeOperation.restartRequired;
    document.querySelectorAll("[data-native-operation]").forEach((control) => {
      const needsSession = control.dataset.nativeOperation === "session";
      control.disabled = busy || (needsSession && !ready);
    });
    document.querySelectorAll("[data-operation-lock]").forEach((control) => {
      control.disabled = busy;
    });
    ["person-name", "person-email", "person-phone"].forEach((id) => {
      byId(id).disabled = busy || !ready;
    });
    byId("cancel-person").disabled = Boolean(nativeOperation.active);
    byId("people-workspace-warning").hidden = ready;
  };

  const refreshPeople = async (operation) => {
    if (!operation.sessionId) return false;
    state.peopleLoading = true;
    renderPeople();
    try {
      const people = await invokeValue("list_people", {
        request: { sessionId: operation.sessionId },
      });
      if (!operationOwnsCurrentSession(operation)) return false;
      state.people = people;
      return true;
    } finally {
      if (operationOwnsCurrentSession(operation)) {
        state.peopleLoading = false;
        renderPeople();
        renderWorkspace();
      }
    }
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
    if (byId("person-dialog").open) byId("person-dialog").close();
    state.workspace = opened.workspace;
    state.people = opened.people;
    state.peopleLoading = false;
    state.peopleQuery = "";
    byId("people-search").value = "";
    byId("workspace-path").value = state.workspace.path;
    showValidation(opened.validation, state.workspace.path, "Active workspace");
    renderPeople();
    renderWorkspace();
    updateControls();
    const health = opened.validation.valid
      ? "Workspace Health is valid."
      : `Workspace Health reports ${opened.validation.findings.length} finding${opened.validation.findings.length === 1 ? "" : "s"}; review Health before editing.`;
    status(`${action}: ${state.workspace.name}. ${state.people.length} ${state.people.length === 1 ? "person" : "people"} loaded. ${health}`);
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

  const editPrimaryContact = (items, value, defaultLabel) => {
    const next = (items || []).map((item) => ({ ...item }));
    const trimmed = value.trim();
    if (!trimmed) {
      if (next.length) next.shift();
      return next;
    }
    if (next.length) {
      next[0] = { ...next[0], value: trimmed };
    } else {
      next.push({ value: trimmed, label: defaultLabel });
    }
    return next;
  };

  const showPersonFormError = (message = "") => {
    const error = byId("person-form-error");
    error.textContent = message;
    error.hidden = !message;
  };

  const openPersonDialog = (mode, invoker, person = null) => {
    if (!state.workspace || nativeOperation.active || nativeOperation.restartRequired) return;
    state.personDialogMode = mode;
    state.selectedPersonId = person?.id || null;
    dialogReturnFocus = invoker;
    showPersonFormError();

    const form = byId("person-form");
    const title = byId("person-dialog-title");
    const description = byId("person-dialog-description");
    const revision = byId("person-dialog-revision");
    const phoneField = byId("person-phone-field");
    const contactHelp = byId("person-contact-help");
    form.reset();

    if (mode === "edit" && person) {
      title.textContent = "Edit person";
      description.textContent = "Save changes through the open workspace session.";
      revision.textContent = `Revision ${person.revision}`;
      revision.hidden = false;
      phoneField.hidden = false;
      contactHelp.hidden = false;
      byId("person-name").value = person.display_name;
      byId("person-email").value = person.emails?.[0]?.value || "";
      byId("person-phone").value = person.phones?.[0]?.value || "";
      byId("save-person").textContent = "Save changes";
    } else {
      title.textContent = "Add a person";
      description.textContent = "Create a basic profile in the open workspace.";
      revision.hidden = true;
      phoneField.hidden = true;
      contactHelp.hidden = true;
      byId("save-person").textContent = "Create profile";
    }

    updateControls();
    byId("person-dialog").showModal();
    window.requestAnimationFrame(() => byId("person-name").focus());
  };

  const closePersonDialog = () => {
    const dialog = byId("person-dialog");
    if (dialog.open) dialog.close();
  };

  const sortedPeople = (people) => people.sort((left, right) => (
    left.display_name.localeCompare(right.display_name)
  ));

  document.querySelectorAll("[data-route]").forEach((button) => {
    button.addEventListener("click", () => navigate(button.dataset.route));
  });

  byId("sections-toggle").addEventListener("click", () => {
    mobileNavigationOpen = !mobileNavigationOpen;
    syncNavigationForViewport();
    if (mobileNavigationOpen) {
      byId("primary-navigation").querySelector("[aria-current=\"page\"]")?.focus();
    }
  });

  if (typeof narrowNavigation.addEventListener === "function") {
    narrowNavigation.addEventListener("change", syncNavigationForViewport);
  } else {
    narrowNavigation.addListener(syncNavigationForViewport);
  }

  byId("theme-toggle").addEventListener("click", () => {
    if (nativeOperation.active || nativeOperation.restartRequired) return;
    const nextIndex = (THEMES.indexOf(state.theme) + 1) % THEMES.length;
    applyTheme(THEMES[nextIndex], { persist: true, announce: true });
  });

  byId("use-default-path").addEventListener("click", async (event) => {
    await withNativeOperation(event.currentTarget, "Selecting…", async (operation) => {
      try {
        const path = await invokeValue("default_workspace_path");
        if (!isCurrentOperation(operation)) return;
        byId("workspace-path").value = path;
        status("Suggested the local Documents folder. Review it before creating the workspace.");
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
        const opened = await invokeValue("initialise_workspace", { request });
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
        const report = await invokeValue("inspect_workspace_health", { path: inspectedPath });
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

  byId("people-search").addEventListener("input", (event) => {
    state.peopleQuery = event.currentTarget.value;
    renderPeople();
  });

  byId("clear-people-search").addEventListener("click", () => {
    state.peopleQuery = "";
    byId("people-search").value = "";
    renderPeople();
    byId("people-search").focus();
  });

  byId("add-person").addEventListener("click", (event) => {
    openPersonDialog("create", event.currentTarget);
  });

  byId("people-list").addEventListener("click", (event) => {
    const button = event.target.closest(".edit-person");
    if (!button) return;
    const person = state.people.find((candidate) => candidate.id === button.dataset.personId);
    if (person) openPersonDialog("edit", button, person);
  });

  byId("cancel-person").addEventListener("click", closePersonDialog);

  byId("person-dialog").addEventListener("cancel", (event) => {
    if (nativeOperation.active) event.preventDefault();
  });

  byId("person-dialog").addEventListener("close", () => {
    state.personDialogMode = null;
    state.selectedPersonId = null;
    showPersonFormError();
    const focusTarget = dialogReturnFocus;
    dialogReturnFocus = null;
    if (focusTarget?.isConnected && !focusTarget.disabled && focusTarget.offsetParent !== null) {
      focusTarget.focus({ preventScroll: true });
    }
  });

  byId("person-form").addEventListener("submit", async (event) => {
    event.preventDefault();
    if (!state.workspace) return;
    const button = event.submitter || byId("save-person");
    const mode = state.personDialogMode;
    const selectedPersonId = state.selectedPersonId;
    showPersonFormError();

    await withNativeOperation(button, "Saving…", async (operation) => {
      try {
        if (!operation.sessionId) return;
        let person;
        if (mode === "edit") {
          const current = state.people.find((candidate) => candidate.id === selectedPersonId);
          if (!current) {
            throw new Error("The selected person is no longer in this workspace. Refresh People and retry.");
          }
          person = await invokeValue("update_person", {
            request: {
              sessionId: operation.sessionId,
              personId: current.id,
              expectedRevision: current.revision,
              displayName: byId("person-name").value,
              emails: editPrimaryContact(current.emails, byId("person-email").value, "primary"),
              phones: editPrimaryContact(current.phones, byId("person-phone").value, "mobile"),
            },
          });
          if (!operationOwnsCurrentSession(operation)) return;
          state.people = sortedPeople(state.people.map((candidate) => (
            candidate.id === person.id ? person : candidate
          )));
          renderPeople();
          renderWorkspace();
          closePersonDialog();
          status(`Saved changes to ${person.display_name}. Revision ${person.revision}.`);
        } else {
          person = await invokeValue("create_person", {
            request: {
              sessionId: operation.sessionId,
              displayName: byId("person-name").value,
              email: byId("person-email").value || null,
            },
          });
          if (!operationOwnsCurrentSession(operation)) return;
          state.people = sortedPeople([...state.people, person]);
          renderPeople();
          renderWorkspace();
          closePersonDialog();
          status(`Saved ${person.display_name} as a local Markdown profile.`);
        }
      } catch (error) {
        if (operationOwnsCurrentSession(operation)) {
          const message = `Person was not saved: ${errorText(error)}`;
          showPersonFormError(message);
          status(message);
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

  const start = async () => {
    applyTheme(storedTheme());
    syncNavigationForViewport();
    updateControls();
    renderWorkspace();
    renderPeople();
    await withNativeOperation(null, "", async (operation) => {
      try {
        const app = await invokeValue("app_status");
        if (!isCurrentOperation(operation)) return;
        byId("authority-label").textContent = `${app.product_state} · ${app.connection_state}`;
        status(`Liaison RM ${app.version}: ${app.release_evidence}. No workspace has been opened.`);
      } catch (error) {
        if (isCurrentOperation(operation)) {
          byId("authority-label").textContent = "Native bridge unavailable";
          status(errorText(error));
        }
        return;
      }
      try {
        const path = await invokeValue("default_workspace_path");
        if (!isCurrentOperation(operation)) return;
        byId("workspace-path").value = path;
      } catch (error) {
        if (isCurrentOperation(operation)) {
          status(`A default workspace folder was not selected: ${errorText(error)}`);
        }
      }
    });
  };

  start();
})();
