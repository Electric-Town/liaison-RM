import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { App } from "./App";
import {
  APPLICATION_RESPONSE,
  B0_ROUTES,
  EVENT_STAGES,
  OPERATION_PHASES,
  assertApplicationResponse,
  type CommandTransport,
} from "./application-contract";

function transportWithWorkspace(): CommandTransport {
  return {
    invoke: vi.fn(async () => ({
      schema: APPLICATION_RESPONSE,
      ok: true,
      value: {
        workspaceId: "018f0000-0000-7000-8000-000000000001",
        schemaVersion: 1,
        displayName: "Example workspace",
        profile: "airgap",
        writeAuthority: "held",
      },
    })) as CommandTransport["invoke"],
  };
}

describe("application contract", () => {
  it("pins the stable B0 routes and Events stages", () => {
    expect(B0_ROUTES).toEqual([
      "overview",
      "directory",
      "events",
      "health",
      "settings",
    ]);
    expect(EVENT_STAGES).toEqual([
      "details",
      "cohort",
      "attendees",
      "readiness",
      "brief",
    ]);
  });

  it("keeps recovery and conflict as explicit operation phases", () => {
    expect(OPERATION_PHASES).toContain("recovering");
    expect(OPERATION_PHASES).toContain("conflict");
    expect(OPERATION_PHASES).toContain("commit-decided");
  });

  it("rejects an unversioned application response", () => {
    expect(() => assertApplicationResponse({ ok: true, value: null })).toThrow(
      "liaison/application-response@1",
    );
  });
});

describe("parallel React shell", () => {
  it("exposes stable route navigation without browser authority", () => {
    render(<App transport={transportWithWorkspace()} />);
    expect(screen.getByRole("heading", { name: "Overview" })).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: "Directory" }));
    expect(screen.getByRole("heading", { name: "Directory" })).toBeInTheDocument();
    expect(screen.getByText("Not authoritative and not used")).toBeInTheDocument();
  });

  it("reports the typed native contract result through a polite status", async () => {
    render(<App transport={transportWithWorkspace()} />);
    fireEvent.click(screen.getByRole("button", { name: "Verify native contract" }));
    expect(await screen.findByText("Workspace contract available for Example workspace.")).toBeInTheDocument();
  });
});
