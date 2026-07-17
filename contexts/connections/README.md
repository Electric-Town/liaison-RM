# Connections bounded context

## Purpose

Own configured external capabilities without allowing provider vocabulary to enter relationship, event, sharing, review, or AI policy.

## Ubiquitous language

- **Provider:** an installed implementation of one or more capability contracts.
- **Capability contract:** a versioned provider-neutral interface such as `object-store@1`.
- **Descriptor:** inspectable provider metadata: identity, version, operations, safe modes, configuration fields, network destinations, and conformance status.
- **Safe mode:** the strongest operation mode proven for a provider and configuration.
- **Connection:** a configured provider instance. A descriptor alone grants no access.
- **Secret reference:** an opaque pointer to secret storage; never the secret value.
- **Grant:** a purpose-, field-, operation-, endpoint-, and time-bounded authorization.
- **Conformance:** evidence that a provider implementation meets a contract under a recorded configuration.

## Aggregate and service boundary

The initial `ProviderDescriptor` protects:

- stable reverse-domain provider identity;
- semantic provider version;
- unique contract name/version pairs;
- explicit operations and safe modes;
- typed configuration fields and secret references;
- non-empty consistency statements;
- declared network destinations;
- explicit conformance status.

The initial application services register and list descriptors. Connection, grant, health, retry, and revocation aggregates arrive in later slices.

## Ports

- `ProviderRegistry`
- `ObjectStore`

Provider adapters implement these ports. Other bounded contexts consume application services or ports through explicit contracts; they must not import Google, AWS, WebDAV, filesystem, HTTP, or plugin-runtime types.

## Safety rule

A successful upload does not prove synchronization safety. The local reference provider claims only `backup` and `single-writer`. `multi-writer` requires conditional-manifest and concurrency evidence against the actual configured service.

## Requirements and UAT

- REQ-161 through REQ-164
- UAT-089 through UAT-092
