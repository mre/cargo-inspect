use errors::InspectError;
use std::io::{Read, Write};
use std::process::Command;
use std::process::Stdio;

// TODO: This should not call rustfmt from the commandline.
// Instead, we should use it as a library. Oh well.
pub fn format(input: String) -> Result<String, InspectError> {
    let mut cmd = Command::new("rustfmt")
        .arg("--emit")
        .arg("stdout")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    cmd.stdin
        .as_mut()
        .ok_or(InspectError::Rustfmt("Cannot pipe input".to_string()))?
        .write_all(input.as_bytes())?;

    let output = cmd.wait_with_output().expect("Failed to read stdout");
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
