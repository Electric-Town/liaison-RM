# Connections bounded context

## Purpose

Own configured external capabilities without letting provider vocabulary enter relationship, event, sharing, or AI policy.

## Language

- provider
- connection
- capability contract
- descriptor
- safe mode
- configuration field
- secret reference
- grant
- conformance
- health

## Aggregates and policies

The initial `ProviderDescriptor` protects:

- stable reverse-domain provider ID;
- semantic provider version;
- unique contract versions;
- explicit operations and safe modes;
- secret-reference field typing;
- non-empty consistency statement;
- declared network destinations;
- conformance status.

Connection and grant aggregates arrive in later slices.

## Application use cases

- register provider;
- list providers;
- later: create connection, grant purpose, test health, export settings, import settings, pause, and revoke.

## Ports

- `ProviderRegistry`
- `ObjectStore`

Provider adapters implement ports. Other contexts consume the ports through application services and do not import provider SDKs.

## Requirements and UAT

- `LRM-CO-001` through `LRM-CO-009`
- `LRM-CO-012`
- `UAT-026` through `UAT-028`
- `UAT-034`
