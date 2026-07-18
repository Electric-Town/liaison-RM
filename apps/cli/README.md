# Liaison CLI

`liaison` is a supported product surface, not a maintenance-only wrapper around the desktop UI. It calls the same Rust application services and writes the same canonical workspace.

## Current R1 vertical slice

```text
liaison --workspace PATH workspace init --name NAME [--profile personal] [--build-profile airgap]
liaison --workspace PATH workspace inspect
liaison --workspace PATH workspace validate
liaison --workspace PATH person create --name NAME [--email ADDRESS]
liaison --workspace PATH person list [--include-archived]
liaison --workspace PATH backup create --destination NEW_BACKUP_DIRECTORY
liaison backup verify --backup BACKUP_DIRECTORY
liaison backup restore --backup BACKUP_DIRECTORY --target NEW_RESTORE_DIRECTORY
```

Use `--output json` for automation. Human output is written to standard output; structured errors are written to standard error. Exit codes are stable by error class rather than provider or operating-system message.

## Backup and restore contract

- `backup create` validates the source workspace, excludes disposable projections and transient state, rejects symbolic links, and refuses an existing or nested destination.
- `backup verify` checks the manifest, exact payload set, byte sizes, and SHA-256 digests.
- `backup restore` refuses an existing target, restores into an isolated directory, validates workspace identity, schema, and layout, and removes its restore marker only after activation succeeds.
- A failed verification does not create a restore target.
- This slice creates an unencrypted local directory backup. Provider upload, encryption, retention, scheduling, and removable-media policy remain separate gates.

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

The current `workspace init`, `person create`, `backup create`, and isolated `backup restore` commands are additive and refuse existing targets or duplicate identity. Edit, import, migration, sharing, provider, and deletion commands remain gated until their dry-run and recovery contracts are implemented.

## Examples

```bash
liaison --workspace "$HOME/People" workspace init \
  --name "People" \
  --profile personal \
  --build-profile airgap

liaison --workspace "$HOME/People" person create \
  --name "Alex Example" \
  --email "alex@example.test"

liaison --workspace "$HOME/People" backup create \
  --destination "$HOME/Backups/people-001"

liaison backup verify --backup "$HOME/Backups/people-001"
liaison backup restore \
  --backup "$HOME/Backups/people-001" \
  --target "$HOME/Restore-tests/people-001"

liaison --workspace "$HOME/People" --output json person list
liaison --workspace "$HOME/People" workspace validate
```

Examples use synthetic `.test` addresses.
