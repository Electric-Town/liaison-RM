import type { DesktopController } from "../useDesktopController";
import { Surface } from "../components";

export interface OverviewRouteProps {
  readonly controller: DesktopController;
}

export function OverviewRoute({ controller }: OverviewRouteProps) {
  const { workspace, people, health } = controller;
  const errors = health?.findings.filter((finding) => finding.severity === "error").length ?? 0;

  return (
    <Surface aria-labelledby="route-heading" emphasis="raised">
      <p className="section-label">Workspace overview</p>
      <h2 id="route-heading" tabIndex={-1}>
        Overview
      </h2>
      {workspace === null ? (
        <p>Open or create a local workspace in Settings before adding relationship records.</p>
      ) : (
        <>
          <p>
            <strong>{workspace.displayName}</strong> is open with {workspace.writeAuthority.replace("-", " ")} authority.
          </p>
          <dl className="metric-grid">
            <div>
              <dt>People</dt>
              <dd>{people.length}</dd>
            </div>
            <div>
              <dt>Workspace profile</dt>
              <dd>{workspace.profile === "airgap" ? "Airgap" : "Connected-local"}</dd>
            </div>
            <div>
              <dt>Health errors</dt>
              <dd>{health === null ? "Not checked" : errors}</dd>
            </div>
          </dl>
          <p className="bounded-note">
            Overview reports stored facts and current operation state. It does not calculate relationship strength, employee value, or a universal attention score.
          </p>
        </>
      )}
    </Surface>
  );
}
