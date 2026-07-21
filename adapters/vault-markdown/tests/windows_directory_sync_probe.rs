use cap_fs_ext::{DirExt, OpenOptionsMaybeDirExt};
use cap_std::{
    ambient_authority,
    fs::{Dir, OpenOptions},
};
use std::io::Write;
use tempfile::tempdir;

#[test]
fn writable_nested_capability_directory_handle_can_be_synced(
) -> Result<(), Box<dyn std::error::Error>> {
    let temporary = tempdir()?;
    let root = Dir::open_ambient_dir(temporary.path(), ambient_authority())?;
    root.create_dir("child")?;
    let child = root.open_dir_nofollow("child")?;
    let mut payload = child.create("payload.txt")?;
    payload.write_all(b"durable payload")?;
    payload.sync_all()?;
    drop(payload);

    let mut options = OpenOptions::new();
    options.read(true).write(true).maybe_dir(true);
    let file = child.open_with(".", &options)?.into_std();
    file.sync_all()?;
    Ok(())
}
