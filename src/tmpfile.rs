pub use crate::errors::InspectError;
use std::fs::{self, File};
use std::path::PathBuf;
use tempfile::tempdir;

/// Create a temporary file
pub fn tmpfile() -> Result<PathBuf, InspectError> {
    let tmp_path = tempdir()?.into_path();
    let file_path = tmp_path.join("temp.rs");
    File::create(&file_path)?;
    Ok(file_path)
}

/// Create a temporary file with the given data
pub fn write_tmp(data: String) -> Result<PathBuf, InspectError> {
    let file = tmpfile()?;
    fs::write(&file, data)?;
    Ok(file)
}
