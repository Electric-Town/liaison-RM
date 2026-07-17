# Provider packages

Providers are optional adapters behind versioned capability contracts. A provider package does not define People, Events, Sharing, Facilities, or other business-domain rules.

## Required package contents

```text
providers/<provider>/
├── descriptor.json
├── README.md
├── config.schema.json
├── migrations/
├── evidence/
└── source crate or component reference
```

A descriptor declares:

- stable provider ID and version;
- capability contract versions and operations;
- configuration schema;
- opaque secret slots;
- network destinations and redirect rules;
- consistency and limits;
- safe modes;
- conformance report reference.

## Registration rules

- Registration validates the descriptor and contract versions.
- Registration grants no access to workspace data.
- A configured connection remains inert until a purpose-bound grant is active.
- Provider-specific types remain behind an anti-corruption layer.
- A provider cannot advertise backup, publication, transport, contact sync, or multi-writer sync beyond its current conformance evidence.
- Provider code cannot obtain a raw database handle or unrestricted workspace path.

## Review requirements

A provider PR includes:

- threat analysis;
- dependency and licence review;
- destination and OAuth scope review where applicable;
- synthetic or emulator-based conformance evidence;
- retry, idempotency, pagination, rate, size, and revocation behaviour;
- setup, test, backup/restore, revocation, failure, and recovery knowledge;
- redaction checks;
- Airgap exclusion evidence;
- an explicit statement of safe modes and unsupported semantics.

Provider API success is not sufficient evidence of a recoverable backup or safe synchronisation mode.

## Initial reference

`local-folder` is the reference implementation for `object-store@1`. It is intended for local/removable storage, backup, restore, and single-writer publication. It does not claim safe multi-writer synchronisation.
