import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";
import { Button } from "./Button";
import { Field } from "./Field";
import { RouteNavigation } from "./RouteNavigation";
import { StatusBanner } from "./StatusBanner";

const routes = [
  { id: "overview" as const, label: "Overview" },
  { id: "directory" as const, label: "Directory" },
  { id: "events" as const, label: "Events" },
] as const;

describe("Button", () => {
  it("exposes an honest busy state and disables repeated activation", () => {
    const onClick = vi.fn();
    render(
      <Button busy busyLabel="Saving person…" onClick={onClick} tone="primary">
        Save person
      </Button>,
    );

    const button = screen.getByRole("button", { name: "Saving person…" });
    expect(button.getAttribute("aria-busy")).toBe("true");
    expect((button as HTMLButtonElement).disabled).toBe(true);
    fireEvent.click(button);
    expect(onClick).not.toHaveBeenCalled();
  });
});

describe("Field", () => {
  it("associates the visible label, hint, and error with the input", () => {
    render(
      <Field
        error="Enter a display name."
        hint="Use the name this person uses."
        label="Display name"
        name="displayName"
      />,
    );

    const input = screen.getByLabelText("Display name");
    const describedBy = input.getAttribute("aria-describedby") ?? "";
    expect(input.getAttribute("aria-invalid")).toBe("true");
    expect(describedBy).toContain("hint");
    expect(describedBy).toContain("error");
    expect(screen.getByText("Use the name this person uses.").id).not.toBe("");
    expect(screen.getByText("Enter a display name.").id).not.toBe("");
  });
});

describe("RouteNavigation", () => {
  it("marks the current route textually and emits typed navigation", () => {
    const onNavigate = vi.fn();
    render(
      <RouteNavigation current="overview" items={routes} onNavigate={onNavigate} />,
    );

    expect(
      screen.getByRole("button", { name: "Overview" }).getAttribute("aria-current"),
    ).toBe("page");
    fireEvent.click(screen.getByRole("button", { name: "Directory" }));
    expect(onNavigate).toHaveBeenCalledWith("directory");
  });
});

describe("StatusBanner", () => {
  it("uses assertive semantics only for a blocking conflict", () => {
    render(
      <StatusBanner
        phase="conflict"
        recovery="Review the external edit before retrying."
        title="This record changed outside Liaison"
      >
        Liaison did not overwrite the newer file.
      </StatusBanner>,
    );

    const alert = screen.getByRole("alert");
    expect(alert.getAttribute("aria-atomic")).toBe("true");
    expect(screen.getByText("Review the external edit before retrying.")).not.toBeNull();
  });

  it("creates unique labelled regions for concurrent operation status", () => {
    render(
      <>
        <StatusBanner phase="publishing" title="Saving Person A">
          Publishing one canonical record.
        </StatusBanner>
        <StatusBanner phase="publishing" title="Saving Person B">
          Publishing one canonical record.
        </StatusBanner>
      </>,
    );

    const statuses = screen.getAllByRole("status");
    const firstLabel = statuses[0]?.getAttribute("aria-labelledby");
    const secondLabel = statuses[1]?.getAttribute("aria-labelledby");
    expect(firstLabel).not.toBeNull();
    expect(secondLabel).not.toBeNull();
    expect(firstLabel).not.toBe(secondLabel);
  });
});
