use cap_std::{ambient_authority, fs::Dir};
use tempfile::tempdir;

#[test]
fn cloned_capability_directory_can_be_synced() -> Result<(), Box<dyn std::error::Error>> {
    let temporary = tempdir()?;
    let directory = Dir::open_ambient_dir(temporary.path(), ambient_authority())?;
    let file = directory.try_clone()?.into_std_file();
    file.sync_all()?;
    Ok(())
}
