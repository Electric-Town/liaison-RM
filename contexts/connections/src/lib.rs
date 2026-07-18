//! Connections bounded context.
//!
//! Owns provider descriptors, capability contracts, registry services, safe
//! operating modes, and provider-neutral storage ports. Provider-specific
//! credentials, SDKs, and transport mechanisms belong in adapters.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

mod domain;
mod object_store;

pub use domain::{
    ConfigurationField, ConfigurationValueType, ConformanceStatus, ContractDescriptor,
    ListProviders, ProviderDescriptor, ProviderDomainError, ProviderId, ProviderRegistry,
    ProviderVersion, RegisterProvider, SafeMode,
};
pub use object_store::{
    ContentDigest, ListPage, ObjectKey, ObjectMetadata, ObjectStore, ObjectStoreError,
    ObjectStoreErrorKind,
};
