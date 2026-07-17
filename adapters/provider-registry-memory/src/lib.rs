//! In-memory provider registry used by the CLI and conformance tests.

use liaison_connections::{
    ProviderDescriptor, ProviderDomainError, ProviderId, ProviderRegistry,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default)]
pub struct MemoryProviderRegistry {
    providers: BTreeMap<ProviderId, ProviderDescriptor>,
}

impl ProviderRegistry for MemoryProviderRegistry {
    fn register(&mut self, descriptor: ProviderDescriptor) -> Result<(), ProviderDomainError> {
        let id = descriptor.id().clone();
        if self.providers.contains_key(&id) {
            return Err(ProviderDomainError::new(format!(
                "provider {id} is already registered"
            )));
        }
        self.providers.insert(id, descriptor);
        Ok(())
    }

    fn list(&self) -> Result<Vec<ProviderDescriptor>, ProviderDomainError> {
        Ok(self.providers.values().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::MemoryProviderRegistry;
    use liaison_connections::{
        ConformanceStatus, ContractDescriptor, ProviderDescriptor, ProviderId, ProviderRegistry,
        ProviderVersion, SafeMode,
    };

    fn fixture() -> Result<ProviderDescriptor, liaison_connections::ProviderDomainError> {
        ProviderDescriptor::new(
            ProviderId::parse("org.example.fixture")?,
            ProviderVersion::parse("1.0.0")?,
            "Fixture",
            vec![ContractDescriptor::new(
                "object-store",
                1,
                vec!["get".to_owned()],
                vec![SafeMode::Backup],
                "Synthetic test fixture",
            )?],
            vec![],
            vec![],
            ConformanceStatus::NotTested,
        )
    }

    #[test]
    fn lists_descriptors_in_stable_provider_id_order() {
        let mut registry = MemoryProviderRegistry::default();
        let first = fixture();
        assert!(first.is_ok());
        if let Ok(first) = first {
            assert!(registry.register(first).is_ok());
            let providers = registry.list();
            assert!(providers.is_ok());
            if let Ok(providers) = providers {
                assert_eq!(providers.len(), 1);
                assert_eq!(providers[0].id().as_str(), "org.example.fixture");
            }
        }
    }

    #[test]
    fn rejects_duplicate_provider_identity() {
        let mut registry = MemoryProviderRegistry::default();
        let descriptor = fixture();
        assert!(descriptor.is_ok());
        if let Ok(descriptor) = descriptor {
            assert!(registry.register(descriptor.clone()).is_ok());
            assert!(registry.register(descriptor).is_err());
        }
    }
}
