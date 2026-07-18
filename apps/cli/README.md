# Liaison CLI

`liaison` is a supported product surface, not a maintenance-only wrapper around the desktop UI. It calls Rust application services and writes the same canonical workspace.

## Current commands

```text
liaison --workspace PATH workspace init --name NAME [--profile personal] [--build-profile airgap]
liaison --workspace PATH workspace inspect
liaison --workspace PATH workspace validate
liaison --workspace PATH person create --name NAME [--email ADDRESS]
liaison --workspace PATH person list [--include-archived]
liaison --workspace PATH relationship show --person-id UUID [--as-of YYYY-MM-DD]
liaison --workspace PATH relationship list [--as-of YYYY-MM-DD]
liaison --workspace PATH relationship set --person-id UUID [RELATIONSHIP OPTIONS]
```

Use `--output json` for automation. Human output is written to standard output; structured errors are written to standard error. Exit codes are stable by error class rather than provider or operating-system message.

## Relationship updates

The first `relationship set` omits `--expected-revision`. Later edits must provide the displayed revision:

```bash
liaison --workspace "$HOME/People" relationship set \
  --person-id 01900000-0000-7000-8000-000000000001 \
  --relationship-type friend \
  --tier core \
  --cadence quarterly \
  --last-contacted 2026-06-01 \
  --reason "Ask how the move went" \
  --circle "Close friends"

liaison --workspace "$HOME/People" relationship set \
  --person-id 01900000-0000-7000-8000-000000000001 \
  --expected-revision 1 \
  --topic "Starting a new role"
```

Omitted fields retain their existing values. Explicit `--clear-*` flags remove optional values. `--do-not-contact` and `--allow-contact` are mutually exclusive hard-boundary controls.

## Mutation contract

As the CLI expands, every mutation must support:

- explicit workspace selection;
- dry-run when a meaningful preview exists;
- revision or other preconditions;
- non-interactive input without placing secrets on the command line;
- structured JSON output;
- audit attribution;
- confirmation proportionate to irreversible consequences;
- recovery instructions.

Examples use synthetic `.test` addresses.
