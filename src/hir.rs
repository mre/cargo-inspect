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

pub fn get_hir(input: Option<PathBuf>, unpretty: String) -> Result<HIR, InspectError> {
    match input {
        Some(path) => from_file(path, unpretty),
        None => from_crate(unpretty),
    }
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
    pb.enable_steady_tick(200);
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("/|\\- ")
            .template("{spinner:.dim.bold} Compiling {wide_msg}"),
    );

    let mut p = process::Command::new("cargo")
        .arg("+nightly")
        .arg("rustc")
        .arg("--")
        .arg(format!("-Zunpretty={}", unpretty))
        .stderr(process::Stdio::piped())
        .stdout(process::Stdio::piped())
        .spawn()?;

    for line in BufReader::new(p.stderr.take().ok_or_else(|| "Cannot read from stderr".to_string())?).lines() {
        let line = line.unwrap();
        let stripped_line = line.trim();
        if !stripped_line.is_empty() {
            pb.set_message(stripped_line);
        }
        pb.tick();
    }

    let mut output = String::new();
    for line in BufReader::new(p.stdout.take().ok_or_else(|| "Cannot read from stdout".to_string())?).lines() {
        let line = line.unwrap();
        output.push_str(&line);
    }
    p.wait().unwrap();

    pb.finish_and_clear();

    Ok(HIR {
        source: "crate".to_string(),
        // output: String::from_utf8(BufReader::new(p.stdout.unwrap()).lines().collect::<String>())?,
        output: output,
    })
}
