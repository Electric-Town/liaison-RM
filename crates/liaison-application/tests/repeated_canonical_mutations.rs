use liaison_application::{
    BuildProfile, CreatePersonCommand, InitialiseWorkspaceCommand, LiaisonApplication,
    ListPeopleQuery, WorkspaceProfile,
};
use std::error::Error;
use tempfile::tempdir;

#[test]
fn consecutive_person_creations_reuse_the_projection_stale_marker() -> Result<(), Box<dyn Error>> {
    let temporary = tempdir()?;
    let root = temporary.path().join("workspace");
    let application = LiaisonApplication::new();
    let opened = application.initialise_workspace(InitialiseWorkspaceCommand {
        path: root.to_string_lossy().into_owned(),
        name: "Repeated mutations".to_owned(),
        profile: WorkspaceProfile::Personal,
        build_profile: BuildProfile::ConnectedLocal,
        locale: "en-IE".to_owned(),
    })?;
    let session_id = opened.value.workspace.session_id;

    application.create_person(CreatePersonCommand {
        session_id,
        display_name: "First Person".to_owned(),
        email: None,
    })?;
    application.create_person(CreatePersonCommand {
        session_id,
        display_name: "Second Person".to_owned(),
        email: None,
    })?;

    let people = application.list_people(ListPeopleQuery {
        session_id,
        include_archived: false,
    })?;
    assert_eq!(people.value.len(), 2);
    assert!(root.join(".liaison/projections/stale").is_file());
    Ok(())
}
