pub use crate::errors::InspectError;
use std::fs::File;
use std::path::PathBuf;
use tempfile::tempdir;

/// Create a temporary file
pub fn tmpfile() -> Result<PathBuf, InspectError> {
    let tmp_path = tempdir()?.into_path();
    let file_path = tmp_path.join("temp.rs");
    File::create(&file_path)?;
    Ok(file_path)
}
