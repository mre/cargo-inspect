use crate::errors::InspectError;
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{self, Command};

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
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(100);
    pb.set_style(ProgressStyle::default_spinner().template("{spinner} cargo {wide_msg}"));

    let mut p = process::Command::new("cargo")
        .arg("+nightly")
        .arg("rustc")
        .arg("--")
        .arg(format!("-Zunpretty={}", unpretty))
        .stderr(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;

    let stderr = BufReader::new(
        p.stderr
            .take()
            .ok_or_else(|| "Cannot read from stderr".to_string())?,
    );
    for line in stderr.lines() {
        let line = line.unwrap();
        let stripped_line = line.trim();
        if !stripped_line.is_empty() {
            pb.set_message(&stripped_line.to_lowercase());
        }
        pb.tick();
    }

    let out = p.wait_with_output()?;
    pb.finish_and_clear();

    Ok(HIR {
        source: "crate".to_string(),
        output: String::from_utf8(out.stdout)?,
    })
}
