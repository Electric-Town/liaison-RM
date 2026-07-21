import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import { App } from "./App";
import { createTauriTransport, unavailableTransport, type NativeInvoke } from "./transport";

interface TauriInternals {
  readonly invoke?: NativeInvoke;
}

declare global {
  interface Window {
    __TAURI_INTERNALS__?: TauriInternals;
  }
}

const host = document.getElementById("root");
if (host === null) {
  throw new Error("Liaison React root element is missing");
}

const invoke = window.__TAURI_INTERNALS__?.invoke;
const transport = invoke === undefined ? unavailableTransport() : createTauriTransport(invoke);

createRoot(host).render(
  <StrictMode>
    <App transport={transport} />
  </StrictMode>,
);
