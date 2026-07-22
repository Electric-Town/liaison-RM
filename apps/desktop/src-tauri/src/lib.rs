//! Native desktop interface for Liaison RM.
//!
//! Tauri commands are inbound adapters over the single application
//! composition root. They translate desktop request shapes but do not
//! construct bounded-context services, repositories, or storage adapters.

#![allow(
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value // Tauri commands own their State extractors.
)]

use liaison_application::{
    AddEventAttendeeCommand, AppStatusDto, ApplicationError, ArchivePersonCommand, BuildProfile,
    CommandResult, CreateEventCommand, CreatePersonCommand, EmailDto, EventDto, EventId,
    FinalizeEventCohortCommand, InitialiseWorkspaceCommand, InspectWorkspaceHealthQuery,
    LiaisonApplication, ListEventsQuery, ListPeopleQuery, NaiveDate, OpenWorkspaceCommand,
    PersonDto, PhoneDto, ResolveAttendeeGapCommand, UpdatePersonCommand, WorkspaceClosedDto,
    WorkspaceOpenDto, WorkspaceProfile, WorkspaceSessionCommand, WorkspaceSessionId,
    WorkspaceValidationDto,
};
use serde::Deserialize;
use tauri::State;

const DEFAULT_LOCALE: &str = "en-IE";

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InitialiseWorkspaceRequest {
    path: String,
    name: String,
    profile: WorkspaceProfile,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreatePersonRequest {
    session_id: WorkspaceSessionId,
    display_name: String,
    email: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct UpdatePersonRequest {
    session_id: WorkspaceSessionId,
    person_id: liaison_application::PersonId,
    expected_revision: liaison_application::Revision,
    display_name: String,
    emails: Vec<EmailDto>,
    phones: Vec<PhoneDto>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ArchivePersonRequest {
    session_id: WorkspaceSessionId,
    person_id: liaison_application::PersonId,
    expected_revision: liaison_application::Revision,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct CreateEventRequest {
    session_id: WorkspaceSessionId,
    name: String,
    date: NaiveDate,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::struct_field_names, dead_code)]
struct AddEventAttendeeRequest {
    session_id: WorkspaceSessionId,
    event_id: EventId,
    person_id: liaison_application::PersonId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct ResolveAttendeeGapRequest {
    session_id: WorkspaceSessionId,
    event_id: EventId,
    row_id: u32,
    action: String,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
struct FinalizeEventCohortRequest {
    session_id: WorkspaceSessionId,
    event_id: EventId,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WorkspaceSessionRequest {
    session_id: WorkspaceSessionId,
}

#[tauri::command]
fn app_status(application: State<'_, LiaisonApplication>) -> CommandResult<AppStatusDto> {
    application.app_status()
}

#[tauri::command]
fn default_workspace_path(
    application: State<'_, LiaisonApplication>,
) -> Result<CommandResult<String>, ApplicationError> {
    application.default_workspace_path()
}

#[tauri::command]
fn initialise_workspace(
    application: State<'_, LiaisonApplication>,
    request: InitialiseWorkspaceRequest,
) -> Result<CommandResult<WorkspaceOpenDto>, ApplicationError> {
    initialise_workspace_impl(&application, request)
}

#[tauri::command]
fn open_workspace(
    application: State<'_, LiaisonApplication>,
    path: String,
) -> Result<CommandResult<WorkspaceOpenDto>, ApplicationError> {
    open_workspace_impl(&application, path)
}

#[tauri::command]
fn list_people(
    application: State<'_, LiaisonApplication>,
    request: WorkspaceSessionRequest,
) -> Result<CommandResult<Vec<PersonDto>>, ApplicationError> {
    list_people_impl(&application, request)
}

#[tauri::command]
fn create_person(
    application: State<'_, LiaisonApplication>,
    request: CreatePersonRequest,
) -> Result<CommandResult<PersonDto>, ApplicationError> {
    create_person_impl(&application, request)
}

// Retained as a P05/P09 incubation boundary. Editing and archiving stay out of
// the production invoke handler until the revisioned Person contract,
// reversible archive/restore workflow, and Directory maintenance gates close.
#[tauri::command]
#[allow(dead_code)]
fn update_person(
    application: State<'_, LiaisonApplication>,
    request: UpdatePersonRequest,
) -> Result<CommandResult<PersonDto>, ApplicationError> {
    application.update_person(UpdatePersonCommand {
        session_id: request.session_id,
        person_id: request.person_id,
        expected_revision: request.expected_revision,
        display_name: request.display_name,
        emails: request.emails,
        phones: request.phones,
    })
}

#[tauri::command]
#[allow(dead_code)]
fn archive_person(
    application: State<'_, LiaisonApplication>,
    request: ArchivePersonRequest,
) -> Result<CommandResult<PersonDto>, ApplicationError> {
    application.archive_person(ArchivePersonCommand {
        session_id: request.session_id,
        person_id: request.person_id,
        expected_revision: request.expected_revision,
    })
}

// Retained as a P10/P11 incubation boundary. These commands are deliberately
// not registered in the production invoke handler until the durable event,
// grant, recovery, and installed-experience gates are complete.
#[tauri::command]
#[allow(dead_code)]
fn create_event(
    application: State<'_, LiaisonApplication>,
    request: CreateEventRequest,
) -> Result<CommandResult<EventDto>, ApplicationError> {
    application.create_event(CreateEventCommand {
        session_id: request.session_id,
        name: request.name,
        date: request.date,
    })
}

#[tauri::command]
#[allow(dead_code)]
fn list_events(
    application: State<'_, LiaisonApplication>,
    request: WorkspaceSessionRequest,
) -> Result<CommandResult<Vec<EventDto>>, ApplicationError> {
    application.list_events(ListEventsQuery {
        session_id: request.session_id,
    })
}

#[tauri::command]
#[allow(dead_code)]
fn add_event_attendee(
    application: State<'_, LiaisonApplication>,
    request: AddEventAttendeeRequest,
) -> Result<CommandResult<EventDto>, ApplicationError> {
    application.add_event_attendee(AddEventAttendeeCommand {
        session_id: request.session_id,
        event_id: request.event_id,
        person_id: request.person_id,
    })
}

#[tauri::command]
#[allow(dead_code)]
fn resolve_attendee_gap(
    application: State<'_, LiaisonApplication>,
    request: ResolveAttendeeGapRequest,
) -> Result<CommandResult<EventDto>, ApplicationError> {
    application.resolve_attendee_gap(ResolveAttendeeGapCommand {
        session_id: request.session_id,
        event_id: request.event_id,
        row_id: request.row_id,
        action: request.action,
    })
}

#[tauri::command]
#[allow(dead_code)]
fn finalize_event_cohort(
    application: State<'_, LiaisonApplication>,
    request: FinalizeEventCohortRequest,
) -> Result<CommandResult<EventDto>, ApplicationError> {
    application.finalize_event_cohort(FinalizeEventCohortCommand {
        session_id: request.session_id,
        event_id: request.event_id,
    })
}

#[tauri::command]
fn validate_workspace(
    application: State<'_, LiaisonApplication>,
    request: WorkspaceSessionRequest,
) -> Result<CommandResult<WorkspaceValidationDto>, ApplicationError> {
    validate_workspace_impl(&application, request)
}

#[tauri::command]
fn inspect_workspace_health(
    application: State<'_, LiaisonApplication>,
    path: String,
) -> Result<CommandResult<WorkspaceValidationDto>, ApplicationError> {
    inspect_workspace_health_impl(&application, path)
}

#[tauri::command]
fn close_workspace(
    application: State<'_, LiaisonApplication>,
    request: WorkspaceSessionRequest,
) -> Result<CommandResult<WorkspaceClosedDto>, ApplicationError> {
    close_workspace_impl(&application, request)
}

fn initialise_workspace_impl(
    application: &LiaisonApplication,
    request: InitialiseWorkspaceRequest,
) -> Result<CommandResult<WorkspaceOpenDto>, ApplicationError> {
    application.initialise_workspace(InitialiseWorkspaceCommand {
        path: request.path,
        name: request.name,
        profile: request.profile,
        build_profile: BuildProfile::ConnectedLocal,
        locale: DEFAULT_LOCALE.to_owned(),
    })
}

fn create_person_impl(
    application: &LiaisonApplication,
    request: CreatePersonRequest,
) -> Result<CommandResult<PersonDto>, ApplicationError> {
    application.create_person(CreatePersonCommand {
        session_id: request.session_id,
        display_name: request.display_name,
        email: request.email,
    })
}

fn open_workspace_impl(
    application: &LiaisonApplication,
    path: String,
) -> Result<CommandResult<WorkspaceOpenDto>, ApplicationError> {
    application.open_workspace(OpenWorkspaceCommand { path })
}

fn list_people_impl(
    application: &LiaisonApplication,
    request: WorkspaceSessionRequest,
) -> Result<CommandResult<Vec<PersonDto>>, ApplicationError> {
    application.list_people(ListPeopleQuery {
        session_id: request.session_id,
        include_archived: false,
    })
}

fn validate_workspace_impl(
    application: &LiaisonApplication,
    request: WorkspaceSessionRequest,
) -> Result<CommandResult<WorkspaceValidationDto>, ApplicationError> {
    application.validate_workspace(WorkspaceSessionCommand {
        session_id: request.session_id,
    })
}

fn inspect_workspace_health_impl(
    application: &LiaisonApplication,
    path: String,
) -> Result<CommandResult<WorkspaceValidationDto>, ApplicationError> {
    application.inspect_workspace_health(InspectWorkspaceHealthQuery { path })
}

fn close_workspace_impl(
    application: &LiaisonApplication,
    request: WorkspaceSessionRequest,
) -> Result<CommandResult<WorkspaceClosedDto>, ApplicationError> {
    application.close_workspace(WorkspaceSessionCommand {
        session_id: request.session_id,
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let result = tauri::Builder::default()
        .manage(LiaisonApplication::new())
        .invoke_handler(tauri::generate_handler![
            app_status,
            default_workspace_path,
            initialise_workspace,
            open_workspace,
            list_people,
            create_person,
            validate_workspace,
            inspect_workspace_health,
            close_workspace
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
        CreatePersonRequest, InitialiseWorkspaceRequest, WorkspaceSessionRequest,
        close_workspace_impl, create_person_impl, initialise_workspace_impl,
        inspect_workspace_health_impl, open_workspace_impl,
    };
    use liaison_application::{
        APPLICATION_CONTRACT_VERSION, BuildProfile, LiaisonApplication, Revision, WorkspaceProfile,
    };
    use serde_json::{Value, json};
    use std::fs;
    use tempfile::tempdir;

    fn parity_fixture() -> Result<Value, serde_json::Error> {
        serde_json::from_str(include_str!(
            "../../../../spec/fixtures/application-parity.json"
        ))
    }

    #[test]
    fn desktop_request_types_deserialize_the_actual_camel_case_ipc_payloads()
    -> Result<(), Box<dyn std::error::Error>> {
        let session = "00000000-0000-4000-8000-000000000123";
        let create: CreatePersonRequest = serde_json::from_value(json!({
            "sessionId": session,
            "displayName": "Alex Murphy",
            "email": "alex@example.test"
        }))?;
        assert_eq!(create.session_id.to_string(), session);
        assert_eq!(create.display_name, "Alex Murphy");
        assert_eq!(create.email.as_deref(), Some("alex@example.test"));

        let workspace: WorkspaceSessionRequest = serde_json::from_value(json!({
            "sessionId": session
        }))?;
        assert_eq!(workspace.session_id.to_string(), session);

        let initialise: InitialiseWorkspaceRequest = serde_json::from_value(json!({
            "path": "/tmp/liaison-review",
            "name": "Review workspace",
            "profile": "workplace"
        }))?;
        assert_eq!(initialise.profile, WorkspaceProfile::Workplace);

        let wrong_case = serde_json::from_value::<CreatePersonRequest>(json!({
            "session_id": session,
            "display_name": "Alex Murphy",
            "email": null
        }));
        assert!(wrong_case.is_err());
        Ok(())
    }

    #[test]
    fn desktop_requests_use_the_application_session() -> Result<(), Box<dyn std::error::Error>> {
        let temporary = tempdir()?;
        let root = temporary.path().join("workspace");
        let application = LiaisonApplication::new();
        let opened = initialise_workspace_impl(
            &application,
            InitialiseWorkspaceRequest {
                path: root.to_string_lossy().into_owned(),
                name: "Review workspace".to_owned(),
                profile: WorkspaceProfile::Workplace,
            },
        )?;

        let person = create_person_impl(
            &application,
            CreatePersonRequest {
                session_id: opened.value.workspace.session_id,
                display_name: "Alex Murphy".to_owned(),
                email: Some("alex@example.test".to_owned()),
            },
        )?;

        assert_eq!(person.value.display_name, "Alex Murphy");
        assert_eq!(person.value.revision, Revision::INITIAL);
        assert_eq!(person.value.emails.len(), 1);
        assert_eq!(
            opened.value.workspace.build_profile,
            BuildProfile::ConnectedLocal
        );
        Ok(())
    }

    #[test]
    fn desktop_boundary_matches_the_shared_application_fixture()
    -> Result<(), Box<dyn std::error::Error>> {
        let expected = parity_fixture()?;
        let temporary = tempdir()?;
        let root = temporary.path().join("workspace");
        let application = LiaisonApplication::new();
        let opened = initialise_workspace_impl(
            &application,
            InitialiseWorkspaceRequest {
                path: root.to_string_lossy().into_owned(),
                name: "Parity workspace".to_owned(),
                profile: WorkspaceProfile::Workplace,
            },
        )?;
        let person = create_person_impl(
            &application,
            CreatePersonRequest {
                session_id: opened.value.workspace.session_id,
                display_name: "Healthy Person".to_owned(),
                email: None,
            },
        )?;
        assert_eq!(
            person.value.revision.get(),
            expected["initial_person_revision"]
        );
        fs::write(root.join("people/malformed.md"), "not front matter\n")?;

        close_workspace_impl(
            &application,
            WorkspaceSessionRequest {
                session_id: opened.value.workspace.session_id,
            },
        )?;

        let reopened = open_workspace_impl(&application, root.to_string_lossy().into_owned())?;
        assert_eq!(
            u64::from(reopened.contract_version),
            expected["contract_version"]
        );
        assert_eq!(reopened.value.people.len(), 1);
        assert!(!reopened.value.validation.valid);
        let finding = serde_json::to_value(&reopened.value.validation.findings[0])?;
        assert_eq!(finding["code"], expected["malformed_health"]["code"]);
        assert_eq!(
            finding["severity"],
            expected["malformed_health"]["severity"]
        );
        assert_eq!(finding["path"], expected["malformed_health"]["path"]);
        assert_eq!(finding["message"], expected["malformed_health"]["message"]);
        assert_eq!(
            finding["recovery"],
            expected["malformed_health"]["recovery"]
        );

        let result = open_workspace_impl(&application, "relative/private-path".to_owned());
        assert!(result.is_err());
        let Err(error) = result else {
            return Ok(());
        };
        let mut normalized = serde_json::to_value(error)?;
        normalized["correlation_id"] = json!("<uuid>");
        assert_eq!(normalized, expected["relative_path_error"]);
        Ok(())
    }

    #[test]
    fn desktop_surfaces_typed_contention_while_health_remains_available()
    -> Result<(), Box<dyn std::error::Error>> {
        let temporary = tempdir()?;
        let root = temporary.path().join("workspace");
        let writer = LiaisonApplication::new();
        let observer = LiaisonApplication::new();
        let _opened = initialise_workspace_impl(
            &writer,
            InitialiseWorkspaceRequest {
                path: root.to_string_lossy().into_owned(),
                name: "Contended".to_owned(),
                profile: WorkspaceProfile::Workplace,
            },
        )?;

        let contended = open_workspace_impl(&observer, root.to_string_lossy().into_owned());
        assert!(contended.is_err());
        let Err(error) = contended else {
            return Ok(());
        };
        assert_eq!(error.code, "workspace.writer-already-active");
        assert!(error.details.is_empty());
        let rendered = serde_json::to_string(&error)?;
        assert!(!rendered.contains(&root.to_string_lossy().into_owned()));
        assert!(!rendered.contains("process_id"));

        let health = inspect_workspace_health_impl(&observer, root.to_string_lossy().into_owned())?;
        assert!(health.value.valid);
        Ok(())
    }

    #[test]
    fn application_errors_remain_structured_at_the_desktop_boundary()
    -> Result<(), Box<dyn std::error::Error>> {
        let application = LiaisonApplication::new();
        let result = initialise_workspace_impl(
            &application,
            InitialiseWorkspaceRequest {
                path: "relative/path".to_owned(),
                name: "Review workspace".to_owned(),
                profile: WorkspaceProfile::Personal,
            },
        );
        let Err(error) = result else {
            return Err("a relative path must be rejected".into());
        };

        assert_eq!(error.code, "application.workspace-path-not-absolute");
        assert_eq!(error.contract_version, APPLICATION_CONTRACT_VERSION);
        assert!(!error.recovery.is_empty());
        assert!(error.details.is_empty());
        Ok(())
    }
}
