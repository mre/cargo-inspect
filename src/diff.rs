use std::process::Command;
use std::path::PathBuf;
pub use crate::errors::InspectError;
pub use crate::tmpfile::write_tmp;

/// Diff to input files and return the output
/// We'll assume `git` is installed on the machine.
/// We could also use a third-party crate for that,
/// but the git diff output is reasonably stable and
/// hassle-free.
/// If git is not installed, this will return an error.
pub fn diff(input1: String, input2: String) -> Result<String, InspectError> {
    let file1: PathBuf = write_tmp(input1)?;
    let file2: PathBuf = write_tmp(input2)?;

    let output = Command::new("git")
        .arg("diff")
        .arg(file1)
        .arg(file2)
        .output()?;
    let stderr = String::from_utf8(output.stderr)?;
    if !stderr.is_empty() {
        return Err(InspectError::Rustc(stderr));
    }
    Ok(String::from_utf8(output.stdout)?)
}