# Liaison CLI

`liaison` is a supported product surface over the same `liaison-application`
composition root used by the desktop interface. The CLI does not construct
context services or storage adapters itself.

## Current review-build commands

Every command requires an explicit absolute workspace path. Liaison never
defaults a mutation to the current directory.

```text
liaison --workspace PATH workspace init --name NAME [--profile workplace] [--build-profile connected-local]
liaison --workspace PATH workspace inspect
liaison --workspace PATH workspace validate
liaison --workspace PATH person create --name NAME [--email ADDRESS]
liaison --workspace PATH person list [--include-archived]
```

Each process opens or initialises a write-authoritative application workspace
session before executing inspection or People commands. The session owns a
composite path-local and per-user `WorkspaceId` operating-system authority plus
root-bound repositories for the command lifetime. A copied workspace with the
same identity returns `workspace.identity-writer-already-active` while the
original is open. `workspace validate` is different by design: it runs lock-free,
one-shot read-only Health so a second process can inspect a contended,
malformed, or newer-schema workspace without acquiring writer authority.
Recoverable operations and final mutation preconditions remain later gates.

The review CLI defaults the manifest declaration to `connected-local` because
the current artifact has not passed the Airgap dependency and socket-denial
gates. Selecting `airgap` records workspace policy only; it does not prove the
binary has no network capability.

Use `--output json` for automation. Successful JSON output includes the
application command ID and completion time. Application errors are written to
standard error and preserve this envelope:

```json
{
  "error": {
    "contract_version": 1,
    "code": "workspace.not-found",
    "message": "workspace does not exist",
    "recovery": "choose an existing Liaison workspace or initialise a new one",
    "details": {},
    "correlation_id": "00000000-0000-0000-0000-000000000000"
  },
  "exit_code": 3
}
```

The CLI does not add command arguments or other user data to that error
envelope.

## Exit codes

| Code | Meaning |
|---:|---|
| 0 | Command completed, or workspace validation is valid |
| 1 | General application or output error |
| 2 | CLI usage error, including omitted `--workspace` |
| 3 | Workspace, session, or Person not found |
| 4 | Existing target, active writer, stale session, or revision conflict |
| 5 | Unsupported workspace schema |
| 6 | Validation completed and returned one or more error findings |

Validation code `6` is a diagnostic result, not an exception. Its complete
structured report is written to standard output and standard error remains
empty.

## Contract for mutating commands

As the CLI expands, every mutation must support:

- explicit workspace selection;
- `--dry-run` when the command can produce a meaningful preview;
- revision or other preconditions;
- non-interactive input without placing secrets on the command line;
- structured JSON output;
- audit attribution;
- confirmation proportionate to irreversible consequences;
- recovery instructions.

The current `workspace init` and `person create` commands are additive and
refuse existing targets or duplicate identity. Edit, import, migration,
checkpoint, encrypted recovery, sharing, provider, and deletion commands remain
gated until their preview and recovery contracts are implemented.

## Examples

```bash
liaison --workspace "/absolute/path/to/People" workspace init \
  --name "People" \
  --profile workplace \
  --build-profile connected-local

liaison --workspace "/absolute/path/to/People" person create \
  --name "Alex Example" \
  --email "alex@example.test"

liaison --workspace "/absolute/path/to/People" --output json person list
liaison --workspace "/absolute/path/to/People" workspace validate
```

Examples use synthetic `.test` addresses.
