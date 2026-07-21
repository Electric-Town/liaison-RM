import { Button, Surface } from "../components";
import type { DesktopController } from "../useDesktopController";

export interface HealthRouteProps {
  readonly controller: DesktopController;
}

export function HealthRoute({ controller }: HealthRouteProps) {
  const findings = controller.health?.findings ?? [];

  return (
    <Surface aria-labelledby="route-heading" emphasis="raised">
      <p className="section-label">Non-destructive inspection</p>
      <h2 id="route-heading" tabIndex={-1}>
        Health
      </h2>
      <p>
        Health reads the selected workspace and reports typed findings. It does not delete, merge, normalize, or repair canonical files automatically.
      </p>

      <div className="action-row">
        <Button
          busy={controller.busy}
          busyLabel="Checking…"
          disabled={controller.workspace === null}
          onClick={() => void controller.validateWorkspace()}
          tone="primary"
        >
          Check workspace health
        </Button>
        <Button
          busy={controller.busy}
          busyLabel="Recovering…"
          disabled={controller.workspace === null}
          onClick={() => void controller.recoverWorkspace()}
        >
          Recover interrupted operations
        </Button>
      </div>

      {controller.workspace === null ? (
        <p>Open a workspace in Settings before running Health.</p>
      ) : controller.health === null ? (
        <p>No Health report has been run in this session.</p>
      ) : findings.length === 0 ? (
        <p className="health-clear">No findings were reported.</p>
      ) : (
        <ol className="finding-list">
          {findings.map((finding, index) => (
            <li className={`finding finding--${finding.severity}`} key={`${finding.code}-${finding.path}-${index}`}>
              <div className="finding__heading">
                <strong>{finding.message}</strong>
                <span>{finding.severity}</span>
              </div>
              <dl>
                <div>
                  <dt>Code</dt>
                  <dd><code>{finding.code}</code></dd>
                </div>
                <div>
                  <dt>Path</dt>
                  <dd><code>{finding.path}</code></dd>
                </div>
              </dl>
              <p><strong>Recovery:</strong> {finding.recovery}</p>
            </li>
          ))}
        </ol>
      )}
    </Surface>
  );
}
