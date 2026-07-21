use cap_fs_ext::OpenOptionsMaybeDirExt;
use cap_std::{
    ambient_authority,
    fs::{Dir, OpenOptions},
};
use tempfile::tempdir;

#[test]
#[cfg(target_os = "windows")]
fn writable_capability_directory_handle_can_be_synced() -> Result<(), Box<dyn std::error::Error>> {
    let temporary = tempdir()?;
    let directory = Dir::open_ambient_dir(temporary.path(), ambient_authority())?;
    let mut options = OpenOptions::new();
    options.read(true).write(true).maybe_dir(true);
    let file = directory.open_with(".", &options)?.into_std();
    file.sync_all()?;
    Ok(())
}
