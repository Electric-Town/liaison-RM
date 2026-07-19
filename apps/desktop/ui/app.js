(() => {
  "use strict";

  const state = {
    workspace: null,
    people: [],
    selectedPersonId: null,
    query: "",
    filter: "active",
    sort: "name",
    page: 1,
  };
  const PAGE_SIZE = 8;
  const COLUMN_PREFERENCE_KEY = "liaison.people.columns.v1";

  /**
   * The Directory column registry.
   *
   * One declarative entry per column keeps the table modular: presentation
   * reads this list and never hard-codes a field. A column becomes live by
   * supplying `value`; until its owning context lands it declares `pending`
   * with the reason, so the gap is visible in the chooser instead of being
   * quietly missing or filled with invented data.
   */
  const PEOPLE_COLUMNS = [
    {
      id: "person",
      label: "Person",
      required: true,
      value: (person) => person.display_name,
    },
    {
      id: "email",
      label: "Email",
      defaultVisible: true,
      value: (person) => person.emails?.[0]?.value || "None recorded",
    },
    {
      id: "phone",
      label: "Phone",
      defaultVisible: false,
      value: (person) => person.phones?.[0]?.value || "None recorded",
    },
    {
      id: "important-date",
      label: "Important date",
      defaultVisible: true,
      value: (person) => birthdayText(person.birthday) || "None recorded",
    },
    {
      id: "contact-points",
      label: "Contact points",
      defaultVisible: false,
      numeric: true,
      value: (person) => String((person.emails?.length || 0) + (person.phones?.length || 0)),
    },
    {
      id: "status",
      label: "Status",
      defaultVisible: true,
      value: (person) => (person.archived ? "Archived" : "Active"),
    },
    {
      id: "revision",
      label: "Revision",
      defaultVisible: true,
      numeric: true,
      value: (person) => `Revision ${person.revision}`,
    },
    {
      id: "record-id",
      label: "Record identifier",
      defaultVisible: false,
      provenance: true,
      value: (person) => person.id,
    },
    // Declared, not invented. Each becomes live by adding `value` when its
    // owning context supplies the fact.
    { id: "role", label: "Role and organisation", pending: "Organisations context" },
    { id: "relationship", label: "Relationship type", pending: "Relationships context, A0" },
    { id: "last-interaction", label: "Last interaction", pending: "Interactions context, A0" },
    { id: "notes", label: "Notes", pending: "Notes context, A0" },
  ];

  const availableColumns = () => PEOPLE_COLUMNS.filter((column) => !column.pending);
  const pendingColumns = () => PEOPLE_COLUMNS.filter((column) => column.pending);

  // Column choice is per-session presentation state held in memory. Durable
  // settings persistence is A0 G2c work (LRM-WS-013/LRM-WS-014); the desktop
  // shell must not adopt browser storage as an authority, so the preference
  // deliberately resets on relaunch rather than being written anywhere.
  const defaultColumnPreference = () =>
    availableColumns()
      .filter((column) => column.required || column.defaultVisible)
      .map((column) => column.id);

  state.visibleColumns = defaultColumnPreference();
  const visibleColumns = () =>
    availableColumns().filter(
      (column) => column.required || state.visibleColumns.includes(column.id),
    );
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

  const navigate = (route) => {
    document.querySelectorAll("[data-page]").forEach((page) => {
      page.hidden = page.dataset.page !== route;
    });
    document.querySelectorAll("[data-route]").forEach((button) => {
      const active = button.dataset.route === route;
      button.classList.toggle("is-active", active);
      if (active) button.setAttribute("aria-current", "page");
      else button.removeAttribute("aria-current");
    });
    const heading = document.querySelector(`[data-page="${route}"] h1`);
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

  const matchesQuery = (person) => {
    const query = state.query.trim().toLowerCase();
    if (!query) return true;
    const haystack = [
      person.display_name,
      ...(person.emails || []).map((email) => email.value),
      ...(person.aliases || []),
    ]
      .join(" ")
      .toLowerCase();
    return haystack.includes(query);
  };

  const matchesFilter = (person) => {
    if (state.filter === "all") return true;
    return state.filter === "archived" ? person.archived : !person.archived;
  };

  const sortedPeople = (people) => {
    const byName = (left, right) => left.display_name.localeCompare(right.display_name);
    if (state.sort === "name-desc") return people.slice().sort((l, r) => byName(r, l));
    if (state.sort === "revision") {
      return people.slice().sort((l, r) => r.revision - l.revision || byName(l, r));
    }
    return people.slice().sort(byName);
  };

  const visiblePeople = () => sortedPeople(state.people.filter((person) => matchesQuery(person) && matchesFilter(person)));

  const renderColumnChooser = () => {
    const options = byId("column-options");
    options.replaceChildren();
    availableColumns().forEach((column) => {
      const label = document.createElement("label");
      label.className = "column-option";
      const input = document.createElement("input");
      input.type = "checkbox";
      input.checked = column.required || state.visibleColumns.includes(column.id);
      input.disabled = Boolean(column.required);
      input.dataset.columnId = column.id;
      const text = document.createElement("span");
      text.textContent = column.required ? `${column.label} (always shown)` : column.label;
      label.append(input, text);
      options.append(label);
    });
    const pending = pendingColumns();
    byId("column-pending-note").textContent = pending.length
      ? `Not available yet: ${pending.map((column) => `${column.label} (${column.pending})`).join("; ")}.`
      : "";
  };

  const renderPeople = () => {
    const head = byId("people-head");
    const body = byId("people-list");
    const columns = visibleColumns();
    const matched = visiblePeople();
    const pageCount = Math.max(1, Math.ceil(matched.length / PAGE_SIZE));
    if (state.page > pageCount) state.page = pageCount;
    const start = (state.page - 1) * PAGE_SIZE;
    const pageRows = matched.slice(start, start + PAGE_SIZE);

    byId("people-count").textContent = String(state.people.length);
    head.replaceChildren(
      ...columns.map((column) => {
        const cell = document.createElement("th");
        cell.scope = "col";
        cell.textContent = column.label;
        if (column.numeric) cell.className = "is-numeric";
        return cell;
      }),
      (() => {
        // The header stays in normal table flow; only its text is hidden.
        // Classing the cell itself would take it out of flow and break the row.
        const cell = document.createElement("th");
        cell.scope = "col";
        const label = document.createElement("span");
        label.className = "visually-hidden";
        label.textContent = "Actions";
        cell.append(label);
        return cell;
      })(),
    );

    body.replaceChildren();
    if (pageRows.length === 0) {
      const row = document.createElement("tr");
      const cell = document.createElement("td");
      cell.colSpan = columns.length + 1;
      cell.className = "empty-state";
      cell.textContent = !state.workspace
        ? "No people loaded."
        : state.people.length === 0
          ? "No people yet. Add the first profile."
          : "No people match this search and filter.";
      row.append(cell);
      body.append(row);
    }
    pageRows.forEach((person) => {
      const row = document.createElement("tr");
      if (state.selectedPersonId === person.id) row.classList.add("is-selected");
      columns.forEach((column) => {
        const cell = document.createElement("td");
        if (column.id === "person") {
          const identity = document.createElement("span");
          identity.className = "cell-identity";
          const avatar = document.createElement("span");
          avatar.className = "person-avatar is-small";
          avatar.setAttribute("aria-hidden", "true");
          avatar.textContent = initials(person.display_name);
          const name = document.createElement("strong");
          name.textContent = person.display_name;
          identity.append(avatar, name);
          cell.append(identity);
        } else {
          cell.textContent = column.value(person);
          if (column.provenance) cell.className = "is-provenance";
          if (column.numeric) cell.className = "is-numeric";
        }
        row.append(cell);
      });
      const actions = document.createElement("td");
      const open = document.createElement("button");
      open.type = "button";
      open.className = "secondary-button row-action";
      open.dataset.personId = person.id;
      open.setAttribute("aria-pressed", String(state.selectedPersonId === person.id));
      open.textContent = "View profile";
      actions.append(open);
      row.append(actions);
      body.append(row);
    });

    // The reconciled sentence comes before the table and always accounts for
    // every record: shown, filtered out, and archived.
    const archived = state.people.filter((person) => person.archived).length;
    byId("people-reconciliation").textContent = state.people.length === 0
      ? "No people recorded yet."
      : `${state.people.length} recorded · ${matched.length} match the current search and filter · ${archived} archived.`;
    byId("people-range").textContent = matched.length === 0
      ? "Showing none."
      : `Showing ${start + 1}–${start + pageRows.length} of ${matched.length}.`;

    const pages = byId("people-pages");
    pages.replaceChildren();
    if (pageCount > 1) {
      for (let number = 1; number <= pageCount; number += 1) {
        const button = document.createElement("button");
        button.type = "button";
        button.className = "page-button";
        button.textContent = String(number);
        button.dataset.page = String(number);
        if (number === state.page) {
          button.classList.add("is-active");
          button.setAttribute("aria-current", "page");
        }
        pages.append(button);
      }
    }
  };

  const factRow = (term, description, provenance = false) => {
    const row = document.createElement("div");
    const dt = document.createElement("dt");
    const dd = document.createElement("dd");
    dt.textContent = term;
    dd.textContent = description;
    if (provenance) dd.className = "is-provenance";
    row.append(dt, dd);
    return row;
  };

  const chip = (label, muted = false) => {
    const element = document.createElement("span");
    element.className = muted ? "chip is-muted" : "chip";
    element.textContent = label;
    return element;
  };

  const birthdayText = (birthday) => {
    if (!birthday) return null;
    if (birthday.date) return birthday.date;
    if (birthday.month && birthday.day) {
      const month = String(birthday.month).padStart(2, "0");
      const day = String(birthday.day).padStart(2, "0");
      return `${month}-${day} · year not recorded`;
    }
    return null;
  };

  const renderPersonDetail = () => {
    const panel = byId("person-detail");
    const fields = byId("person-detail-fields");
    const meta = byId("person-detail-meta");
    const chips = byId("person-detail-chips");
    const person = state.people.find((item) => item.id === state.selectedPersonId);
    if (!person) {
      panel.hidden = true;
      fields.replaceChildren();
      meta.replaceChildren();
      chips.replaceChildren();
      return;
    }
    const emails = person.emails || [];
    const phones = person.phones || [];
    const birthday = birthdayText(person.birthday);
    const primaryEmail = emails[0]?.value;

    byId("person-detail-heading").textContent = person.display_name;
    byId("person-detail-avatar").textContent = initials(person.display_name);
    // Only facts the record actually holds are stated. Role, organisation and
    // time zone belong to later contracts and are not implied by empty slots.
    byId("person-detail-subtitle").textContent = primaryEmail
      ? `Preferred contact: ${primaryEmail}`
      : "No contact point recorded yet";

    chips.replaceChildren(
      chip(person.archived ? "Archived" : "Active", person.archived),
      chip(`Revision ${person.revision}`, true),
    );

    // The metadata grid carries only dated or counted facts, never a score.
    meta.replaceChildren(
      summaryRow("Important date", birthday || "None recorded"),
      summaryRow("Contact points", String(emails.length + phones.length)),
    );

    fields.replaceChildren(
      factRow("Display name", person.display_name),
      factRow(
        "Email",
        emails.length ? emails.map((email) => `${email.value} · ${email.label}`).join("\n") : "None recorded",
      ),
      factRow(
        "Phone",
        phones.length ? phones.map((phone) => `${phone.value} · ${phone.label}`).join("\n") : "None recorded",
      ),
      factRow("Birthday", birthday || "None recorded"),
      factRow("Record identifier", person.id, true),
    );
    panel.hidden = false;
  };

  const selectPerson = (personId) => {
    state.selectedPersonId = state.selectedPersonId === personId ? null : personId;
    renderPeople();
    renderPersonDetail();
    if (state.selectedPersonId) {
      byId("person-detail-heading").focus();
      status(`Showing the stored profile for the selected person. Records stay readable on disk.`);
    }
  };

  const updateControls = () => {
    const ready = Boolean(state.workspace);
    const busy = Boolean(nativeOperation.active) || nativeOperation.restartRequired;
    document.querySelectorAll("[data-native-operation]").forEach((control) => {
      const needsSession = control.dataset.nativeOperation === "session";
      control.disabled = busy || (needsSession && !ready);
    });
    ["person-name", "person-email", "people-search", "people-filter", "people-sort"].forEach((id) => {
      byId(id).disabled = busy || !ready;
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

  byId("people-list").addEventListener("click", (event) => {
    const trigger = event.target.closest?.("[data-person-id]");
    if (trigger) selectPerson(trigger.dataset.personId);
  });

  byId("people-pages").addEventListener("click", (event) => {
    const trigger = event.target.closest?.("[data-page]");
    if (!trigger) return;
    state.page = Number(trigger.dataset.page);
    renderPeople();
  });

  byId("people-search").addEventListener("input", (event) => {
    state.query = event.target.value;
    state.page = 1;
    renderPeople();
  });

  byId("people-filter").addEventListener("change", (event) => {
    state.filter = event.target.value;
    state.page = 1;
    renderPeople();
  });

  byId("people-sort").addEventListener("change", (event) => {
    state.sort = event.target.value;
    state.page = 1;
    renderPeople();
  });

  byId("column-options").addEventListener("change", (event) => {
    const input = event.target.closest?.("[data-column-id]");
    if (!input) return;
    const id = input.dataset.columnId;
    state.visibleColumns = input.checked
      ? [...new Set([...state.visibleColumns, id])]
      : state.visibleColumns.filter((column) => column !== id);
    renderPeople();
    status(`${input.checked ? "Showing" : "Hiding"} the ${id.replace(/-/g, " ")} column.`);
  });

  byId("close-person-detail").addEventListener("click", () => {
    const previous = state.selectedPersonId;
    state.selectedPersonId = null;
    renderPeople();
    renderPersonDetail();
    const restored = document.querySelector(`[data-person-id="${previous}"]`);
    if (restored) restored.focus();
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
        const person = await invokeValue("create_person", {
          request: {
            sessionId: operation.sessionId,
            displayName: byId("person-name").value,
            email: byId("person-email").value || null,
          },
        });
        if (!operationOwnsCurrentSession(operation)) return;
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

  const start = async () => {
    updateControls();
    renderWorkspace();
    renderColumnChooser();
    renderPeople();
    await withNativeOperation(null, "", async (operation) => {
      let buildStatement = "";
      try {
        const app = await invokeValue("app_status");
        if (!isCurrentOperation(operation)) return;
        byId("authority-label").textContent = `${app.product_state} · ${app.connection_state}`;
        buildStatement = `Liaison RM ${app.version}: ${app.release_evidence}.`;
        status(`${buildStatement} No workspace has been opened.`);
      } catch (error) {
        if (isCurrentOperation(operation)) {
          byId("authority-label").textContent = "Native bridge unavailable";
          status(errorText(error));
        }
        return;
      }
      let defaultPath = null;
      try {
        defaultPath = await invokeValue("default_workspace_path");
        if (!isCurrentOperation(operation)) return;
        byId("workspace-path").value = defaultPath;
      } catch (error) {
        if (isCurrentOperation(operation)) {
          status(`A default workspace folder was not selected: ${errorText(error)}`);
        }
        return;
      }
      // Resume the usual workspace without a click. Opening is read-then-lock
      // and creates nothing: when no workspace exists at the default path the
      // typed failure is expected, so it leaves the create form ready instead
      // of reporting an error the operator did not cause.
      try {
        const opened = await invokeValue("open_workspace", { path: defaultPath });
        if (!isCurrentOperation(operation)) return;
        if (await acceptWorkspace(opened, "Reopened workspace", operation)) {
          navigate("people");
        }
      } catch {
        // The build statement stays visible: an expected "nothing there yet"
        // must not silently drop the release-evidence disclosure.
        if (isCurrentOperation(operation)) {
          status(`${buildStatement} No workspace opened yet — review the folder below, then create one.`);
        }
      }
    });
  };

  start();
})();
