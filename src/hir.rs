use crate::errors::InspectError;
use std::path::PathBuf;
use std::process::Command;

/// A wrapper around Rust's high-level intermediate representation
pub struct HIR {
    /// Source file or crate of HIR
    pub source: String,
    /// The actual HIR output
    pub output: String,
}

// TODO: This should probably not take a filename,
// but take a String as an input.
// Would make testing easier.
pub fn from_file(path: PathBuf, unpretty: String) -> Result<HIR, InspectError> {
    let output = Command::new("rustc")
        .arg("+nightly")
        .arg(format!("-Zunpretty={}", unpretty))
        .arg(&path)
        .output()?;
    let stderr = String::from_utf8(output.stderr)?;
    if !stderr.is_empty() {
        return Err(InspectError::Rustc(stderr));
    }
    Ok(HIR {
        source: path.to_string_lossy().to_string(),
        output: String::from_utf8(output.stdout)?,
    })
}

pub fn from_crate(unpretty: String) -> Result<HIR, InspectError> {
    let output = Command::new("cargo")
        .arg("+nightly")
        .arg("rustc")
        .arg("--")
        .arg(format!("-Zunpretty={}", unpretty))
        .output()?;

    Ok(HIR {
        source: "crate".to_string(),
        output: String::from_utf8(output.stdout)?,
    })
}
