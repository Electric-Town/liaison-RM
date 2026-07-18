# Providers

Providers implement capability contracts from the Connections bounded context.

## Package structure

```text
providers/<provider>/
├── Cargo.toml
├── README.md
├── descriptor.json
├── src/
├── tests/
├── fixtures/
└── evidence/
```

## Required contents

- stable reverse-domain provider ID;
- versioned contracts;
- non-secret configuration fields;
- secret references;
- exact network destinations;
- operations, limits, and consistency statement;
- safe-mode claim;
- conformance results;
- retry and idempotency behaviour;
- recovery instructions;
- synthetic fixtures;
- licence provenance.

## Safe-mode labels

- **import:** one-way source read;
- **export:** one-way destination write;
- **backup:** encrypted snapshot or objects with restore verification;
- **single-writer:** one active writer, possibly several readers;
- **multi-writer:** concurrent operation exchange with safe conditional manifests.

A provider must not claim a stronger mode than its configured service proves.

## Initial provider plan

| Provider | Contract | Earliest release |
|---|---|---|
| Local folder | object-store@1 | R1 |
| Removable media | object-store@1 profile | R4 |
| WebDAV | object-store@1 | R4 |
| S3-compatible | object-store@1 | R4 |
| CardDAV | address-book@1 | R5 |
| CalDAV/ICS | calendar@1 | R5 |
| Gmail metadata | mail-metadata@1 | R5 |
| Ollama | ai-model@1 | R5 |
| Google Drive | object-store@1 where conformance permits | R5 |
| Google Cloud Storage | object-store@1 | later |
| Azure Blob Storage | object-store@1 | later |

The product has no preferred commercial provider.
