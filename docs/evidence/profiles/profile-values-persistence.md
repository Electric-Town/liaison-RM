# Profile-value persistence evidence plan

Status: draft and not release-ready.

This slice will be considered reviewable only when the exact branch head demonstrates:

- a separate revisioned profile-value record per Person;
- readable YAML sidecars under `profiles/`;
- optimistic revision conflict rejection;
- unknown top-level extension preservation;
- rejection of sealed values before plaintext persistence;
- unchanged Person Markdown after profile-value edits;
- domain, adapter, architecture, repository, and full-workspace checks on Linux, macOS, and Windows.

The evidence does not claim encrypted sealed storage, Topic Pack inheritance, profile layouts, imports, desktop editing, or production schema stability.
