//! Native desktop interface for Liaison RM.
//!
//! Tauri commands are inbound adapters over the Workspace and People
//! application services. They do not define canonical formats or domain rules.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use liaison_people::{CreatePerson, ListPeople, PersonProfile};
use liaison_vault_markdown::MarkdownVault;
use liaison_workspace::{
    BuildProfile, InitialiseWorkspace, ValidateWorkspace, WorkspaceManifest, WorkspaceProfile,
    WorkspaceStore,
};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const DEFAULT_LOCALE: &str = "en-IE";

#[derive(Clone, Debug, Serialize)]
struct AppStatus {
    version: &'static str,
    local_authority: bool,
    network_clients_compiled: bool,
    canonical_storage: &'static str,
}

#[derive(Clone, Debug, Deserialize)]
struct InitialiseWorkspaceRequest {
    path: String,
    name: String,
    profile: String,
}

#[derive(Clone, Debug, Deserialize)]
struct CreatePersonRequest {
    workspace_path: String,
    display_name: String,
    email: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
struct WorkspaceView {
    path: String,
    workspace_id: String,
    name: String,
    profile: &'static str,
    build_profile: &'static str,
    locale: String,
    people_count: usize,
}

#[derive(Clone, Debug, Serialize)]
struct PersonView {
    id: String,
    display_name: String,
    primary_email: Option<String>,
    revision: u64,
}

#[derive(Clone, Debug, Serialize)]
struct ValidationView {
    valid: bool,
    schema_version: u32,
    finding_count: usize,
    findings: Vec<liaison_workspace::ValidationFinding>,
}

#[tauri::command]
fn app_status() -> AppStatus {
    AppStatus {
        version: env!("CARGO_PKG_VERSION"),
        local_authority: true,
        network_clients_compiled: false,
        canonical_storage: "Markdown/YAML and documented JSONL",
    }
}

#[tauri::command]
fn default_workspace_path() -> String {
    default_workspace_path_impl().to_string_lossy().into_owned()
}

#[tauri::command]
fn initialise_workspace(request: InitialiseWorkspaceRequest) -> Result<WorkspaceView, String> {
    initialise_workspace_impl(request)
}

#[tauri::command]
fn open_workspace(path: String) -> Result<WorkspaceView, String> {
    open_workspace_impl(&path)
}

#[tauri::command]
fn list_people(workspace_path: String) -> Result<Vec<PersonView>, String> {
    list_people_impl(&workspace_path)
}

#[tauri::command]
fn create_person(request: CreatePersonRequest) -> Result<PersonView, String> {
    create_person_impl(request)
}

#[tauri::command]
fn validate_workspace(path: String) -> Result<ValidationView, String> {
    validate_workspace_impl(&path)
}

fn default_workspace_path_impl() -> PathBuf {
    let home = std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    home.join("Documents").join("Liaison RM")
}

fn initialise_workspace_impl(request: InitialiseWorkspaceRequest) -> Result<WorkspaceView, String> {
    let root = absolute_path(&request.path)?;
    let profile = parse_profile(&request.profile)?;
    let vault = MarkdownVault::new();
    let manifest = InitialiseWorkspace::new(vault.clone())
        .execute(
            &root,
            request.name,
            profile,
            BuildProfile::Airgap,
            DEFAULT_LOCALE,
        )
        .map_err(|error| error.to_string())?;
    workspace_view(&root, manifest, &vault)
}

fn open_workspace_impl(path: &str) -> Result<WorkspaceView, String> {
    let root = absolute_path(path)?;
    let vault = MarkdownVault::new();
    let manifest = vault.load(&root).map_err(|error| error.to_string())?;
    workspace_view(&root, manifest, &vault)
}

fn workspace_view(
    root: &Path,
    manifest: WorkspaceManifest,
    vault: &MarkdownVault,
) -> Result<WorkspaceView, String> {
    let people_count = ListPeople::new(vault.clone())
        .execute(root, false)
        .map_err(|error| error.to_string())?
        .len();
    Ok(WorkspaceView {
        path: root.to_string_lossy().into_owned(),
        workspace_id: manifest.workspace_id.to_string(),
        name: manifest.name,
        profile: profile_name(manifest.profile),
        build_profile: build_profile_name(manifest.build_profile),
        locale: manifest.default_locale,
        people_count,
    })
}

fn list_people_impl(workspace_path: &str) -> Result<Vec<PersonView>, String> {
    let root = absolute_path(workspace_path)?;
    ListPeople::new(MarkdownVault::new())
        .execute(&root, false)
        .map_err(|error| error.to_string())
        .map(|people| people.into_iter().map(person_view).collect())
}

fn create_person_impl(request: CreatePersonRequest) -> Result<PersonView, String> {
    let root = absolute_path(&request.workspace_path)?;
    let email = request.email.and_then(|value| {
        let value = value.trim().to_owned();
        (!value.is_empty()).then_some(value)
    });
    CreatePerson::new(MarkdownVault::new())
        .execute(&root, request.display_name, email)
        .map(person_view)
        .map_err(|error| error.to_string())
}

fn validate_workspace_impl(path: &str) -> Result<ValidationView, String> {
    let root = absolute_path(path)?;
    let report = ValidateWorkspace::new(MarkdownVault::new())
        .execute(&root)
        .map_err(|error| error.to_string())?;
    Ok(ValidationView {
        valid: report.is_valid(),
        schema_version: report.schema_version,
        finding_count: report.findings.len(),
        findings: report.findings,
    })
}

fn person_view(person: PersonProfile) -> PersonView {
    let primary_email = person.emails.first().map(|email| email.value.clone());
    PersonView {
        id: person.id.to_string(),
        display_name: person.display_name,
        primary_email,
        revision: person.revision.get(),
    }
}

fn absolute_path(value: &str) -> Result<PathBuf, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err("workspace path is required".to_owned());
    }
    let path = PathBuf::from(value);
    if !path.is_absolute() {
        return Err("workspace path must be absolute".to_owned());
    }
    Ok(path)
}

fn parse_profile(value: &str) -> Result<WorkspaceProfile, String> {
    match value {
        "personal" => Ok(WorkspaceProfile::Personal),
        "family" => Ok(WorkspaceProfile::Family),
        "team" => Ok(WorkspaceProfile::Team),
        "workplace" => Ok(WorkspaceProfile::Workplace),
        _ => Err(format!("unsupported workspace profile: {value}")),
    }
}

const fn profile_name(profile: WorkspaceProfile) -> &'static str {
    match profile {
        WorkspaceProfile::Personal => "personal",
        WorkspaceProfile::Family => "family",
        WorkspaceProfile::Team => "team",
        WorkspaceProfile::Workplace => "workplace",
    }
}

const fn build_profile_name(profile: BuildProfile) -> &'static str {
    match profile {
        BuildProfile::Airgap => "airgap",
        BuildProfile::ConnectedLocal => "connected-local",
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let result = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            app_status,
            default_workspace_path,
            initialise_workspace,
            open_workspace,
            list_people,
            create_person,
            validate_workspace
        ])
        .run(tauri::generate_context!());
    if let Err(error) = result {
        eprintln!("Liaison RM could not start: {error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CreatePersonRequest, InitialiseWorkspaceRequest, create_person_impl,
        initialise_workspace_impl, list_people_impl, validate_workspace_impl,
    };
    use tempfile::tempdir;

    #[test]
    fn local_vertical_slice_creates_readable_people() {
        let directory = tempdir();
        assert!(directory.is_ok());
        let Ok(directory) = directory else {
            return;
        };
        let path = directory.path().join("workspace");
        let path_text = path.to_string_lossy().into_owned();
        let workspace = initialise_workspace_impl(InitialiseWorkspaceRequest {
            path: path_text.clone(),
            name: "Personal relationships".to_owned(),
            profile: "personal".to_owned(),
        });
        assert!(workspace.is_ok());

        let person = create_person_impl(CreatePersonRequest {
            workspace_path: path_text.clone(),
            display_name: "Alex Murphy".to_owned(),
            email: Some("alex@example.test".to_owned()),
        });
        assert!(person.is_ok());

        let people = list_people_impl(&path_text);
        assert!(people.is_ok());
        let Ok(people) = people else {
            return;
        };
        assert_eq!(people.len(), 1);
        assert_eq!(people[0].display_name, "Alex Murphy");
        assert_eq!(
            people[0].primary_email.as_deref(),
            Some("alex@example.test")
        );

        let validation = validate_workspace_impl(&path_text);
        assert!(validation.is_ok());
        let Ok(validation) = validation else {
            return;
        };
        assert!(validation.valid);
        assert_eq!(validation.finding_count, 0);
    }

    #[test]
    fn relative_workspace_path_is_rejected() {
        let result = initialise_workspace_impl(InitialiseWorkspaceRequest {
            path: "relative/workspace".to_owned(),
            name: "People".to_owned(),
            profile: "personal".to_owned(),
        });
        assert_eq!(
            result.map(|workspace| workspace.path),
            Err("workspace path must be absolute".to_owned())
        );
    }
}
