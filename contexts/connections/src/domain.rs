use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ProviderId(String);

impl ProviderId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ProviderDomainError> {
        let value = value.into();
        let segments = value.split('.').collect::<Vec<_>>();
        if segments.len() < 3 || segments.iter().any(|segment| !valid_segment(segment)) {
            return Err(ProviderDomainError::new(
                "provider ID must use at least three lowercase reverse-domain segments",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for ProviderId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderVersion(String);

impl ProviderVersion {
    pub fn parse(value: impl Into<String>) -> Result<Self, ProviderDomainError> {
        let value = value.into();
        let (without_build, build) = value
            .split_once('+')
            .map_or((value.as_str(), None), |(core, suffix)| {
                (core, Some(suffix))
            });
        if value.matches('+').count() > 1
            || build.is_some_and(|suffix| !valid_version_suffix(suffix))
        {
            return Err(ProviderDomainError::new(
                "provider version has an invalid build suffix",
            ));
        }

        let (core, prerelease) = without_build
            .split_once('-')
            .map_or((without_build, None), |(core, suffix)| (core, Some(suffix)));
        if prerelease.is_some_and(|suffix| !valid_version_suffix(suffix)) {
            return Err(ProviderDomainError::new(
                "provider version has an invalid prerelease suffix",
            ));
        }

        let parts = core.split('.').collect::<Vec<_>>();
        if parts.len() != 3
            || parts
                .iter()
                .any(|part| part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()))
        {
            return Err(ProviderDomainError::new(
                "provider version must start with major.minor.patch",
            ));
        }
        Ok(Self(value))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SafeMode {
    Import,
    Export,
    Backup,
    SingleWriter,
    MultiWriter,
}

impl SafeMode {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Import => "import",
            Self::Export => "export",
            Self::Backup => "backup",
            Self::SingleWriter => "single-writer",
            Self::MultiWriter => "multi-writer",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContractDescriptor {
    name: String,
    version: u32,
    operations: Vec<String>,
    safe_modes: Vec<SafeMode>,
    consistency: String,
}

impl ContractDescriptor {
    pub fn new(
        name: impl Into<String>,
        version: u32,
        operations: Vec<String>,
        safe_modes: Vec<SafeMode>,
        consistency: impl Into<String>,
    ) -> Result<Self, ProviderDomainError> {
        let name = name.into();
        if !valid_kebab_name(&name) {
            return Err(ProviderDomainError::new(
                "contract name must use kebab case",
            ));
        }
        if version == 0 {
            return Err(ProviderDomainError::new(
                "contract version must be at least one",
            ));
        }
        let operations = normalize_names(operations, "operation")?;
        if operations.is_empty() {
            return Err(ProviderDomainError::new(
                "contract must declare at least one operation",
            ));
        }
        let unique_modes = safe_modes.iter().copied().collect::<BTreeSet<_>>();
        if unique_modes.len() != safe_modes.len() {
            return Err(ProviderDomainError::new(
                "contract safe modes cannot contain duplicates",
            ));
        }
        if safe_modes.is_empty() {
            return Err(ProviderDomainError::new(
                "contract must declare at least one safe mode",
            ));
        }
        let consistency = consistency.into().trim().to_owned();
        if consistency.is_empty() {
            return Err(ProviderDomainError::new(
                "contract consistency statement is required",
            ));
        }

        Ok(Self {
            name,
            version,
            operations,
            safe_modes,
            consistency,
        })
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    #[must_use]
    pub fn operations(&self) -> &[String] {
        &self.operations
    }

    #[must_use]
    pub fn safe_modes(&self) -> &[SafeMode] {
        &self.safe_modes
    }

    #[must_use]
    pub fn consistency(&self) -> &str {
        &self.consistency
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigurationValueType {
    String,
    Integer,
    Boolean,
    StringList,
    SecretReference,
}

impl ConfigurationValueType {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::String => "string",
            Self::Integer => "integer",
            Self::Boolean => "boolean",
            Self::StringList => "string-list",
            Self::SecretReference => "secret-ref",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConfigurationField {
    key: String,
    value_type: ConfigurationValueType,
    secret: bool,
    required: bool,
    description: String,
}

impl ConfigurationField {
    pub fn new(
        key: impl Into<String>,
        value_type: ConfigurationValueType,
        secret: bool,
        required: bool,
        description: impl Into<String>,
    ) -> Result<Self, ProviderDomainError> {
        let key = key.into();
        if !valid_snake_name(&key) {
            return Err(ProviderDomainError::new(
                "configuration key must use snake case",
            ));
        }
        if secret != (value_type == ConfigurationValueType::SecretReference) {
            return Err(ProviderDomainError::new(
                "secret fields must use secret-ref and non-secret fields must not",
            ));
        }
        let description = description.into().trim().to_owned();
        if description.is_empty() {
            return Err(ProviderDomainError::new(
                "configuration field description is required",
            ));
        }
        Ok(Self {
            key,
            value_type,
            secret,
            required,
            description,
        })
    }

    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    #[must_use]
    pub const fn value_type(&self) -> ConfigurationValueType {
        self.value_type
    }

    #[must_use]
    pub const fn is_secret(&self) -> bool {
        self.secret
    }

    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.required
    }

    #[must_use]
    pub fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceStatus {
    NotTested,
    Passed,
    PassedWithLimits,
    Failed,
}

impl ConformanceStatus {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotTested => "not-tested",
            Self::Passed => "passed",
            Self::PassedWithLimits => "passed-with-limits",
            Self::Failed => "failed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDescriptor {
    id: ProviderId,
    version: ProviderVersion,
    display_name: String,
    contracts: Vec<ContractDescriptor>,
    configuration_fields: Vec<ConfigurationField>,
    network_destinations: Vec<String>,
    conformance_status: ConformanceStatus,
}

impl ProviderDescriptor {
    pub fn new(
        id: ProviderId,
        version: ProviderVersion,
        display_name: impl Into<String>,
        contracts: Vec<ContractDescriptor>,
        configuration_fields: Vec<ConfigurationField>,
        network_destinations: Vec<String>,
        conformance_status: ConformanceStatus,
    ) -> Result<Self, ProviderDomainError> {
        let display_name = display_name.into().trim().to_owned();
        if display_name.is_empty() || display_name.chars().count() > 120 {
            return Err(ProviderDomainError::new(
                "provider display name must contain 1 to 120 characters",
            ));
        }
        if contracts.is_empty() {
            return Err(ProviderDomainError::new(
                "provider must implement at least one contract",
            ));
        }

        let contract_keys = contracts
            .iter()
            .map(|contract| (contract.name().to_owned(), contract.version()))
            .collect::<BTreeSet<_>>();
        if contract_keys.len() != contracts.len() {
            return Err(ProviderDomainError::new(
                "provider cannot repeat a contract name and version",
            ));
        }

        let field_keys = configuration_fields
            .iter()
            .map(|field| field.key().to_owned())
            .collect::<BTreeSet<_>>();
        if field_keys.len() != configuration_fields.len() {
            return Err(ProviderDomainError::new(
                "provider cannot repeat a configuration field",
            ));
        }

        let destinations = network_destinations
            .into_iter()
            .map(|destination| destination.trim().to_owned())
            .collect::<Vec<_>>();
        if destinations.iter().any(String::is_empty) {
            return Err(ProviderDomainError::new(
                "network destination cannot be empty",
            ));
        }
        if destinations.iter().collect::<BTreeSet<_>>().len() != destinations.len() {
            return Err(ProviderDomainError::new(
                "network destinations cannot contain duplicates",
            ));
        }

        Ok(Self {
            id,
            version,
            display_name,
            contracts,
            configuration_fields,
            network_destinations: destinations,
            conformance_status,
        })
    }

    #[must_use]
    pub fn id(&self) -> &ProviderId {
        &self.id
    }

    #[must_use]
    pub fn version(&self) -> &ProviderVersion {
        &self.version
    }

    #[must_use]
    pub fn display_name(&self) -> &str {
        &self.display_name
    }

    #[must_use]
    pub fn contracts(&self) -> &[ContractDescriptor] {
        &self.contracts
    }

    #[must_use]
    pub fn configuration_fields(&self) -> &[ConfigurationField] {
        &self.configuration_fields
    }

    #[must_use]
    pub fn network_destinations(&self) -> &[String] {
        &self.network_destinations
    }

    #[must_use]
    pub const fn conformance_status(&self) -> ConformanceStatus {
        self.conformance_status
    }
}

fn valid_version_suffix(value: &str) -> bool {
    !value.is_empty()
        && value.split('.').all(|segment| {
            !segment.is_empty()
                && segment
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
}

fn normalize_names(values: Vec<String>, label: &str) -> Result<Vec<String>, ProviderDomainError> {
    let normalized = values
        .into_iter()
        .map(|value| value.trim().to_owned())
        .collect::<Vec<_>>();
    if normalized.iter().any(|value| !valid_kebab_name(value)) {
        return Err(ProviderDomainError::new(format!(
            "{label} names must use kebab case"
        )));
    }
    if normalized.iter().collect::<BTreeSet<_>>().len() != normalized.len() {
        return Err(ProviderDomainError::new(format!(
            "{label} names cannot contain duplicates"
        )));
    }
    Ok(normalized)
}

fn valid_segment(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn valid_kebab_name(value: &str) -> bool {
    valid_segment(value)
}

fn valid_snake_name(value: &str) -> bool {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDomainError {
    message: String,
}

impl ProviderDomainError {
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for ProviderDomainError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl Error for ProviderDomainError {}

#[cfg(test)]
mod tests {
    use super::{
        ConfigurationField, ConfigurationValueType, ConformanceStatus, ContractDescriptor,
        ProviderDescriptor, ProviderId, ProviderVersion, SafeMode,
    };

    #[test]
    fn provider_id_uses_reverse_domain_segments() {
        assert!(ProviderId::parse("org.electric-town.local-folder").is_ok());
        assert!(ProviderId::parse("local-folder").is_err());
        assert!(ProviderId::parse("Org.Electric.Provider").is_err());
    }

    #[test]
    fn provider_version_accepts_semantic_suffixes() {
        assert!(ProviderVersion::parse("1.2.3-alpha.1+linux-x64").is_ok());
        assert!(ProviderVersion::parse("1.2.3+").is_err());
        assert!(ProviderVersion::parse("1.2").is_err());
    }

    #[test]
    fn secret_field_requires_secret_reference_type() {
        assert!(
            ConfigurationField::new(
                "token",
                ConfigurationValueType::String,
                true,
                true,
                "Credential"
            )
            .is_err()
        );
    }

    #[test]
    fn duplicate_contract_is_rejected() {
        let contract = ContractDescriptor::new(
            "object-store",
            1,
            vec!["get".to_owned()],
            vec![SafeMode::Backup],
            "Read-only fixture",
        );
        assert!(contract.is_ok());
        let Ok(contract) = contract else {
            return;
        };
        let provider_id = ProviderId::parse("org.example.provider");
        let version = ProviderVersion::parse("1.0.0");
        assert!(provider_id.is_ok());
        assert!(version.is_ok());
        let (Ok(provider_id), Ok(version)) = (provider_id, version) else {
            return;
        };
        let result = ProviderDescriptor::new(
            provider_id,
            version,
            "Example",
            vec![contract.clone(), contract],
            vec![],
            vec![],
            ConformanceStatus::NotTested,
        );
        assert!(result.is_err());
    }
}
