export const APPLICATION_CONTRACT = "liaison/application-contract@1" as const;
export const APPLICATION_RESPONSE = "liaison/application-response@1" as const;

export const B0_ROUTES = [
  "overview",
  "directory",
  "events",
  "health",
  "settings",
] as const;

export type B0Route = (typeof B0_ROUTES)[number];

export const EVENT_STAGES = [
  "details",
  "cohort",
  "attendees",
  "readiness",
  "brief",
] as const;

export type EventStage = (typeof EVENT_STAGES)[number];

export const OPERATION_PHASES = [
  "idle",
  "validating",
  "staging",
  "commit-decided",
  "publishing",
  "recovering",
  "complete",
  "conflict",
  "failed",
] as const;

export type OperationPhase = (typeof OPERATION_PHASES)[number];

export type ErrorCategory =
  | "validation"
  | "conflict"
  | "permission"
  | "not-found"
  | "unsupported"
  | "storage"
  | "internal";

export interface CommandError {
  readonly code: string;
  readonly category: ErrorCategory;
  readonly message: string;
  readonly recovery: string;
  readonly retryable: boolean;
}

export type CommandResult<T> =
  | {
      readonly schema: typeof APPLICATION_RESPONSE;
      readonly ok: true;
      readonly value: T;
    }
  | {
      readonly schema: typeof APPLICATION_RESPONSE;
      readonly ok: false;
      readonly error: CommandError;
    };

export interface OperationStatus {
  readonly operationId: string;
  readonly phase: OperationPhase;
  readonly headline: string;
  readonly detail: string;
  readonly interruptible: boolean;
  readonly receiptId?: string;
  readonly error?: CommandError;
}

export interface WorkspaceSummary {
  readonly workspaceId: string;
  readonly schemaVersion: number;
  readonly displayName: string;
  readonly profile: "airgap" | "connected-local";
  readonly writeAuthority: "held" | "read-only" | "unavailable";
}

export interface PersonSummary {
  readonly personId: string;
  readonly revision: number;
  readonly displayName: string;
  readonly primaryEmail?: string;
  readonly primaryPhone?: string;
  readonly birthday?: string;
  readonly archived: boolean;
}

export interface HealthFinding {
  readonly code: string;
  readonly severity: "information" | "warning" | "error";
  readonly path: string;
  readonly message: string;
  readonly recovery: string;
}

export interface HealthReport {
  readonly workspaceId?: string;
  readonly findings: readonly HealthFinding[];
}

export const COMMANDS = {
  openWorkspace: "open_workspace",
  closeWorkspace: "close_workspace",
  workspaceStatus: "workspace_status",
  listPeople: "list_people",
  createPerson: "create_person",
  validateWorkspace: "validate_workspace",
  recoverWorkspace: "recover_workspace",
} as const;

export type CommandName = (typeof COMMANDS)[keyof typeof COMMANDS];

export interface CommandPayloadMap {
  readonly open_workspace: { readonly path: string };
  readonly close_workspace: Record<string, never>;
  readonly workspace_status: Record<string, never>;
  readonly list_people: { readonly includeArchived: boolean };
  readonly create_person: {
    readonly displayName: string;
    readonly email?: string;
  };
  readonly validate_workspace: Record<string, never>;
  readonly recover_workspace: Record<string, never>;
}

export interface CommandResponseMap {
  readonly open_workspace: WorkspaceSummary;
  readonly close_workspace: null;
  readonly workspace_status: WorkspaceSummary;
  readonly list_people: readonly PersonSummary[];
  readonly create_person: PersonSummary;
  readonly validate_workspace: HealthReport;
  readonly recover_workspace: OperationStatus;
}

export interface CommandTransport {
  invoke<Name extends CommandName>(
    name: Name,
    payload: CommandPayloadMap[Name],
  ): Promise<CommandResult<CommandResponseMap[Name]>>;
}

export function assertApplicationResponse<T>(value: unknown): CommandResult<T> {
  if (typeof value !== "object" || value === null) {
    throw new TypeError("application response must be an object");
  }
  const candidate = value as { schema?: unknown; ok?: unknown };
  if (candidate.schema !== APPLICATION_RESPONSE || typeof candidate.ok !== "boolean") {
    throw new TypeError("application response does not match liaison/application-response@1");
  }
  return value as CommandResult<T>;
}
