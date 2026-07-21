import { useMemo, useState } from "react";
import {
  B0_ROUTES,
  type B0Route,
  type CommandTransport,
} from "./application-contract";
import "./styles.css";

interface AppProps {
  readonly transport: CommandTransport;
}

const routeLabels: Record<B0Route, string> = {
  overview: "Overview",
  directory: "Directory",
  events: "Events",
  health: "Health",
  settings: "Settings",
};

export function App({ transport }: AppProps): JSX.Element {
  const [route, setRoute] = useState<B0Route>("overview");
  const [status, setStatus] = useState("Typed interface foundation loaded.");
  const routeHeading = useMemo(() => routeLabels[route], [route]);

  async function verifyTransport(): Promise<void> {
    setStatus("Checking the native application contract…");
    const result = await transport.invoke("workspace_status", {});
    if (result.ok) {
      setStatus(`Workspace contract available for ${result.value.displayName}.`);
    } else {
      setStatus(`${result.error.message} ${result.error.recovery}`);
    }
  }

  return (
    <div className="app-shell">
      <a className="skip-link" href="#main-content">
        Skip to main content
      </a>
      <header className="app-header">
        <div>
          <p className="eyebrow">Liaison RM · P04 foundation</p>
          <h1>Editorial Ledger</h1>
        </div>
        <p className="local-authority">Local workspace authority only</p>
      </header>

      <div className="app-layout">
        <nav aria-label="Primary" className="primary-navigation">
          {B0_ROUTES.map((candidate) => (
            <button
              className="nav-item"
              aria-current={candidate === route ? "page" : undefined}
              key={candidate}
              onClick={() => setRoute(candidate)}
              type="button"
            >
              {routeLabels[candidate]}
            </button>
          ))}
        </nav>

        <main id="main-content" tabIndex={-1}>
          <section aria-labelledby="route-heading" className="work-surface">
            <p className="section-label">Typed route seam</p>
            <h2 id="route-heading">{routeHeading}</h2>
            <p>
              This parallel foundation proves the versioned React and application-contract
              boundary. The legacy shell remains the active interface until route parity,
              accessibility, localization, and installed-platform evidence pass.
            </p>
            <dl className="contract-summary">
              <div>
                <dt>Canonical data</dt>
                <dd>Rust application services and the local Workspace</dd>
              </div>
              <div>
                <dt>Browser storage</dt>
                <dd>Not authoritative and not used</dd>
              </div>
              <div>
                <dt>Network</dt>
                <dd>No runtime request authority</dd>
              </div>
            </dl>
            <button className="primary-action" onClick={() => void verifyTransport()} type="button">
              Verify native contract
            </button>
            <p aria-live="polite" className="status-line">
              {status}
            </p>
          </section>
        </main>
      </div>
    </div>
  );
}
