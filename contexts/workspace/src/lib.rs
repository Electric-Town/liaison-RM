//! Workspace bounded context.
//!
//! Owns workspace identity, manifest invariants, lifecycle use cases, and the
//! repository port required by storage adapters.

#![allow(clippy::module_name_repetitions, clippy::missing_errors_doc)]

use liaison_shared_kernel::WorkspaceId;
use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

pub const WORKSPACE_FORMAT: &str = "liaison-workspace";
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildProfile {
    Airgap,
    ConnectedLocal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum WorkspaceProfile {
    Personal,
    Family,
    Team,
    Workplace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceManifest {
    pub format: String,
    pub schema_version: u32,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub profile: WorkspaceProfile,
    pub build_profile: BuildProfile,
    pub default_locale: String,
}

impl WorkspaceManifest {
    pub fn new(
        workspace_id: WorkspaceId,
        name: impl Into<String>,
        profile: WorkspaceProfile,
        build_profile: BuildProfile,
        default_locale: impl Into<String>,
    ) -> Result<Self, WorkspaceError> {
        let name = name.into();
        let name = normalise_required(&name, "workspace name")?;
        let default_locale = default_locale.into();
        let default_locale = normalise_required(&default_locale, "default locale")?;
        Ok(Self {
            format: WORKSPACE_FORMAT.to_owned(),
            schema_version: CURRENT_SCHEMA_VERSION,
            workspace_id,
            name,
            profile,
            build_profile,
            default_locale,
        })
    }

    pub fn validate(&self) -> Result<(), WorkspaceError> {
        if self.format != WORKSPACE_FORMAT {
            return Err(WorkspaceError::UnexpectedFormat(self.format.clone()));
        }
        if self.schema_version != CURRENT_SCHEMA_VERSION {
            return Err(WorkspaceError::UnsupportedSchema {
                found: self.schema_version,
                supported: CURRENT_SCHEMA_VERSION,
            });
        }
        normalise_required(&self.name, "workspace name")?;
        normalise_required(&self.default_locale, "default locale")?;
        Ok(())
    }
}

fn normalise_required(value: &str, field: &'static str) -> Result<String, WorkspaceError> {
    let value = value.trim().to_owned();
    if value.is_empty() {
        Err(WorkspaceError::RequiredField(field))
    } else {
        Ok(value)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum WorkspaceError {
    #[error("{0} is required")]
    RequiredField(&'static str),
    #[error("unexpected workspace format: {0}")]
    UnexpectedFormat(String),
    #[error("workspace schema {found} is not supported; this build supports {supported}")]
    UnsupportedSchema { found: u32, supported: u32 },
    #[error("workspace already exists")]
    AlreadyExists,
    #[error("workspace initialisation target is not empty")]
    InitialiseTargetNotEmpty,
    #[error("workspace does not exist")]
    NotFound,
    #[error("workspace storage error: {0}")]
    Storage(String),
}

pub trait WorkspaceStore: Send + Sync {
    fn initialise(&self, root: &Path, manifest: &WorkspaceManifest) -> Result<(), WorkspaceError>;

    fn load(&self, root: &Path) -> Result<WorkspaceManifest, WorkspaceError>;

    fn validate_layout(&self, root: &Path) -> Result<Vec<ValidationFinding>, WorkspaceError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ValidationFinding {
    pub code: String,
    pub severity: FindingSeverity,
    pub path: String,
    pub message: String,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FindingSeverity {
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct InitialiseWorkspace<S> {
    store: S,
}

impl<S> InitialiseWorkspace<S>
where
    S: WorkspaceStore,
{
    #[must_use]
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub fn execute(
        &self,
        root: &Path,
        workspace_id: WorkspaceId,
        name: impl Into<String>,
        profile: WorkspaceProfile,
        build_profile: BuildProfile,
        locale: impl Into<String>,
    ) -> Result<WorkspaceManifest, WorkspaceError> {
        let manifest = WorkspaceManifest::new(workspace_id, name, profile, build_profile, locale)?;
        self.store.initialise(root, &manifest)?;
        Ok(manifest)
    }
}

#[derive(Debug)]
pub struct ValidateWorkspace<S> {
    store: S,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct WorkspaceValidationReport {
    pub workspace_id: WorkspaceId,
    pub schema_version: u32,
    pub findings: Vec<ValidationFinding>,
}

impl WorkspaceValidationReport {
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self
            .findings
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Error)
    }
}

impl<S> ValidateWorkspace<S>
where
    S: WorkspaceStore,
{
    #[must_use]
    pub const fn new(store: S) -> Self {
        Self { store }
    }

    pub fn execute(&self, root: &Path) -> Result<WorkspaceValidationReport, WorkspaceError> {
        let manifest = self.store.load(root)?;
        manifest.validate()?;
        let findings = self.store.validate_layout(root)?;
        Ok(WorkspaceValidationReport {
            workspace_id: manifest.workspace_id,
            schema_version: manifest.schema_version,
            findings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{BuildProfile, WorkspaceError, WorkspaceManifest, WorkspaceProfile};
    use liaison_shared_kernel::WorkspaceId;

    #[test]
    fn manifest_rejects_blank_name() {
        let result = WorkspaceManifest::new(
            WorkspaceId::new(),
            "  ",
            WorkspaceProfile::Personal,
            BuildProfile::Airgap,
            "en-IE",
        );
        assert_eq!(result, Err(WorkspaceError::RequiredField("workspace name")));
    }

    #[test]
    fn manifest_uses_current_format_and_schema() {
        let result = WorkspaceManifest::new(
            WorkspaceId::new(),
            "People",
            WorkspaceProfile::Personal,
            BuildProfile::Airgap,
            "en-IE",
        );
        assert!(result.is_ok());
        let manifest = result.unwrap_or_else(|error| unreachable!("unexpected error: {error}"));
        assert_eq!(manifest.format, "liaison-workspace");
        assert_eq!(manifest.schema_version, 1);
        assert!(manifest.validate().is_ok());
    }
}
