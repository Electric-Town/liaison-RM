use cap_std::{ambient_authority, fs::Dir};
use liaison_application::{
    BuildProfile, CreatePersonCommand, InitialiseWorkspaceCommand, LiaisonApplication,
    WorkspaceProfile,
};
use std::{fs, path::Path};
use tempfile::tempdir;

fn describe_tree(path: &Path, indent: usize) {
    let Ok(entries) = fs::read_dir(path) else {
        eprintln!("{:indent$}<unreadable {}>", "", path.display(), indent = indent);
        return;
    };
    let mut entries = entries.flatten().collect::<Vec<_>>();
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let entry_path = entry.path();
        eprintln!("{:indent$}{}", "", entry_path.display(), indent = indent);
        if entry_path.is_dir() {
            describe_tree(&entry_path, indent + 2);
        } else if let Ok(bytes) = fs::read(&entry_path) {
            if bytes.len() <= 4096 {
                if let Ok(text) = String::from_utf8(bytes) {
                    eprintln!("{:indent$}{}", "", text, indent = indent + 2);
                }
            }
        }
    }
}

#[test]
fn cloned_capability_directory_can_be_synced() -> Result<(), Box<dyn std::error::Error>> {
    let temporary = tempdir()?;
    let directory = Dir::open_ambient_dir(temporary.path(), ambient_authority())?;
    let file = directory.try_clone()?.into_std_file();
    file.sync_all()?;
    Ok(())
}

#[test]
fn expose_first_person_operation_state_on_windows() {
    let temporary = tempdir().expect("temporary directory");
    let root = temporary.path().join("workspace");
    let application = LiaisonApplication::new();
    let opened = application
        .initialise_workspace(InitialiseWorkspaceCommand {
            path: root.to_string_lossy().into_owned(),
            name: "Windows operation diagnostic".to_owned(),
            profile: WorkspaceProfile::Personal,
            build_profile: BuildProfile::ConnectedLocal,
            locale: "en-IE".to_owned(),
        })
        .expect("workspace initialises");

    let result = application.create_person(CreatePersonCommand {
        session_id: opened.value.workspace.session_id,
        display_name: "Diagnostic Person".to_owned(),
        email: None,
    });
    if let Err(error) = result {
        eprintln!("application error: {error:?}");
        describe_tree(&root, 0);
        panic!("first Person create failed");
    }
}
