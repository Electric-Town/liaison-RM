//! Application composition root for Liaison RM.
//!
//! Inbound adapters call this crate instead of constructing context services
//! themselves. A workspace path is accepted only while a workspace is opened
//! or initialised. Later commands use the resulting session identifier. The
//! session currently binds identity and repository access; writer locking,
//! recovery, key state, and projections are added by the Workspace Session
//! delivery phase rather than being implied here.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use chrono::{DateTime, NaiveDate, Utc};
use liaison_people::{
    CreatePerson, EmailAddress, ListPeople, PartialDate, PeopleError, PersonProfile, PhoneNumber,
};
pub use liaison_shared_kernel::{
    CommandId, JobId, PersonId, Revision, WorkspaceId, WorkspaceSessionId,
};
use liaison_vault_markdown::MarkdownVault;
pub use liaison_workspace::{BuildProfile, WorkspaceProfile};
use liaison_workspace::{
    FindingSeverity, InitialiseWorkspace, ValidateWorkspace, ValidationFinding, WorkspaceError,
    WorkspaceManifest, WorkspaceStore,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    path::{Component, Path, PathBuf},
    sync::{Arc, Mutex},
};
use thiserror::Error;

pub const APPLICATION_CONTRACT_VERSION: u32 = 1;

/// Runtime capabilities whose outputs must be replaceable in deterministic
/// tests. Cryptographic consumers must still apply their own purpose-specific
/// key and nonce rules to bytes returned by [`RuntimePorts::random_bytes`].
pub trait RuntimePorts: fmt::Debug + Send + Sync {
    fn now(&self) -> DateTime<Utc>;
    fn next_command_id(&self) -> CommandId;
    fn next_workspace_id(&self) -> WorkspaceId;
    fn next_person_id(&self) -> PersonId;
    fn next_workspace_session_id(&self) -> WorkspaceSessionId;
    fn next_job_id(&self) -> JobId;
    fn random_bytes(&self, length: usize) -> Result<Vec<u8>, RuntimePortError>;
    fn documents_directory(&self) -> Result<PathBuf, RuntimePortError>;
}

#[derive(Debug, Default)]
pub struct SystemRuntime;

impl RuntimePorts for SystemRuntime {
    fn now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn next_command_id(&self) -> CommandId {
        CommandId::new()
    }

    fn next_workspace_id(&self) -> WorkspaceId {
        WorkspaceId::new()
    }

    fn next_person_id(&self) -> PersonId {
        PersonId::new()
    }

    fn next_workspace_session_id(&self) -> WorkspaceSessionId {
        WorkspaceSessionId::new()
    }

    fn next_job_id(&self) -> JobId {
        JobId::new()
    }

    fn random_bytes(&self, length: usize) -> Result<Vec<u8>, RuntimePortError> {
        let mut bytes = vec![0_u8; length];
        getrandom::fill(&mut bytes)
            .map_err(|error| RuntimePortError::Randomness(error.to_string()))?;
        Ok(bytes)
    }

    fn documents_directory(&self) -> Result<PathBuf, RuntimePortError> {
        std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .map(|home| home.join("Documents"))
            .filter(|path| path.is_absolute())
            .ok_or(RuntimePortError::DocumentsDirectoryUnavailable)
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RuntimePortError {
    #[error("operating-system randomness failed: {0}")]
    Randomness(String),
    #[error("the operating system did not provide an absolute Documents directory")]
    DocumentsDirectoryUnavailable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandResult<T> {
    pub contract_version: u32,
    pub command_id: CommandId,
    pub completed_at: DateTime<Utc>,
    pub value: T,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppStatusDto {
    pub version: String,
    pub product_state: String,
    pub authority_model: String,
    pub connection_state: String,
    pub release_evidence: String,
    pub canonical_storage: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InitialiseWorkspaceCommand {
    pub path: String,
    pub name: String,
    pub profile: WorkspaceProfile,
    pub build_profile: BuildProfile,
    pub locale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenWorkspaceCommand {
    pub path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSessionCommand {
    pub session_id: WorkspaceSessionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreatePersonCommand {
    pub session_id: WorkspaceSessionId,
    pub display_name: String,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ListPeopleQuery {
    pub session_id: WorkspaceSessionId,
    pub include_archived: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceDto {
    pub session_id: WorkspaceSessionId,
    pub path: String,
    pub workspace_id: WorkspaceId,
    pub schema_version: u32,
    pub name: String,
    pub profile: WorkspaceProfile,
    pub build_profile: BuildProfile,
    pub locale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceOpenDto {
    pub workspace: WorkspaceDto,
    pub people: Vec<PersonDto>,
    pub validation: WorkspaceValidationDto,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersonDto {
    pub id: PersonId,
    pub revision: Revision,
    pub display_name: String,
    pub aliases: Vec<String>,
    pub emails: Vec<EmailDto>,
    pub phones: Vec<PhoneDto>,
    pub birthday: Option<PartialDateDto>,
    pub archived: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailDto {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneDto {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "precision", rename_all = "kebab-case")]
pub enum PartialDateDto {
    Full { date: NaiveDate },
    MonthDay { month: u8, day: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceValidationDto {
    pub contract_version: u32,
    pub workspace_id: WorkspaceId,
    pub schema_version: u32,
    pub valid: bool,
    pub findings: Vec<ValidationFindingDto>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFindingDto {
    pub contract_version: u32,
    pub code: String,
    pub severity: FindingSeverityDto,
    pub path: String,
    pub message: String,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FindingSeverityDto {
    Info,
    Warning,
    Error,
}

/// Error contract shared by native, command-line, and local API adapters.
/// `code` is stable and suitable for branching; `message` is display text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApplicationError {
    pub contract_version: u32,
    pub code: String,
    pub message: String,
    pub recovery: String,
    pub details: BTreeMap<String, Value>,
    pub correlation_id: CommandId,
}

impl fmt::Display for ApplicationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.message.fmt(formatter)
    }
}

impl std::error::Error for ApplicationError {}

#[derive(Debug, Clone)]
struct SessionState {
    root: PathBuf,
    workspace_id: WorkspaceId,
}

/// Sole application composition root for the current local Markdown adapter.
#[derive(Debug)]
pub struct LiaisonApplication {
    vault: MarkdownVault,
    runtime: Arc<dyn RuntimePorts>,
    sessions: Mutex<HashMap<WorkspaceSessionId, SessionState>>,
}

impl LiaisonApplication {
    #[must_use]
    pub fn new() -> Self {
        Self::with_runtime(Arc::new(SystemRuntime))
    }

    #[must_use]
    pub fn with_runtime(runtime: Arc<dyn RuntimePorts>) -> Self {
        Self {
            vault: MarkdownVault::new(),
            runtime,
            sessions: Mutex::new(HashMap::new()),
        }
    }

    #[must_use]
    pub fn app_status(&self) -> CommandResult<AppStatusDto> {
        let command_id = self.runtime.next_command_id();
        self.complete(
            command_id,
            AppStatusDto {
                version: env!("CARGO_PKG_VERSION").to_owned(),
                product_state: "local-authoritative review build".to_owned(),
                authority_model: "canonical records stay in the selected local workspace"
                    .to_owned(),
                connection_state: "no connection configured".to_owned(),
                release_evidence: "not yet release-proven".to_owned(),
                canonical_storage: "Markdown/YAML and documented JSONL".to_owned(),
            },
        )
    }

    pub fn default_workspace_path(&self) -> Result<CommandResult<String>, ApplicationError> {
        let command_id = self.runtime.next_command_id();
        let path = self.runtime.documents_directory().map_err(|error| {
            application_error(
                "application.documents-directory-unavailable",
                "the Documents folder could not be located",
                "choose a local workspace folder with its full absolute path",
                details([("reason", Value::String(error.to_string()))]),
                command_id,
            )
        })?;
        Ok(self.complete(
            command_id,
            path.join("Liaison RM").to_string_lossy().into_owned(),
        ))
    }

    pub fn initialise_workspace(
        &self,
        command: InitialiseWorkspaceCommand,
    ) -> Result<CommandResult<WorkspaceOpenDto>, ApplicationError> {
        let command_id = self.runtime.next_command_id();
        let root = Self::absolute_workspace_path(&command.path, command_id)?;
        let manifest = InitialiseWorkspace::new(self.vault.clone())
            .execute(
                &root,
                self.runtime.next_workspace_id(),
                command.name,
                command.profile,
                command.build_profile,
                command.locale,
            )
            .map_err(|error| initialise_workspace_error(error, command_id))?;
        let value = self
            .opened_workspace(&root, manifest, command_id)
            .map_err(|error| workspace_initialised_open_error(error, command_id))?;
        Ok(self.complete(command_id, value))
    }

    pub fn open_workspace(
        &self,
        command: OpenWorkspaceCommand,
    ) -> Result<CommandResult<WorkspaceOpenDto>, ApplicationError> {
        let command_id = self.runtime.next_command_id();
        let OpenWorkspaceCommand { path } = command;
        let root = Self::absolute_workspace_path(&path, command_id)?;
        let manifest = self
            .vault
            .load(&root)
            .map_err(|error| workspace_error(error, command_id))?;
        let value = self.opened_workspace(&root, manifest, command_id)?;
        Ok(self.complete(command_id, value))
    }

    pub fn validate_workspace(
        &self,
        command: WorkspaceSessionCommand,
    ) -> Result<CommandResult<WorkspaceValidationDto>, ApplicationError> {
        let command_id = self.runtime.next_command_id();
        let session = self.resolve_session(command.session_id, command_id)?;
        let report = ValidateWorkspace::new(self.vault.clone())
            .execute(&session.root)
            .map_err(|error| workspace_error(error, command_id))?;
        Ok(self.complete(command_id, validation_dto(report)))
    }

    pub fn create_person(
        &self,
        command: CreatePersonCommand,
    ) -> Result<CommandResult<PersonDto>, ApplicationError> {
        let command_id = self.runtime.next_command_id();
        let session = self.resolve_session(command.session_id, command_id)?;
        let email = command.email.and_then(|value| {
            let value = value.trim().to_owned();
            (!value.is_empty()).then_some(value)
        });
        let person = CreatePerson::new(self.vault.clone())
            .execute(
                &session.root,
                self.runtime.next_person_id(),
                command.display_name,
                email,
            )
            .map_err(|error| people_error(&error, command_id))?;
        Ok(self.complete(command_id, person_dto(person)))
    }

    pub fn list_people(
        &self,
        query: ListPeopleQuery,
    ) -> Result<CommandResult<Vec<PersonDto>>, ApplicationError> {
        let command_id = self.runtime.next_command_id();
        let session = self.resolve_session(query.session_id, command_id)?;
        let people = ListPeople::new(self.vault.clone())
            .execute(&session.root, query.include_archived)
            .map_err(|error| people_error(&error, command_id))?
            .into_iter()
            .map(person_dto)
            .collect();
        Ok(self.complete(command_id, people))
    }

    fn opened_workspace(
        &self,
        root: &Path,
        manifest: WorkspaceManifest,
        command_id: CommandId,
    ) -> Result<WorkspaceOpenDto, ApplicationError> {
        let report = ValidateWorkspace::new(self.vault.clone())
            .execute(root)
            .map_err(|error| workspace_error(error, command_id))?;
        let people = ListPeople::new(self.vault.clone())
            .execute(root, false)
            .map_err(|error| people_error(&error, command_id))?
            .into_iter()
            .map(person_dto)
            .collect();
        let session_id = self.runtime.next_workspace_session_id();
        let mut sessions = self.sessions.lock().map_err(|_| {
            application_error(
                "application.session-state-unavailable",
                "workspace session state is unavailable",
                "close Liaison RM, reopen it, and open the workspace again",
                BTreeMap::new(),
                command_id,
            )
        })?;
        if sessions.contains_key(&session_id) {
            return Err(application_error(
                "application.identifier-collision",
                "a workspace session identifier was generated more than once",
                "retry the operation; if it repeats, preserve the workspace and report the build",
                details([("identifier", Value::String(session_id.to_string()))]),
                command_id,
            ));
        }
        sessions.insert(
            session_id,
            SessionState {
                root: root.to_path_buf(),
                workspace_id: manifest.workspace_id,
            },
        );
        Ok(WorkspaceOpenDto {
            workspace: WorkspaceDto {
                session_id,
                path: root.to_string_lossy().into_owned(),
                workspace_id: manifest.workspace_id,
                schema_version: manifest.schema_version,
                name: manifest.name,
                profile: manifest.profile,
                build_profile: manifest.build_profile,
                locale: manifest.default_locale,
            },
            people,
            validation: validation_dto(report),
        })
    }

    fn resolve_session(
        &self,
        session_id: WorkspaceSessionId,
        command_id: CommandId,
    ) -> Result<SessionState, ApplicationError> {
        let session = self
            .sessions
            .lock()
            .map_err(|_| {
                application_error(
                    "application.session-state-unavailable",
                    "workspace session state is unavailable",
                    "close Liaison RM, reopen it, and open the workspace again",
                    BTreeMap::new(),
                    command_id,
                )
            })?
            .get(&session_id)
            .cloned()
            .ok_or_else(|| {
                application_error(
                    "application.workspace-session-not-found",
                    "the workspace session is not open",
                    "open the workspace again and retry the operation",
                    details([("session_id", Value::String(session_id.to_string()))]),
                    command_id,
                )
            })?;
        let manifest = self
            .vault
            .load(&session.root)
            .map_err(|error| workspace_error(error, command_id))?;
        if manifest.workspace_id != session.workspace_id {
            return Err(application_error(
                "application.workspace-session-stale",
                "the workspace identity changed after this session was opened",
                "stop editing and open the intended workspace again",
                details([
                    (
                        "expected_workspace_id",
                        Value::String(session.workspace_id.to_string()),
                    ),
                    (
                        "found_workspace_id",
                        Value::String(manifest.workspace_id.to_string()),
                    ),
                ]),
                command_id,
            ));
        }
        Ok(session)
    }

    fn absolute_workspace_path(
        value: &str,
        command_id: CommandId,
    ) -> Result<PathBuf, ApplicationError> {
        let value = value.trim();
        if value.is_empty() {
            return Err(application_error(
                "application.workspace-path-required",
                "workspace path is required",
                "choose a local workspace folder and retry",
                BTreeMap::new(),
                command_id,
            ));
        }
        let path = PathBuf::from(value);
        if !path.is_absolute() {
            return Err(application_error(
                "application.workspace-path-not-absolute",
                "workspace path must be absolute",
                "choose the full local folder path and retry",
                BTreeMap::new(),
                command_id,
            ));
        }
        if path.parent().is_none() {
            return Err(application_error(
                "application.workspace-path-root",
                "workspace path cannot be a filesystem or volume root",
                "choose a dedicated new or empty folder inside a local user directory",
                BTreeMap::new(),
                command_id,
            ));
        }
        if path
            .components()
            .any(|component| matches!(component, Component::CurDir | Component::ParentDir))
        {
            return Err(application_error(
                "application.workspace-path-unsafe-alias",
                "workspace path cannot contain dot or parent aliases",
                "choose the full direct path to a dedicated new or empty folder",
                BTreeMap::new(),
                command_id,
            ));
        }
        Ok(path)
    }

    fn complete<T>(&self, command_id: CommandId, value: T) -> CommandResult<T> {
        CommandResult {
            contract_version: APPLICATION_CONTRACT_VERSION,
            command_id,
            completed_at: self.runtime.now(),
            value,
        }
    }
}

impl Default for LiaisonApplication {
    fn default() -> Self {
        Self::new()
    }
}

fn validation_dto(report: liaison_workspace::WorkspaceValidationReport) -> WorkspaceValidationDto {
    WorkspaceValidationDto {
        contract_version: APPLICATION_CONTRACT_VERSION,
        workspace_id: report.workspace_id,
        schema_version: report.schema_version,
        valid: report.is_valid(),
        findings: report.findings.into_iter().map(finding_dto).collect(),
    }
}

fn finding_dto(finding: ValidationFinding) -> ValidationFindingDto {
    ValidationFindingDto {
        contract_version: APPLICATION_CONTRACT_VERSION,
        code: finding.code,
        severity: match finding.severity {
            FindingSeverity::Info => FindingSeverityDto::Info,
            FindingSeverity::Warning => FindingSeverityDto::Warning,
            FindingSeverity::Error => FindingSeverityDto::Error,
        },
        path: finding.path,
        message: finding.message,
        recovery: finding.recovery,
    }
}

fn person_dto(person: PersonProfile) -> PersonDto {
    PersonDto {
        id: person.id,
        revision: person.revision,
        display_name: person.display_name,
        aliases: person.aliases,
        emails: person.emails.into_iter().map(email_dto).collect(),
        phones: person.phones.into_iter().map(phone_dto).collect(),
        birthday: person.birthday.as_ref().map(partial_date_dto),
        archived: person.archived,
    }
}

fn email_dto(email: EmailAddress) -> EmailDto {
    EmailDto {
        value: email.value,
        label: email.label,
    }
}

fn phone_dto(phone: PhoneNumber) -> PhoneDto {
    PhoneDto {
        value: phone.value,
        label: phone.label,
    }
}

fn partial_date_dto(date: &PartialDate) -> PartialDateDto {
    match date {
        PartialDate::Full { date } => PartialDateDto::Full { date: *date },
        PartialDate::MonthDay { month, day } => PartialDateDto::MonthDay {
            month: *month,
            day: *day,
        },
    }
}

fn workspace_error(error: WorkspaceError, correlation_id: CommandId) -> ApplicationError {
    let message = error.to_string();
    match error {
        WorkspaceError::RequiredField(field) => application_error(
            "workspace.required-field",
            message,
            "provide the required workspace value and retry",
            details([("field", Value::String((*field).to_owned()))]),
            correlation_id,
        ),
        WorkspaceError::UnexpectedFormat(found) => application_error(
            "workspace.unexpected-format",
            message,
            "choose a Liaison workspace or restore a verified copy of its manifest",
            details([("found", Value::String(found))]),
            correlation_id,
        ),
        WorkspaceError::UnsupportedSchema { found, supported } => application_error(
            "workspace.unsupported-schema",
            message,
            "open the workspace with a compatible Liaison build; do not rewrite it manually",
            details([
                ("found", Value::from(found)),
                ("supported", Value::from(supported)),
            ]),
            correlation_id,
        ),
        WorkspaceError::AlreadyExists => application_error(
            "workspace.already-exists",
            message,
            "open the existing workspace or choose a new empty folder",
            BTreeMap::new(),
            correlation_id,
        ),
        WorkspaceError::InitialiseTargetNotEmpty => application_error(
            "workspace.initialise-target-not-empty",
            "workspace initialisation requires a new or empty directory",
            "choose a new empty folder; Liaison will not mix workspace files with existing content",
            BTreeMap::new(),
            correlation_id,
        ),
        WorkspaceError::NotFound => application_error(
            "workspace.not-found",
            message,
            "choose an existing Liaison workspace or initialise a new one",
            BTreeMap::new(),
            correlation_id,
        ),
        WorkspaceError::Storage(_) => application_error(
            "workspace.storage-error",
            "the workspace storage operation failed",
            "stop editing, preserve the workspace, and inspect Health before retrying",
            BTreeMap::new(),
            correlation_id,
        ),
    }
}

fn people_error(error: &PeopleError, correlation_id: CommandId) -> ApplicationError {
    let message = error.to_string();
    match error {
        PeopleError::RequiredField(field) => application_error(
            "people.required-field",
            message,
            "provide the required Person value and retry",
            details([("field", Value::String((*field).to_owned()))]),
            correlation_id,
        ),
        PeopleError::InvalidEmail(_) => application_error(
            "people.invalid-email",
            "email address is invalid",
            "correct the email address or leave it empty",
            BTreeMap::new(),
            correlation_id,
        ),
        PeopleError::InvalidPhone(_) => application_error(
            "people.invalid-phone",
            "phone number is invalid",
            "correct the phone number or leave it empty",
            BTreeMap::new(),
            correlation_id,
        ),
        PeopleError::InvalidPartialDate { month, day } => application_error(
            "people.invalid-partial-date",
            message,
            "choose a valid month and day",
            details([("month", Value::from(*month)), ("day", Value::from(*day))]),
            correlation_id,
        ),
        PeopleError::AlreadyExists => application_error(
            "people.already-exists",
            message,
            "refresh the directory and edit the existing Person instead",
            BTreeMap::new(),
            correlation_id,
        ),
        PeopleError::NotFound => application_error(
            "people.not-found",
            message,
            "refresh the directory and choose an existing Person",
            BTreeMap::new(),
            correlation_id,
        ),
        PeopleError::RevisionConflict { expected, found } => application_error(
            "people.revision-conflict",
            message,
            "reload the Person, review the external change, and retry deliberately",
            details([
                ("expected", Value::from(*expected)),
                ("found", Value::from(*found)),
            ]),
            correlation_id,
        ),
        PeopleError::RevisionOverflow => application_error(
            "people.revision-overflow",
            message,
            "stop editing and preserve the workspace for repair",
            BTreeMap::new(),
            correlation_id,
        ),
        PeopleError::Storage(_) => application_error(
            "people.storage-error",
            "the Person storage operation failed",
            "preserve the affected file and inspect Workspace Health before retrying",
            BTreeMap::new(),
            correlation_id,
        ),
    }
}

fn initialise_workspace_error(
    error: WorkspaceError,
    correlation_id: CommandId,
) -> ApplicationError {
    if matches!(&error, WorkspaceError::Storage(_)) {
        return application_error(
            "workspace.initialise-incomplete",
            "workspace initialisation did not complete",
            "do not add records or retry in place; inspect the selected folder, preserve any existing files, and choose a verified-empty folder",
            BTreeMap::new(),
            correlation_id,
        );
    }
    workspace_error(error, correlation_id)
}

fn workspace_initialised_open_error(
    error: ApplicationError,
    correlation_id: CommandId,
) -> ApplicationError {
    application_error(
        "application.workspace-initialised-open-incomplete",
        "the workspace was initialised but its session could not be opened",
        "do not initialise it again; open the existing workspace and inspect Health",
        details([("cause_code", Value::String(error.code))]),
        correlation_id,
    )
}

fn application_error(
    code: impl Into<String>,
    message: impl Into<String>,
    recovery: impl Into<String>,
    details: BTreeMap<String, Value>,
    correlation_id: CommandId,
) -> ApplicationError {
    ApplicationError {
        contract_version: APPLICATION_CONTRACT_VERSION,
        code: code.into(),
        message: message.into(),
        recovery: recovery.into(),
        details,
        correlation_id,
    }
}

fn details<const N: usize>(entries: [(&str, Value); N]) -> BTreeMap<String, Value> {
    entries
        .into_iter()
        .map(|(key, value)| (key.to_owned(), value))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        APPLICATION_CONTRACT_VERSION, AppStatusDto, CreatePersonCommand,
        InitialiseWorkspaceCommand, LiaisonApplication, ListPeopleQuery, OpenWorkspaceCommand,
        RuntimePortError, RuntimePorts, WorkspaceSessionCommand,
    };
    use chrono::{DateTime, TimeZone, Utc};
    use liaison_shared_kernel::{CommandId, JobId, PersonId, WorkspaceId, WorkspaceSessionId};
    use liaison_workspace::{BuildProfile, WorkspaceProfile};
    use std::{
        fs,
        path::PathBuf,
        sync::{
            Arc,
            atomic::{AtomicU64, Ordering},
        },
    };
    use tempfile::tempdir;
    use uuid::Uuid;

    #[derive(Debug)]
    struct FakeRuntime {
        now: DateTime<Utc>,
        commands: AtomicU64,
        workspaces: AtomicU64,
        people: AtomicU64,
        sessions: AtomicU64,
        jobs: AtomicU64,
        random_byte: u8,
        reuse_session: bool,
    }

    impl FakeRuntime {
        fn new() -> Self {
            let now = Utc
                .with_ymd_and_hms(2026, 7, 18, 12, 0, 0)
                .single()
                .unwrap_or_else(|| unreachable!("fixed test time is valid"));
            Self {
                now,
                commands: AtomicU64::new(1),
                workspaces: AtomicU64::new(301),
                people: AtomicU64::new(401),
                sessions: AtomicU64::new(101),
                jobs: AtomicU64::new(201),
                random_byte: 0xA5,
                reuse_session: false,
            }
        }

        fn with_reused_session() -> Self {
            Self {
                reuse_session: true,
                ..Self::new()
            }
        }

        fn next_uuid(counter: &AtomicU64) -> Uuid {
            Uuid::from_u128(u128::from(counter.fetch_add(1, Ordering::SeqCst)))
        }
    }

    impl RuntimePorts for FakeRuntime {
        fn now(&self) -> DateTime<Utc> {
            self.now
        }

        fn next_command_id(&self) -> CommandId {
            CommandId::from_uuid(Self::next_uuid(&self.commands))
        }

        fn next_workspace_id(&self) -> WorkspaceId {
            WorkspaceId::from_uuid(Self::next_uuid(&self.workspaces))
        }

        fn next_person_id(&self) -> PersonId {
            PersonId::from_uuid(Self::next_uuid(&self.people))
        }

        fn next_workspace_session_id(&self) -> WorkspaceSessionId {
            if self.reuse_session {
                WorkspaceSessionId::from_uuid(Uuid::from_u128(101))
            } else {
                WorkspaceSessionId::from_uuid(Self::next_uuid(&self.sessions))
            }
        }

        fn next_job_id(&self) -> JobId {
            JobId::from_uuid(Self::next_uuid(&self.jobs))
        }

        fn random_bytes(&self, length: usize) -> Result<Vec<u8>, RuntimePortError> {
            Ok(vec![self.random_byte; length])
        }

        fn documents_directory(&self) -> Result<std::path::PathBuf, RuntimePortError> {
            Ok(std::path::PathBuf::from("/Users/example/Documents"))
        }
    }

    fn application() -> (LiaisonApplication, Arc<FakeRuntime>) {
        let runtime = Arc::new(FakeRuntime::new());
        (LiaisonApplication::with_runtime(runtime.clone()), runtime)
    }

    #[test]
    fn status_uses_honest_review_build_claims() {
        let (application, _) = application();
        let result = application.app_status();
        assert_eq!(
            result.value,
            AppStatusDto {
                version: "0.1.0-alpha.1".to_owned(),
                product_state: "local-authoritative review build".to_owned(),
                authority_model: "canonical records stay in the selected local workspace"
                    .to_owned(),
                connection_state: "no connection configured".to_owned(),
                release_evidence: "not yet release-proven".to_owned(),
                canonical_storage: "Markdown/YAML and documented JSONL".to_owned(),
            }
        );
        assert_eq!(result.contract_version, APPLICATION_CONTRACT_VERSION);
        assert_eq!(result.command_id.as_uuid(), Uuid::from_u128(1));
    }

    #[test]
    fn default_workspace_path_uses_the_runtime_documents_directory() {
        let (application, _) = application();
        let result = application.default_workspace_path();
        assert!(result.is_ok());
        if let Ok(result) = result {
            let expected = PathBuf::from("/Users/example/Documents")
                .join("Liaison RM")
                .to_string_lossy()
                .into_owned();
            assert_eq!(result.value, expected);
            assert_eq!(result.command_id.as_uuid(), Uuid::from_u128(1));
        }
    }

    #[test]
    fn commands_share_one_open_workspace_session() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path().join("workspace");
            let (application, _) = application();
            let opened = application.initialise_workspace(InitialiseWorkspaceCommand {
                path: root.to_string_lossy().into_owned(),
                name: "People".to_owned(),
                profile: WorkspaceProfile::Personal,
                build_profile: BuildProfile::Airgap,
                locale: "en-IE".to_owned(),
            });
            assert!(opened.is_ok());
            if let Ok(opened) = opened {
                let session_id = opened.value.workspace.session_id;
                assert_eq!(session_id.as_uuid(), Uuid::from_u128(101));
                assert_eq!(
                    opened.value.workspace.workspace_id.as_uuid(),
                    Uuid::from_u128(301)
                );
                assert!(opened.value.validation.valid);

                let created = application.create_person(CreatePersonCommand {
                    session_id,
                    display_name: "Alex Murphy".to_owned(),
                    email: Some("alex@example.test".to_owned()),
                });
                assert!(created.is_ok());
                if let Ok(created) = created {
                    assert_eq!(created.value.id.as_uuid(), Uuid::from_u128(401));
                    assert_eq!(created.value.revision.get(), 1);
                    assert_eq!(created.value.emails.len(), 1);
                }

                let listed = application.list_people(ListPeopleQuery {
                    session_id,
                    include_archived: false,
                });
                assert!(listed.is_ok());
                if let Ok(listed) = listed {
                    assert_eq!(listed.value.len(), 1);
                    assert_eq!(listed.value[0].display_name, "Alex Murphy");
                }

                let validated =
                    application.validate_workspace(WorkspaceSessionCommand { session_id });
                assert!(validated.is_ok());
                if let Ok(validated) = validated {
                    assert!(validated.value.valid);
                }
            }
        }
    }

    #[test]
    fn open_keeps_healthy_people_visible_when_a_sibling_is_malformed() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let root = directory.path().join("workspace");
            let (application, _) = application();
            let initial = application.initialise_workspace(InitialiseWorkspaceCommand {
                path: root.to_string_lossy().into_owned(),
                name: "Workplace".to_owned(),
                profile: WorkspaceProfile::Workplace,
                build_profile: BuildProfile::ConnectedLocal,
                locale: "en-IE".to_owned(),
            });
            assert!(initial.is_ok());
            if let Ok(initial) = initial {
                let created = application.create_person(CreatePersonCommand {
                    session_id: initial.value.workspace.session_id,
                    display_name: "Healthy Person".to_owned(),
                    email: None,
                });
                assert!(created.is_ok());
            }
            assert!(fs::write(root.join("people/broken.md"), "not front matter\n").is_ok());

            let reopened = application.open_workspace(OpenWorkspaceCommand {
                path: root.to_string_lossy().into_owned(),
            });
            assert!(reopened.is_ok());
            if let Ok(reopened) = reopened {
                assert_eq!(reopened.value.people.len(), 1);
                assert!(!reopened.value.validation.valid);
                assert!(reopened.value.validation.findings.iter().any(|finding| {
                    finding.code == "people.invalid-record"
                        && finding.severity == super::FindingSeverityDto::Error
                }));
            }
        }
    }

    #[test]
    fn errors_are_structured_and_correlated_to_the_command() {
        let (application, _) = application();
        let result = application.open_workspace(OpenWorkspaceCommand {
            path: "relative/workspace".to_owned(),
        });
        assert!(result.is_err());
        if let Err(error) = result {
            assert_eq!(error.contract_version, APPLICATION_CONTRACT_VERSION);
            assert_eq!(error.code, "application.workspace-path-not-absolute");
            assert_eq!(error.correlation_id.as_uuid(), Uuid::from_u128(1));
            assert!(error.recovery.contains("full local folder path"));
            assert!(error.details.is_empty());
            let serialized = serde_json::to_value(&error);
            assert!(serialized.is_ok());
            if let Ok(serialized) = serialized {
                assert_eq!(
                    serialized
                        .get("contract_version")
                        .and_then(serde_json::Value::as_u64),
                    Some(u64::from(APPLICATION_CONTRACT_VERSION))
                );
                assert_eq!(
                    serialized.get("code").and_then(serde_json::Value::as_str),
                    Some("application.workspace-path-not-absolute")
                );
                assert_eq!(
                    serialized
                        .get("correlation_id")
                        .and_then(serde_json::Value::as_str),
                    Some("00000000-0000-0000-0000-000000000001")
                );
            }
        }
    }

    #[test]
    fn workspace_paths_reject_roots_and_dot_aliases_without_echoing_input() {
        let (application, _) = application();
        let current = std::env::current_dir();
        assert!(current.is_ok());
        let Ok(current) = current else {
            return;
        };
        let root = current.ancestors().last();
        assert!(root.is_some());
        let Some(root) = root.map(std::path::Path::to_path_buf) else {
            return;
        };
        let aliased = root.join("tmp").join("..").join("private");
        for (path, expected_code) in [
            (
                root.to_string_lossy().into_owned(),
                "application.workspace-path-root",
            ),
            (
                aliased.to_string_lossy().into_owned(),
                "application.workspace-path-unsafe-alias",
            ),
        ] {
            let result = application.open_workspace(OpenWorkspaceCommand { path: path.clone() });
            assert!(result.is_err());
            let Err(error) = result else {
                continue;
            };
            assert_eq!(error.code, expected_code);
            assert!(error.details.is_empty());
            assert!(!error.message.contains(&path));
            assert!(!error.recovery.contains(&path));
        }
    }

    #[test]
    fn initialised_workspace_reports_an_honest_open_incomplete_error_on_session_collision() {
        let directory = tempdir();
        assert!(directory.is_ok());
        if let Ok(directory) = directory {
            let runtime = Arc::new(FakeRuntime::with_reused_session());
            let application = LiaisonApplication::with_runtime(runtime);
            let first = directory.path().join("first");
            let second = directory.path().join("second");
            let first_result = application.initialise_workspace(InitialiseWorkspaceCommand {
                path: first.to_string_lossy().into_owned(),
                name: "First".to_owned(),
                profile: WorkspaceProfile::Personal,
                build_profile: BuildProfile::ConnectedLocal,
                locale: "en-IE".to_owned(),
            });
            assert!(first_result.is_ok());

            let result = application.initialise_workspace(InitialiseWorkspaceCommand {
                path: second.to_string_lossy().into_owned(),
                name: "Second".to_owned(),
                profile: WorkspaceProfile::Workplace,
                build_profile: BuildProfile::ConnectedLocal,
                locale: "en-IE".to_owned(),
            });
            assert!(result.is_err());
            let Err(error) = result else {
                return;
            };
            assert_eq!(
                error.code,
                "application.workspace-initialised-open-incomplete"
            );
            assert_eq!(
                error
                    .details
                    .get("cause_code")
                    .and_then(serde_json::Value::as_str),
                Some("application.identifier-collision")
            );
            assert!(second.join(".liaison/workspace.yaml").is_file());
            assert!(error.recovery.contains("do not initialise it again"));
        }
    }

    #[test]
    fn fake_runtime_controls_job_ids_and_random_bytes() {
        let runtime = FakeRuntime::new();
        assert_eq!(runtime.next_job_id().as_uuid(), Uuid::from_u128(201));
        assert_eq!(runtime.random_bytes(4), Ok(vec![0xA5; 4]));
    }
}
