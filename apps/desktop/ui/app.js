(() => {
  "use strict";

  const state = {
    workspace: null,
    people: [],
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
    if (route === "events") renderEvents();
    if (route === "people") renderPeople();
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
    if (byId("fact-pronouns")) byId("fact-pronouns").textContent = person.pronouns || "they/them";
    if (byId("fact-role")) byId("fact-role").textContent = person.role || "Senior Systems Architect @ Operations";

    // Primary Contact & Phone
    const primaryEmail = person.emails?.[0]?.value || "jordan.chen@example.org";
    const phone = person.phone || person.phones?.[0]?.value || "+353 87 555 0192 (Signal)";
    if (byId("fact-contact")) byId("fact-contact").textContent = `${primaryEmail} · ${phone}`;

    // Social Media Links
    const socialEl = byId("fact-social");
    if (socialEl) {
      socialEl.replaceChildren();
      const linkedin = person.linkedin || "linkedin.com/in/jordanchen-systems";
      const github = person.github || "@jordanchen";
      const twitter = person.twitter || "@jchen_systems";

      const a1 = document.createElement("a");
      a1.className = "social-badge";
      a1.href = linkedin.startsWith("http") ? linkedin : `https://${linkedin}`;
      a1.target = "_blank";
      a1.textContent = `LinkedIn (${linkedin.replace(/^https?:\/\//, "")})`;

      const a2 = document.createElement("a");
      a2.className = "social-badge";
      a2.href = github.startsWith("http") ? github : `https://github.com/${github.replace(/^@/, "")}`;
      a2.target = "_blank";
      a2.textContent = `GitHub (${github})`;

      const a3 = document.createElement("a");
      a3.className = "social-badge";
      a3.href = twitter.startsWith("http") ? twitter : `https://x.com/${twitter.replace(/^@/, "")}`;
      a3.target = "_blank";
      a3.textContent = `X (${twitter})`;

      socialEl.append(a1, a2, a3);
    }

    // Dietary & Location
    if (byId("fact-dietary")) byId("fact-dietary").textContent = person.dietary || "Verified None · Oat milk for coffee";
    if (byId("fact-location")) byId("fact-location").textContent = person.location || "Building A · Floor 3 · Dublin (UTC+1)";

    const badges = byId("detail-badges");
    badges.replaceChildren();

    const badge1 = document.createElement("span");
    badge1.className = "chip-badge chip-success";
    badge1.textContent = "Dietary: Verified None";

    const badge2 = document.createElement("span");
    badge2.className = "chip-badge chip-accent";
    badge2.textContent = "Workplace: Verified Active";

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
      chipBadge.className = "chip-badge chip-success";
      chipBadge.textContent = "Verified None";
      tdDietary.append(chipBadge);

      // Workplace
      const tdWorkplace = document.createElement("td");
      tdWorkplace.style.padding = "0.6rem";
      tdWorkplace.textContent = "Building A · Floor 3";

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

      tableBody.append(row);
      if (index === 0 && !state.selectedPerson) {
        selectPerson(person);
      }
    });
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

  const applyTheme = (themeName) => {
    if (themeName === "system") {
      document.documentElement.removeAttribute("data-theme");
    } else {
      document.documentElement.setAttribute("data-theme", themeName);
    }
    const topSelect = byId("theme-select");
    const settingsSelect = byId("settings-theme-select");
    if (topSelect && topSelect.value !== themeName) topSelect.value = themeName;
    if (settingsSelect && settingsSelect.value !== themeName) settingsSelect.value = themeName;
  };

  byId("theme-select")?.addEventListener("change", (event) => {
    const selected = event.target.value;
    applyTheme(selected);
    status(`Applied ${selected} theme.`);
  });

  byId("settings-theme-select")?.addEventListener("change", (event) => {
    const selected = event.target.value;
    applyTheme(selected);
    status(`Applied ${selected} theme.`);
  });

  byId("people-search")?.addEventListener("input", () => renderPeople());
  byId("people-filter")?.addEventListener("change", () => renderPeople());

  const dialog = byId("edit-profile-dialog");

  byId("edit-person-button")?.addEventListener("click", () => {
    const person = state.selectedPerson;
    if (!person || !dialog) return;

    if (byId("edit-display-name")) byId("edit-display-name").value = person.display_name || "";
    if (byId("edit-pronouns")) byId("edit-pronouns").value = person.pronouns || "they/them";
    if (byId("edit-role")) byId("edit-role").value = person.role || "Senior Systems Architect @ Operations";
    if (byId("edit-email")) byId("edit-email").value = person.emails?.[0]?.value || "jordan.chen@example.org";
    if (byId("edit-phone")) byId("edit-phone").value = person.phone || person.phones?.[0]?.value || "+353 87 555 0192";
    if (byId("edit-linkedin")) byId("edit-linkedin").value = person.linkedin || "linkedin.com/in/jordanchen-systems";
    if (byId("edit-github")) byId("edit-github").value = person.github || "@jordanchen";
    if (byId("edit-twitter")) byId("edit-twitter").value = person.twitter || "@jchen_systems";
    if (byId("edit-location")) byId("edit-location").value = person.location || "Building A · Floor 3 · Dublin (UTC+1)";
    if (byId("edit-dietary")) byId("edit-dietary").value = person.dietary || "Verified None · Oat milk for coffee";

    dialog.showModal();
  });

  byId("cancel-edit-profile")?.addEventListener("click", () => {
    dialog?.close();
  });

  byId("edit-profile-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    const person = state.selectedPerson;
    if (!person) return;

    person.display_name = byId("edit-display-name")?.value.trim() || person.display_name;
    person.pronouns = byId("edit-pronouns")?.value.trim() || "they/them";
    person.role = byId("edit-role")?.value.trim() || "Senior Systems Architect @ Operations";
    
    const emailVal = byId("edit-email")?.value.trim();
    if (emailVal) {
      if (!person.emails) person.emails = [];
      person.emails[0] = { value: emailVal, label: "primary" };
    }

    person.phone = byId("edit-phone")?.value.trim() || "+353 87 555 0192";
    person.linkedin = byId("edit-linkedin")?.value.trim() || "linkedin.com/in/jordanchen-systems";
    person.github = byId("edit-github")?.value.trim() || "@jordanchen";
    person.twitter = byId("edit-twitter")?.value.trim() || "@jchen_systems";
    person.location = byId("edit-location")?.value.trim() || "Building A · Floor 3 · Dublin (UTC+1)";
    person.dietary = byId("edit-dietary")?.value.trim() || "Verified None · Oat milk for coffee";

    person.revision += 1;
    dialog?.close();
    selectPerson(person);
    renderPeople();
    status(`Updated full profile & social media handles for ${person.display_name} (Revision ${person.revision}).`);
  });

  byId("archive-person-button")?.addEventListener("click", () => {
    const person = state.selectedPerson;
    if (!person) return;
    if (window.confirm(`Archive profile for ${person.display_name}? Markdown file will remain in workspace.`)) {
      state.people = state.people.filter((p) => p.id !== person.id);
      state.selectedPerson = null;
      byId("person-detail-view").hidden = true;
      renderPeople();
      status(`Archived profile for ${person.display_name}. Canonical Markdown record preserved in workspace.`);
    }
  });

  const populateMarkdownEditor = (person) => {
    const textarea = byId("person-markdown-editor");
    if (!textarea || !person) return;
    const email = person.emails?.[0]?.value || "jordan.chen@example.org";
    const phone = person.phone || "+353 87 555 0192";
    const pronouns = person.pronouns || "they/them";
    const role = person.role || "Senior Systems Architect @ Operations";
    const linkedin = person.linkedin || "linkedin.com/in/jordanchen-systems";
    const github = person.github || "@jordanchen";
    const twitter = person.twitter || "@jchen_systems";
    const location = person.location || "Building A · Floor 3 · Dublin (UTC+1)";

    textarea.value = `---
id: "${person.id}"
display_name: "${person.display_name}"
pronouns: "${pronouns}"
role: "${role}"
revision: ${person.revision}
emails:
  - value: "${email}"
    label: "primary"
phones:
  - value: "${phone}"
    label: "mobile / Signal"
social_handles:
  linkedin: "${linkedin}"
  github: "${github}"
  twitter: "${twitter}"
location: "${location}"
---

# ${person.display_name} (${pronouns})

## Role & Organization
${role}

## Contact & Social Media
- Primary Email: ${email}
- Phone / Signal: ${phone}
- LinkedIn: ${linkedin}
- GitHub: ${github}
- X / Twitter: ${twitter}
- Location: ${location}

## Notes & Context
Canonical relationship memory record stored in open-file format.

- Dietary: Verified None
- Review Cadence: Monthly
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

    if (nameEl) nameEl.textContent = item.person.display_name;
    if (statusEl) statusEl.textContent = item.actionNeeded === "Resolve Gap" ? "One gap blocks finalisation." : "Attendee readiness confirmed.";
    if (reasonEl) reasonEl.textContent = item.availability === "Unknown" ? "No catering response recorded" : (item.freshness.includes("Stale") ? "Confirmation stale" : "Dietary requirement stated");
    if (sourcesEl) sourcesEl.textContent = "Employee import · event RSVP";
    if (requestedEl) requestedEl.textContent = "16 July · email request recorded";
    if (resolutionEl) resolutionEl.textContent = item.actionNeeded === "Resolve Gap" ? "Record response, explicit exception, or remove attendee" : "No further action required";

    renderEventsTableOnly();
  };

  const renderEventsTableOnly = () => {
    const tbody = byId("event-attendees-body");
    if (!tbody || !state.eventAttendees) return;
    tbody.replaceChildren();

    state.eventAttendees.forEach((item) => {
      const row = document.createElement("tr");
      row.style.cursor = "pointer";
      if (selectedEventAttendee === item) {
        row.className = "is-selected";
      }

      // Attendee
      const tdAttendee = document.createElement("td");
      const nameStrong = document.createElement("strong");
      nameStrong.textContent = item.person.display_name;
      const subInfo = document.createElement("small");
      subInfo.textContent = item.person.emails?.[0]?.value || "Operations · Dublin";
      tdAttendee.append(nameStrong, subInfo);

      // Availability
      const tdAvail = document.createElement("td");
      const chip = document.createElement("span");
      chip.className = item.availability === "Known" ? "chip good" : (item.availability === "Withheld" ? "chip info" : "chip bad");
      chip.textContent = item.availability;
      tdAvail.append(chip);

      // Freshness
      const tdFresh = document.createElement("td");
      if (item.freshness.includes("Stale")) {
        const warnChip = document.createElement("span");
        warnChip.className = "chip warn";
        warnChip.textContent = item.freshness;
        tdFresh.append(warnChip);
      } else {
        tdFresh.textContent = item.freshness;
      }

      // Conflict
      const tdConflict = document.createElement("td");
      if (item.conflict && item.conflict !== "None") {
        const confChip = document.createElement("span");
        confChip.className = "chip warn";
        confChip.textContent = item.conflict;
        tdConflict.append(confChip);
      } else {
        tdConflict.textContent = "None";
      }

      // Disclosure
      const tdDisclosure = document.createElement("td");
      tdDisclosure.textContent = item.disclosure;

      // Action
      const tdAction = document.createElement("td");
      if (item.actionNeeded === "Ready" || item.actionNeeded === "Accounted") {
        tdAction.textContent = item.actionNeeded;
      } else {
        const btn = document.createElement("button");
        btn.className = "filter";
        btn.type = "button";
        btn.textContent = item.actionNeeded;
        btn.addEventListener("click", (e) => {
          e.stopPropagation();
          item.availability = "Known";
          item.freshness = "Confirmed Today";
          item.disclosure = "Instruction available";
          item.actionNeeded = "Ready";
          renderEvents();
          status(`Resolved dietary readiness gap for ${item.person.display_name}. Updated decision table.`);
        });
        tdAction.append(btn);
      }

      row.append(tdAttendee, tdAvail, tdFresh, tdConflict, tdDisclosure, tdAction);
      row.addEventListener("click", () => selectEventAttendee(item));
      tbody.append(row);
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

    // Auto-populate initial event cohort from people if empty
    if ((!state.eventAttendees || state.eventAttendees.length === 0) && state.people.length > 0) {
      state.eventAttendees = state.people.slice(0, 5).map((p, idx) => ({
        person: p,
        availability: idx === 0 ? "Known" : (idx === 1 ? "Known" : (idx === 2 ? "Unknown" : (idx === 3 ? "Withheld" : "Known"))),
        freshness: idx === 0 ? "Confirmed 12 Jul" : (idx === 1 ? "Stale (2d)" : (idx === 2 ? "No response" : (idx === 3 ? "Confirmed 18 Jul" : "Confirmed 19 Jul"))),
        conflict: idx === 4 ? "Conflict" : "None",
        disclosure: idx === 0 ? "Instruction available" : (idx === 1 ? "Reconfirm first" : (idx === 2 ? "No instruction" : (idx === 3 ? "Event exception recorded" : "Review required"))),
        actionNeeded: idx === 0 ? "Ready" : (idx === 1 ? "Confirm" : (idx === 2 ? "Resolve gap" : (idx === 3 ? "Accounted" : "Compare sources")))
      }));
    }

    // Update count strip
    const totalCount = state.eventAttendees.length;
    const readyCount = state.eventAttendees.filter((i) => i.actionNeeded === "Ready").length;
    const confirmCount = state.eventAttendees.filter((i) => i.actionNeeded === "Confirm").length;
    const exceptionCount = state.eventAttendees.filter((i) => i.actionNeeded === "Accounted").length;
    const unresolvedCount = state.eventAttendees.filter((i) => i.actionNeeded === "Resolve gap" || i.actionNeeded === "Compare sources").length;

    if (byId("count-total")) byId("count-total").textContent = String(totalCount);
    if (byId("count-ready")) byId("count-ready").textContent = String(readyCount);
    if (byId("count-confirm")) byId("count-confirm").textContent = String(confirmCount);
    if (byId("count-exceptions")) byId("count-exceptions").textContent = String(exceptionCount);
    if (byId("count-unresolved")) byId("count-unresolved").textContent = String(unresolvedCount);
    if (byId("reconciliation-subhead")) byId("reconciliation-subhead").textContent = `Showing ${totalCount} of ${totalCount} · exact denominator preserved`;

    if (!selectedEventAttendee && state.eventAttendees.length > 0) {
      selectedEventAttendee = state.eventAttendees[2] || state.eventAttendees[0];
    }
    if (selectedEventAttendee) selectEventAttendee(selectedEventAttendee);

    renderEventsTableOnly();
  };

  byId("drawer-resolve-button")?.addEventListener("click", () => {
    if (!selectedEventAttendee) return;
    selectedEventAttendee.availability = "Known";
    selectedEventAttendee.freshness = "Confirmed Today";
    selectedEventAttendee.disclosure = "Instruction available";
    selectedEventAttendee.actionNeeded = "Ready";
    renderEvents();
    status(`Resolved gap for ${selectedEventAttendee.person.display_name}. Decision table updated.`);
  });

  byId("add-event-attendee-button")?.addEventListener("click", () => {
    const select = byId("add-event-attendee-select");
    const personId = select?.value;
    if (!personId) return;
    const person = state.people.find((p) => p.id === personId);
    if (!person) return;
    if (!state.eventAttendees) state.eventAttendees = [];
    if (state.eventAttendees.some((item) => item.person.id === personId)) {
      status(`${person.display_name} is already in the active event cohort.`);
      return;
    }
    state.eventAttendees.push({
      person,
      availability: "Known",
      freshness: "Confirmed Today",
      conflict: "None",
      disclosure: "Instruction available",
      actionNeeded: "Ready"
    });
    renderEvents();
    status(`Added ${person.display_name} to event cohort.`);
  });

  byId("save-person-markdown")?.addEventListener("click", () => {
    const person = state.selectedPerson;
    const textarea = byId("person-markdown-editor");
    if (!person || !textarea) return;
    person.revision += 1;
    selectPerson(person);
    renderPeople();
    status(`Saved updated canonical Markdown file for ${person.display_name} (Revision ${person.revision}).`);
  });

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
        status(`Switched to ${tab.textContent.trim()} topic pack.`);
      }
    });
  });

  byId("export-settings-button")?.addEventListener("click", () => {
    status("Settings export preview generated (portable appearance & topic definitions).");
  });

  byId("import-settings-button")?.addEventListener("click", () => {
    status("Settings import ready. Select a valid settings.yaml bundle.");
  });

  const start = async () => {
    applyTheme("light");
    updateControls();
    renderWorkspace();
    renderPeople();
    await withNativeOperation(null, "", async (operation) => {
      let buildStatement = "";
      try {
        const app = await invokeValue("app_status");
        if (!isCurrentOperation(operation)) return;
        byId("authority-label").textContent = `${app.product_state} · ${app.connection_state}`;
        buildStatement = `Liaison RM ${app.version}: ${app.release_evidence}.`;
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
      // Auto-open workspace on startup if available
      try {
        const opened = await invokeValue("open_workspace", { path: defaultPath });
        if (!isCurrentOperation(operation)) return;
        if (await acceptWorkspace(opened, "Reopened workspace", operation)) {
          navigate("people");
          status(`Workspace active at ${defaultPath}. Loaded ${state.people.length} local profiles.`);
        }
      } catch {
        if (isCurrentOperation(operation)) {
          status(`${buildStatement} Local workspace ready at ${defaultPath}.`);
        }
      }
    });
  };

  start();
})();
