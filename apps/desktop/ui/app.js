(() => {
  "use strict";

  const APPLICATION_CONTRACT_VERSION = 1;
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
    selectedPersonId: null,
  };
  const nativeOperation = {
    generation: 0,
    active: null,
    restartRequired: false,
  };
  let dialogReturnFocus = null;
  let detailReturnFocus = null;
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

  const narrowNavigation = window.matchMedia("(max-width: 900px)");

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
    if (byId("person-detail-dialog").open) byId("person-detail-dialog").close();
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
    byId("workspace-path-label").textContent = `${state.workspace.name} · ${profileLabel(state.workspace.profile)}`;
  };

  const initials = (name) => name
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2)
    .map((part) => part[0]?.toUpperCase() || "")
    .join("") || "?";

  const primaryContact = (items) => items?.[0]?.value || "";
  const informationValue = (value) => String(value || "").trim() || "Unknown in current profile";

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

  const clearPeopleSearch = () => {
    state.peopleQuery = "";
    byId("people-search").value = "";
    renderPeople();
    byId("people-search").focus();
  };

  const emptyPeopleRow = (message, actionLabel = "", action = null) => {
    const row = document.createElement("tr");
    const cell = document.createElement("td");
    cell.className = "empty-state";
    cell.colSpan = 3;
    const description = document.createElement("p");
    description.textContent = message;
    cell.append(description);
    if (actionLabel && action) {
      const button = document.createElement("button");
      button.className = "secondary-button empty-state-action";
      button.type = "button";
      button.dataset.directoryAction = "true";
      button.textContent = actionLabel;
      button.addEventListener("click", () => action(button));
      cell.append(button);
    }
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
    const selected = person.id === state.selectedPersonId;
    const row = document.createElement("tr");
    row.className = "person-row";
    row.dataset.personId = person.id;
    row.classList.toggle("is-selected", selected);

    const identity = document.createElement("td");
    identity.dataset.label = "Person";
    const select = document.createElement("button");
    select.type = "button";
    select.className = "person-select";
    select.dataset.personSelect = "true";
    select.dataset.personId = person.id;
    select.setAttribute("aria-controls", narrowNavigation.matches ? "person-detail-dialog" : "person-detail");
    select.setAttribute("aria-pressed", String(selected));
    select.setAttribute("aria-label", `View ${person.display_name}`);
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
    select.append(identityLayout);
    identity.append(select);

    const email = primaryContact(person.emails);
    const phone = primaryContact(person.phones);
    const contact = email || phone || "Unknown in current profile";
    const revision = personCell("Record", `r${person.revision}`, "revision");
    revision.setAttribute("aria-label", `Revision ${person.revision}`);

    row.append(
      identity,
      personCell("Contact", contact, "person-contact"),
      revision,
    );
    return row;
  };

  const detailRow = (term, description) => {
    const row = document.createElement("div");
    const dt = document.createElement("dt");
    const dd = document.createElement("dd");
    dt.textContent = term;
    dd.textContent = informationValue(description);
    row.append(dt, dd);
    return row;
  };

  const birthdayLabel = (birthday) => {
    if (!birthday || typeof birthday !== "object") return "";
    if (birthday.precision === "full" && birthday.date) return String(birthday.date);
    if (birthday.precision === "month-day" && birthday.month && birthday.day) {
      const date = new Date(Date.UTC(2000, Number(birthday.month) - 1, Number(birthday.day)));
      if (!Number.isNaN(date.getTime())) {
        return new Intl.DateTimeFormat(document.documentElement.lang || undefined, {
          day: "numeric",
          month: "long",
          timeZone: "UTC",
        }).format(date);
      }
    }
    return "";
  };

  const renderPersonDetailInto = (
    detail,
    headingId,
    person,
    emptyMessage = "Select a person from the directory to inspect their local profile.",
  ) => {
    detail.replaceChildren();
    if (!person) {
      const empty = document.createElement("div");
      empty.className = "detail-empty";
      const eyebrow = document.createElement("p");
      eyebrow.className = "eyebrow";
      eyebrow.textContent = "Record detail";
      const heading = document.createElement("h2");
      heading.id = headingId;
      heading.tabIndex = -1;
      heading.textContent = "No person selected";
      const description = document.createElement("p");
      description.textContent = emptyMessage;
      empty.append(eyebrow, heading, description);
      detail.append(empty);
      return;
    }

    const header = document.createElement("header");
    header.className = "person-detail-header";
    const avatar = document.createElement("span");
    avatar.className = "person-detail-avatar";
    avatar.setAttribute("aria-hidden", "true");
    avatar.textContent = initials(person.display_name);
    const titleGroup = document.createElement("div");
    const eyebrow = document.createElement("p");
    eyebrow.className = "eyebrow";
    eyebrow.textContent = "Canonical person record";
    const heading = document.createElement("h2");
    heading.id = headingId;
    heading.tabIndex = -1;
    heading.textContent = person.display_name;
    const recordState = document.createElement("p");
    recordState.className = "record-state";
    recordState.textContent = person.archived ? "Archived record" : "Active record";
    titleGroup.append(eyebrow, heading, recordState);
    header.append(avatar, titleGroup);

    const facts = document.createElement("dl");
    facts.className = "person-detail-list";
    facts.append(
      detailRow("Primary email", primaryContact(person.emails)),
      detailRow("Primary phone", primaryContact(person.phones)),
      detailRow("Known as", person.aliases?.join(", ")),
      detailRow("Birthday", birthdayLabel(person.birthday)),
      detailRow("Revision", `Revision ${person.revision}`),
      detailRow("Person ID", person.id),
    );

    const provenance = document.createElement("section");
    provenance.className = "detail-provenance";
    const provenanceLabel = document.createElement("p");
    provenanceLabel.className = "eyebrow";
    provenanceLabel.textContent = "Source";
    const provenanceHeading = document.createElement("h3");
    provenanceHeading.textContent = "Local Markdown profile";
    const provenanceText = document.createElement("p");
    provenanceText.textContent = "Loaded through the active workspace session. Rebuildable indexes are not the canonical record.";
    provenance.append(provenanceLabel, provenanceHeading, provenanceText);

    detail.append(header, facts, provenance);
  };

  const renderPersonDetail = (person, emptyMessage) => {
    renderPersonDetailInto(byId("person-detail"), "person-detail-heading", person, emptyMessage);
    renderPersonDetailInto(
      byId("person-detail-dialog-content"),
      "person-detail-dialog-heading",
      person,
      emptyMessage,
    );
  };

  const renderPeople = () => {
    const list = byId("people-list");
    const count = byId("people-count-summary");
    const visiblePeople = matchingPeople();
    list.replaceChildren();
    byId("people-count").textContent = `${state.people.length} ${state.people.length === 1 ? "person" : "people"}`;

    if (state.peopleLoading) {
      count.textContent = "Loading people…";
      list.append(emptyPeopleRow("Loading profiles from the open workspace…"));
      renderPersonDetail(null, "People are refreshing from the active workspace.");
    } else if (!state.workspace) {
      count.textContent = "No people loaded.";
      state.selectedPersonId = null;
      list.append(emptyPeopleRow("Open a workspace to load its people.", "Open workspace", () => navigate("setup")));
      renderPersonDetail(null, "Open a workspace, then select a person to inspect their local profile.");
    } else if (state.people.length === 0) {
      count.textContent = "0 people in this workspace.";
      state.selectedPersonId = null;
      list.append(emptyPeopleRow("No people yet. Add the first local profile.", "Add first person", openPersonDialog));
      renderPersonDetail(null, "Add a person to create the first local profile in this workspace.");
    } else if (visiblePeople.length === 0) {
      count.textContent = `Showing 0 of ${state.people.length} people.`;
      list.append(emptyPeopleRow(
        `No people match “${state.peopleQuery.trim()}”. Try another name, email, phone, or alias, or clear the search.`,
      ));
      renderPersonDetail(null, "Clear or change the search to select a person.");
    } else {
      if (!visiblePeople.some((person) => person.id === state.selectedPersonId)) {
        state.selectedPersonId = visiblePeople[0].id;
      }
      count.textContent = state.peopleQuery.trim()
        ? `Showing ${visiblePeople.length} of ${state.people.length} people.`
        : `${state.people.length} ${state.people.length === 1 ? "person" : "people"} in this workspace.`;
      visiblePeople.forEach((person) => list.append(renderPersonRow(person)));
      renderPersonDetail(visiblePeople.find((person) => person.id === state.selectedPersonId));
    }
    updateControls();
  };

  const updateControls = () => {
    const ready = Boolean(state.workspace);
    const busy = Boolean(nativeOperation.active) || nativeOperation.restartRequired;
    document.querySelectorAll("[data-native-operation]").forEach((control) => {
      const needsSession = control.dataset.nativeOperation === "session";
      control.disabled = busy || (needsSession && !ready);
    });
    ["person-name", "person-email"].forEach((id) => {
      byId(id).disabled = busy || !ready;
    });
    byId("people-search").disabled = busy || !ready;
    byId("clear-people-search").disabled = busy || !ready || state.peopleQuery.length === 0;
    document.querySelectorAll("[data-person-select], [data-directory-action]").forEach((control) => {
      control.disabled = busy;
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
    if (byId("person-detail-dialog").open) byId("person-detail-dialog").close();
    state.workspace = opened.workspace;
    state.people = opened.people;
    state.peopleLoading = false;
    state.peopleQuery = "";
    state.selectedPersonId = state.people[0]?.id || null;
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

  const showPersonFormError = (message = "") => {
    const error = byId("person-form-error");
    error.textContent = message;
    error.hidden = !message;
  };

  const openPersonDialog = (invoker) => {
    if (!state.workspace || nativeOperation.active || nativeOperation.restartRequired) return;
    dialogReturnFocus = invoker;
    showPersonFormError();
    byId("person-form").reset();
    updateControls();
    byId("person-dialog").showModal();
    byId("person-name").focus({ preventScroll: true });
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

  const handleViewportChange = () => {
    syncNavigationForViewport();
    if (!narrowNavigation.matches && byId("person-detail-dialog").open) {
      byId("person-detail-dialog").close();
    }
    renderPeople();
  };

  if (typeof narrowNavigation.addEventListener === "function") {
    narrowNavigation.addEventListener("change", handleViewportChange);
  } else {
    narrowNavigation.addListener(handleViewportChange);
  }

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

  byId("clear-people-search").addEventListener("click", clearPeopleSearch);

  byId("add-person").addEventListener("click", (event) => {
    openPersonDialog(event.currentTarget);
  });

  byId("people-list").addEventListener("click", (event) => {
    const button = event.target.closest("[data-person-select]");
    if (!button) return;
    const openInDialog = narrowNavigation.matches;
    state.selectedPersonId = button.dataset.personId;
    renderPeople();
    window.requestAnimationFrame(() => {
      const focusTarget = [...document.querySelectorAll("[data-person-select]")]
        .find((candidate) => candidate.dataset.personId === state.selectedPersonId);
      if (openInDialog) {
        detailReturnFocus = focusTarget;
        byId("person-detail-dialog").showModal();
        byId("person-detail-dialog-heading")?.focus({ preventScroll: true });
      } else {
        focusTarget?.focus({ preventScroll: true });
      }
    });
  });

  byId("close-person-detail").addEventListener("click", () => {
    byId("person-detail-dialog").close();
  });

  byId("person-detail-dialog").addEventListener("close", () => {
    const focusTarget = detailReturnFocus;
    detailReturnFocus = null;
    if (focusTarget?.isConnected && !focusTarget.disabled) {
      focusTarget.focus({ preventScroll: true });
    }
  });

  byId("cancel-person").addEventListener("click", closePersonDialog);

  byId("person-dialog").addEventListener("cancel", (event) => {
    if (nativeOperation.active) event.preventDefault();
  });

  byId("person-dialog").addEventListener("close", () => {
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
    showPersonFormError();

    await withNativeOperation(button, "Saving…", async (operation) => {
      try {
        if (!operation.sessionId) return;
        const person = await invokeValue("create_person", {
          request: {
            sessionId: operation.sessionId,
            displayName: byId("person-name").value,
            email: byId("person-email").value || null,
          },
        });
        if (!operationOwnsCurrentSession(operation)) return;
        state.people = sortedPeople([...state.people, person]);
        state.selectedPersonId = person.id;
        renderPeople();
        renderWorkspace();
        closePersonDialog();
        status(`Saved ${person.display_name} as a local Markdown profile.`);
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
    syncNavigationForViewport();
    updateControls();
    renderWorkspace();
    renderPeople();
    await withNativeOperation(null, "", async (operation) => {
      try {
        const app = await invokeValue("app_status");
        if (!isCurrentOperation(operation)) return;
        document.querySelector(".status-dot")?.classList.remove("is-error");
        byId("authority-label").textContent = `${app.product_state} · ${app.connection_state}`;
        status(`Liaison RM ${app.version}: ${app.release_evidence}. No workspace has been opened.`);
      } catch (error) {
        if (isCurrentOperation(operation)) {
          document.querySelector(".status-dot")?.classList.add("is-error");
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
