import type { OperationPhase, OperationStatus } from "./application-contract";

export type OperationTone = "neutral" | "progress" | "success" | "warning" | "danger";
export type OperationAction = "run-health" | "recover";

export interface OperationPresentation {
  readonly phaseLabel: string;
  readonly tone: OperationTone;
  readonly guidance: string;
  readonly actions: readonly OperationAction[];
  readonly canInterrupt: boolean;
}

const labels: Record<OperationPhase, string> = {
  idle: "Ready",
  validating: "Validating",
  staging: "Preparing local changes",
  "commit-decided": "Commit decision recorded",
  publishing: "Publishing canonical records",
  recovering: "Recovering committed work",
  complete: "Complete",
  conflict: "Review required",
  failed: "Did not finish",
};

export function presentOperation(
  status: OperationStatus,
  hasWorkspace: boolean,
): OperationPresentation {
  switch (status.phase) {
    case "idle":
      return {
        phaseLabel: labels.idle,
        tone: "neutral",
        guidance: "Choose a task when you are ready.",
        actions: [],
        canInterrupt: true,
      };
    case "validating":
      return {
        phaseLabel: labels.validating,
        tone: "progress",
        guidance: "Liaison is checking the request before any commit decision.",
        actions: [],
        canInterrupt: status.interruptible,
      };
    case "staging":
      return {
        phaseLabel: labels.staging,
        tone: "progress",
        guidance: "Changes are staged locally. No canonical target has been published yet.",
        actions: [],
        canInterrupt: status.interruptible,
      };
    case "commit-decided":
      return {
        phaseLabel: labels["commit-decided"],
        tone: "warning",
        guidance: "Keep Liaison open. The durable commit decision exists and the operation must roll forward.",
        actions: [],
        canInterrupt: false,
      };
    case "publishing":
      return {
        phaseLabel: labels.publishing,
        tone: "progress",
        guidance: "Keep Liaison open while committed targets are published in stable order.",
        actions: [],
        canInterrupt: false,
      };
    case "recovering":
      return {
        phaseLabel: labels.recovering,
        tone: "warning",
        guidance: "Recovery is rolling committed work forward. It will stop rather than overwrite an external edit.",
        actions: [],
        canInterrupt: false,
      };
    case "complete":
      return {
        phaseLabel: labels.complete,
        tone: "success",
        guidance: status.receiptId === undefined
          ? "The operation finished."
          : "The operation finished and a receipt is available below.",
        actions: [],
        canInterrupt: true,
      };
    case "conflict":
      return {
        phaseLabel: labels.conflict,
        tone: "danger",
        guidance: "Liaison preserved the current files. Review the finding before choosing a recovery action.",
        actions: hasWorkspace ? ["run-health", "recover"] : [],
        canInterrupt: true,
      };
    case "failed": {
      const recoverable = hasWorkspace && status.error?.retryable === true;
      return {
        phaseLabel: labels.failed,
        tone: "danger",
        guidance: status.error?.recovery ?? "Review the error before trying another operation.",
        actions: recoverable ? ["run-health", "recover"] : hasWorkspace ? ["run-health"] : [],
        canInterrupt: true,
      };
    }
  }
}
