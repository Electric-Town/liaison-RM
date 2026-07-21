import { useCallback, useRef, useState } from "react";
import type {
  CommandError,
  CommandTransport,
  HealthReport,
  OperationPhase,
  OperationStatus,
  PersonSummary,
  WorkspaceProfile,
  WorkspaceSummary,
} from "./application-contract";

export interface DesktopController {
  readonly workspace: WorkspaceSummary | null;
  readonly people: readonly PersonSummary[];
  readonly health: HealthReport | null;
  readonly operation: OperationStatus;
  readonly busy: boolean;
  initialiseWorkspace(input: {
    readonly path: string;
    readonly displayName: string;
    readonly profile: WorkspaceProfile;
  }): Promise<boolean>;
  openWorkspace(path: string): Promise<boolean>;
  closeWorkspace(): Promise<boolean>;
  refreshPeople(): Promise<boolean>;
  createPerson(input: {
    readonly displayName: string;
    readonly email?: string;
  }): Promise<PersonSummary | null>;
  validateWorkspace(): Promise<boolean>;
  recoverWorkspace(): Promise<boolean>;
}

const idleOperation: OperationStatus = {
  operationId: "ui-idle",
  phase: "idle",
  headline: "Ready",
  detail: "No canonical operation is running.",
  interruptible: true,
};

function phaseForError(error: CommandError): OperationPhase {
  return error.category === "conflict" ? "conflict" : "failed";
}

function failureStatus(operationId: string, error: CommandError): OperationStatus {
  return {
    operationId,
    phase: phaseForError(error),
    headline: error.category === "conflict" ? "Review required" : "Operation did not finish",
    detail: error.message,
    interruptible: true,
    error,
  };
}

export function useDesktopController(transport: CommandTransport): DesktopController {
  const [workspace, setWorkspace] = useState<WorkspaceSummary | null>(null);
  const [people, setPeople] = useState<readonly PersonSummary[]>([]);
  const [health, setHealth] = useState<HealthReport | null>(null);
  const [operation, setOperation] = useState<OperationStatus>(idleOperation);
  const [busy, setBusy] = useState(false);
  const inFlight = useRef(false);
  const sequence = useRef(0);

  const begin = useCallback((headline: string, detail: string): string | null => {
    if (inFlight.current) {
      return null;
    }
    inFlight.current = true;
    setBusy(true);
    sequence.current += 1;
    const operationId = `ui-operation-${sequence.current}`;
    setOperation({
      operationId,
      phase: "validating",
      headline,
      detail,
      interruptible: false,
    });
    return operationId;
  }, []);

  const finish = useCallback(() => {
    inFlight.current = false;
    setBusy(false);
  }, []);

  const loadPeople = useCallback(async (): Promise<boolean> => {
    const result = await transport.invoke("list_people", { includeArchived: false });
    if (result.ok) {
      setPeople(result.value);
      return true;
    }
    setOperation(failureStatus(`ui-operation-${sequence.current}`, result.error));
    return false;
  }, [transport]);

  const initialiseWorkspace = useCallback(
    async (input: {
      readonly path: string;
      readonly displayName: string;
      readonly profile: WorkspaceProfile;
    }): Promise<boolean> => {
      const operationId = begin("Creating workspace", "Validating the selected local folder.");
      if (operationId === null) {
        return false;
      }
      try {
        const result = await transport.invoke("initialise_workspace", input);
        if (!result.ok) {
          setOperation(failureStatus(operationId, result.error));
          return false;
        }
        setWorkspace(result.value);
        setHealth(null);
        const loaded = await loadPeople();
        if (!loaded) {
          return false;
        }
        setOperation({
          operationId,
          phase: "complete",
          headline: "Workspace created",
          detail: `${result.value.displayName} is open with local write authority.`,
          interruptible: true,
        });
        return true;
      } finally {
        finish();
      }
    },
    [begin, finish, loadPeople, transport],
  );

  const openWorkspace = useCallback(
    async (path: string): Promise<boolean> => {
      const operationId = begin("Opening workspace", "Checking identity, schema, and writer authority.");
      if (operationId === null) {
        return false;
      }
      try {
        const result = await transport.invoke("open_workspace", { path });
        if (!result.ok) {
          setOperation(failureStatus(operationId, result.error));
          return false;
        }
        setWorkspace(result.value);
        setHealth(null);
        const loaded = await loadPeople();
        if (!loaded) {
          return false;
        }
        setOperation({
          operationId,
          phase: "complete",
          headline: "Workspace opened",
          detail: `${result.value.displayName} is ready.`,
          interruptible: true,
        });
        return true;
      } finally {
        finish();
      }
    },
    [begin, finish, loadPeople, transport],
  );

  const closeWorkspace = useCallback(async (): Promise<boolean> => {
    const operationId = begin("Closing workspace", "Waiting for local work to become quiescent.");
    if (operationId === null) {
      return false;
    }
    try {
      const result = await transport.invoke("close_workspace", {});
      if (!result.ok) {
        setOperation(failureStatus(operationId, result.error));
        return false;
      }
      setWorkspace(null);
      setPeople([]);
      setHealth(null);
      setOperation({
        operationId,
        phase: "complete",
        headline: "Workspace closed",
        detail: "Liaison released local write authority.",
        interruptible: true,
      });
      return true;
    } finally {
      finish();
    }
  }, [begin, finish, transport]);

  const refreshPeople = useCallback(async (): Promise<boolean> => {
    const operationId = begin("Refreshing Directory", "Reading current Person records.");
    if (operationId === null) {
      return false;
    }
    try {
      const loaded = await loadPeople();
      if (!loaded) {
        return false;
      }
      setOperation({
        operationId,
        phase: "complete",
        headline: "Directory refreshed",
        detail: "The visible list reflects the current readable Person records.",
        interruptible: true,
      });
      return true;
    } finally {
      finish();
    }
  }, [begin, finish, loadPeople]);

  const createPerson = useCallback(
    async (input: {
      readonly displayName: string;
      readonly email?: string;
    }): Promise<PersonSummary | null> => {
      const operationId = begin("Saving person", "Validating and publishing one canonical Person record.");
      if (operationId === null) {
        return null;
      }
      try {
        const payload =
          input.email === undefined
            ? { displayName: input.displayName }
            : { displayName: input.displayName, email: input.email };
        const result = await transport.invoke("create_person", payload);
        if (!result.ok) {
          setOperation(failureStatus(operationId, result.error));
          return null;
        }
        setPeople((current) =>
          [...current, result.value].sort((left, right) =>
            left.displayName.localeCompare(right.displayName),
          ),
        );
        setOperation({
          operationId,
          phase: "complete",
          headline: "Person saved",
          detail: `${result.value.displayName} was added to the local Directory.`,
          interruptible: true,
          receiptId: `${result.value.personId}@${result.value.revision}`,
        });
        return result.value;
      } finally {
        finish();
      }
    },
    [begin, finish, transport],
  );

  const validateWorkspace = useCallback(async (): Promise<boolean> => {
    const operationId = begin("Checking workspace health", "Reading without modifying canonical records.");
    if (operationId === null) {
      return false;
    }
    try {
      const result = await transport.invoke("validate_workspace", {});
      if (!result.ok) {
        setOperation(failureStatus(operationId, result.error));
        return false;
      }
      setHealth(result.value);
      const errors = result.value.findings.filter((finding) => finding.severity === "error").length;
      setOperation({
        operationId,
        phase: errors === 0 ? "complete" : "conflict",
        headline: errors === 0 ? "Health check complete" : "Health findings need review",
        detail:
          result.value.findings.length === 0
            ? "No findings were reported."
            : `${result.value.findings.length} finding(s) were reported; ${errors} require correction.`,
        interruptible: true,
      });
      return errors === 0;
    } finally {
      finish();
    }
  }, [begin, finish, transport]);

  const recoverWorkspace = useCallback(async (): Promise<boolean> => {
    const operationId = begin("Recovering workspace", "Inspecting interrupted canonical operations.");
    if (operationId === null) {
      return false;
    }
    try {
      const result = await transport.invoke("recover_workspace", {});
      if (!result.ok) {
        setOperation(failureStatus(operationId, result.error));
        return false;
      }
      setOperation(result.value);
      if (result.value.phase === "complete") {
        await loadPeople();
        return true;
      }
      return false;
    } finally {
      finish();
    }
  }, [begin, finish, loadPeople, transport]);

  return {
    workspace,
    people,
    health,
    operation,
    busy,
    initialiseWorkspace,
    openWorkspace,
    closeWorkspace,
    refreshPeople,
    createPerson,
    validateWorkspace,
    recoverWorkspace,
  };
}
