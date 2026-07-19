(() => {
  "use strict";

  const state = {
    workspace: null,
    people: [],
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

  const withBusy = async (button, busyLabel, work) => {
    const original = button.textContent;
    button.disabled = true;
    button.textContent = busyLabel;
    try {
      return await work();
    } finally {
      button.textContent = original;
      button.disabled = false;
      updateControls();
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

  const renderPeople = () => {
    const list = byId("people-list");
    list.replaceChildren();
    byId("people-count").textContent = String(state.people.length);
    if (state.people.length === 0) {
      const empty = document.createElement("li");
      empty.className = "empty-state";
      empty.textContent = state.workspace ? "No people yet. Add the first profile." : "No people loaded.";
      list.append(empty);
      return;
    }
    state.people.forEach((person) => {
      const row = document.createElement("li");
      row.className = "person-row";
      const avatar = document.createElement("span");
      avatar.className = "person-avatar";
      avatar.setAttribute("aria-hidden", "true");
      avatar.textContent = initials(person.display_name);
      const details = document.createElement("span");
      const name = document.createElement("strong");
      name.textContent = person.display_name;
      const email = document.createElement("small");
      email.textContent = person.emails?.[0]?.value || "No email recorded";
      details.append(name, email);
      const revision = document.createElement("span");
      revision.className = "revision";
      revision.textContent = `Revision ${person.revision}`;
      row.append(avatar, details, revision);
      list.append(row);
    });
  };

  const updateControls = () => {
    const ready = Boolean(state.workspace);
    ["person-name", "person-email", "create-person", "run-validation", "refresh-people"].forEach((id) => {
      byId(id).disabled = !ready;
    });
    byId("people-workspace-warning").hidden = ready;
  };

  const refreshPeople = async () => {
    if (!state.workspace) return;
    state.people = await invokeValue("list_people", {
      request: { sessionId: state.workspace.session_id },
    });
    renderPeople();
    renderWorkspace();
  };

  const acceptWorkspace = async (opened, action) => {
    const previous = state.workspace;
    if (previous && previous.session_id !== opened.workspace.session_id) {
      try {
        await invokeValue("close_workspace", {
          request: { sessionId: previous.session_id },
        });
      } catch (error) {
        try {
          await invokeValue("close_workspace", {
            request: { sessionId: opened.workspace.session_id },
          });
        } catch {
          // Closing the replacement is best-effort; the original close error
          // is the actionable failure and the original selection stays put.
        }
        status(`Workspace switch did not complete: ${errorText(error)}`);
        return false;
      }
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

  byId("use-default-path").addEventListener("click", async () => {
    try {
      byId("workspace-path").value = await invokeValue("default_workspace_path");
      status("Suggested a local Documents folder. Review it before creating the workspace.");
    } catch (error) {
      status(errorText(error));
    }
  });

  byId("workspace-form").addEventListener("submit", async (event) => {
    event.preventDefault();
    const button = event.submitter;
    await withBusy(button, "Creating…", async () => {
      try {
        const opened = await invokeValue("initialise_workspace", {
          request: {
            path: byId("workspace-path").value,
            name: byId("workspace-name").value,
            profile: byId("workspace-profile").value,
          },
        });
        if (await acceptWorkspace(opened, "Created local workspace")) {
          navigate("people");
        }
      } catch (error) {
        status(`Workspace setup did not complete: ${errorText(error)}`);
      }
    });
  });

  byId("open-workspace").addEventListener("click", async (event) => {
    await withBusy(event.currentTarget, "Opening…", async () => {
      try {
        const opened = await invokeValue("open_workspace", { path: byId("workspace-path").value });
        await acceptWorkspace(opened, "Opened workspace");
      } catch (error) {
        status(`Workspace was not opened: ${errorText(error)}`);
      }
    });
  });

  byId("inspect-workspace-health").addEventListener("click", async (event) => {
    await withBusy(event.currentTarget, "Inspecting…", async () => {
      try {
        const inspectedPath = byId("workspace-path").value;
        const report = await invokeValue("inspect_workspace_health", {
          path: inspectedPath,
        });
        showValidation(report, inspectedPath, "Read-only folder");
        navigate("health");
        status(report.valid
          ? "Read-only Workspace Health passed without acquiring writer authority."
          : `Read-only Workspace Health reported ${report.findings.length} finding${report.findings.length === 1 ? "" : "s"} without changing files.`);
      } catch (error) {
        status(`Read-only Workspace Health did not complete: ${errorText(error)}`);
      }
    });
  });

  byId("person-form").addEventListener("submit", async (event) => {
    event.preventDefault();
    if (!state.workspace) return;
    const form = event.currentTarget;
    const button = event.submitter || byId("create-person");
    await withBusy(button, "Saving…", async () => {
      try {
        const person = await invokeValue("create_person", {
          request: {
            sessionId: state.workspace.session_id,
            displayName: byId("person-name").value,
            email: byId("person-email").value || null,
          },
        });
        state.people.push(person);
        state.people.sort((left, right) => left.display_name.localeCompare(right.display_name));
        form.reset();
        renderPeople();
        renderWorkspace();
        status(`Saved ${person.display_name} as a local Markdown profile.`);
        byId("person-name").focus();
      } catch (error) {
        status(`Person was not saved: ${errorText(error)}`);
      }
    });
  });

  byId("refresh-people").addEventListener("click", async (event) => {
    await withBusy(event.currentTarget, "Refreshing…", async () => {
      try {
        await refreshPeople();
        status(`Refreshed ${state.people.length} local profile${state.people.length === 1 ? "" : "s"}.`);
      } catch (error) {
        status(`People were not refreshed: ${errorText(error)}`);
      }
    });
  });

  byId("run-validation").addEventListener("click", async (event) => {
    if (!state.workspace) return;
    await withBusy(event.currentTarget, "Validating…", async () => {
      try {
        const report = await invokeValue("validate_workspace", {
          request: { sessionId: state.workspace.session_id },
        });
        showValidation(report, state.workspace.path, "Active workspace");
        status(report.valid ? "Workspace validation passed." : "Workspace validation reported findings.");
      } catch (error) {
        status(`Validation did not complete: ${errorText(error)}`);
      }
    });
  });

  const start = async () => {
    updateControls();
    renderWorkspace();
    renderPeople();
    try {
      const app = await invokeValue("app_status");
      byId("authority-label").textContent = `${app.product_state} · ${app.connection_state}`;
      status(`Liaison RM ${app.version}: ${app.release_evidence}. No workspace has been opened.`);
    } catch (error) {
      byId("authority-label").textContent = "Native bridge unavailable";
      status(errorText(error));
      return;
    }
    try {
      byId("workspace-path").value = await invokeValue("default_workspace_path");
    } catch (error) {
      status(`A default workspace folder was not selected: ${errorText(error)}`);
    }
  };

  start();
})();
