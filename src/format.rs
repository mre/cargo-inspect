use crate::errors::InspectError;
use std::io::Write;
use std::process::Command;
use std::process::Stdio;

use log::error;

// TODO: This should not call rustfmt from the commandline.
// Instead, we should use it as a library. Oh well.
pub fn format(input: &String, verbose: bool) -> Result<String, InspectError> {
    let mut builder = Command::new("rustfmt");
    builder
        .arg("--emit")
        .arg("stdout")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped());

    let mut cmd = builder.spawn()?;

    cmd.stdin
        .as_mut()
        .ok_or(InspectError::Rustfmt("Cannot pipe input".to_string()))?
        .write_all(input.as_bytes())?;

    let output = cmd.wait_with_output().expect("Failed to read stdout");

    // Only log out formatting errors when the verbose flag is specified, 
    // if the formatting failed we print out the plain text
    if verbose {
        error!("Formatting failed with following errors:");
        use std::io::BufRead;
        for line in output.stderr.lines() {
            let line = line?;
            error!("{}", line);
        }
    }



    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}
