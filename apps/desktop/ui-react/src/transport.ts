import {
  assertApplicationResponse,
  type CommandName,
  type CommandPayloadMap,
  type CommandResponseMap,
  type CommandResult,
  type CommandTransport,
} from "./application-contract";

export type NativeInvoke = (
  command: string,
  payload?: Record<string, unknown>,
) => Promise<unknown>;

export function createTauriTransport(invoke: NativeInvoke): CommandTransport {
  return {
    async invoke<Name extends CommandName>(
      name: Name,
      payload: CommandPayloadMap[Name],
    ): Promise<CommandResult<CommandResponseMap[Name]>> {
      const response = await invoke(name, payload as Record<string, unknown>);
      return assertApplicationResponse<CommandResponseMap[Name]>(response);
    },
  };
}

export function unavailableTransport(): CommandTransport {
  return {
    async invoke<Name extends CommandName>(
      _name: Name,
      _payload: CommandPayloadMap[Name],
    ): Promise<CommandResult<CommandResponseMap[Name]>> {
      return Promise.resolve({
        schema: "liaison/application-response@1",
        ok: false,
        error: {
          code: "transport.unavailable",
          category: "internal",
          message: "The native Liaison command bridge is not available in this preview.",
          recovery: "Run the interface inside the Liaison desktop application.",
          retryable: false,
        },
      });
    },
  };
}
