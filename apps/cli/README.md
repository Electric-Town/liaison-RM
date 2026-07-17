# Liaison CLI

`liaison` is a supported product surface, not a maintenance-only wrapper around the desktop UI. It calls the same Rust application services and writes the same canonical workspace.

## Current R1 vertical slice

```text
liaison --workspace PATH workspace init --name NAME [--profile personal] [--build-profile airgap]
liaison --workspace PATH workspace inspect
liaison --workspace PATH workspace validate
liaison --workspace PATH person create --name NAME [--email ADDRESS]
liaison --workspace PATH person list [--include-archived]
```

Use `--output json` for automation. Human output is written to standard output; structured errors are written to standard error. Exit codes are stable by error class rather than provider or operating-system message.

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

The current `workspace init` and `person create` commands are additive and refuse existing targets or duplicate identity. Edit, import, migration, restore, sharing, provider, and deletion commands remain gated until their dry-run and recovery contracts are implemented.

## Examples

```bash
liaison --workspace "$HOME/People" workspace init \
  --name "People" \
  --profile personal \
  --build-profile airgap

liaison --workspace "$HOME/People" person create \
  --name "Alex Example" \
  --email "alex@example.test"

liaison --workspace "$HOME/People" --output json person list
liaison --workspace "$HOME/People" workspace validate
```

Examples use synthetic `.test` addresses.
