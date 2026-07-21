import { useState, type FormEvent } from "react";
import type { WorkspaceProfile } from "../application-contract";
import { Button, Field, Surface } from "../components";
import type { DesktopController } from "../useDesktopController";

export interface SettingsRouteProps {
  readonly controller: DesktopController;
}

export function SettingsRoute({ controller }: SettingsRouteProps) {
  const [path, setPath] = useState("");
  const [displayName, setDisplayName] = useState("My Liaison workspace");
  const [profile, setProfile] = useState<WorkspaceProfile>("airgap");

  async function open(event: FormEvent<HTMLFormElement>): Promise<void> {
    event.preventDefault();
    if (path.trim() === "") {
      return;
    }
    await controller.openWorkspace(path.trim());
  }

  async function create(): Promise<void> {
    if (path.trim() === "" || displayName.trim() === "") {
      return;
    }
    await controller.initialiseWorkspace({
      path: path.trim(),
      displayName: displayName.trim(),
      profile,
    });
  }

  return (
    <Surface aria-labelledby="route-heading" emphasis="raised">
      <p className="section-label">Local workspace</p>
      <h2 id="route-heading" tabIndex={-1}>
        Settings
      </h2>
      <p>
        Select an absolute local folder. Liaison never creates or opens a workspace merely because text was entered.
      </p>

      <form className="settings-form" onSubmit={(event) => void open(event)}>
        <Field
          autoComplete="off"
          disabled={controller.busy}
          hint="Use an empty folder to create a workspace, or the folder containing an existing liaison.yaml manifest to open one."
          label="Workspace folder"
          name="workspacePath"
          onChange={(event) => setPath(event.currentTarget.value)}
          required
          value={path}
        />
        <Field
          autoComplete="off"
          disabled={controller.busy}
          hint="Stored in the local workspace manifest when a new workspace is created."
          label="Workspace name"
          name="workspaceName"
          onChange={(event) => setDisplayName(event.currentTarget.value)}
          required
          value={displayName}
        />
        <div className="lrm-field">
          <label className="lrm-field__label" htmlFor="workspace-profile">
            Workspace profile
          </label>
          <p className="lrm-field__hint" id="workspace-profile-hint">
            Airgap compiles out network authority. Connected-local permits separately granted local connections.
          </p>
          <select
            aria-describedby="workspace-profile-hint"
            className="lrm-field__input"
            disabled={controller.busy}
            id="workspace-profile"
            onChange={(event) => setProfile(event.currentTarget.value as WorkspaceProfile)}
            value={profile}
          >
            <option value="airgap">Airgap</option>
            <option value="connected-local">Connected-local</option>
          </select>
        </div>

        <div className="action-row">
          <Button busy={controller.busy} busyLabel="Opening…" type="submit">
            Open existing workspace
          </Button>
          <Button
            busy={controller.busy}
            busyLabel="Creating…"
            onClick={() => void create()}
            tone="primary"
          >
            Create new workspace
          </Button>
          {controller.workspace === null ? null : (
            <Button
              busy={controller.busy}
              busyLabel="Closing…"
              onClick={() => void controller.closeWorkspace()}
              tone="quiet"
            >
              Close workspace
            </Button>
          )}
        </div>
      </form>

      {controller.workspace === null ? (
        <p className="bounded-note">No workspace is open.</p>
      ) : (
        <dl className="record-summary">
          <div>
            <dt>Name</dt>
            <dd>{controller.workspace.displayName}</dd>
          </div>
          <div>
            <dt>Identifier</dt>
            <dd><code>{controller.workspace.workspaceId}</code></dd>
          </div>
          <div>
            <dt>Schema</dt>
            <dd>{controller.workspace.schemaVersion}</dd>
          </div>
          <div>
            <dt>Authority</dt>
            <dd>{controller.workspace.writeAuthority.replace("-", " ")}</dd>
          </div>
        </dl>
      )}
    </Surface>
  );
}
