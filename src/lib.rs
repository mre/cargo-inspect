//! A library for inspecting Rust code
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]

/// Available configuration settings when using cargo-inspect as a library
pub mod config;

mod comment;
/// Contains all types defined for error handling
pub mod errors;
mod format;
mod hir;

use prettyprint::PrettyPrinter;

use crate::comment::comment_file;
pub use crate::config::{Config, Opt};
pub use crate::errors::InspectError;
use crate::format::format;
use crate::hir::HIR;
use std::env;
use std::fs::{self, File};
use std::path::PathBuf;
use tempfile::tempdir;

/// inspect takes a Rust file or crate as an input and returns the desugared
/// output.
pub fn inspect(config: &Config) -> Result<(), InspectError> {
    let hir = match config.input {
        Some(_) => inspect_file(config),
        None => inspect_crate(config),
    }?;

    let mut formatted = format(&hir.output)?;
    if formatted.is_empty() {
        // In case of an error, rustfmt returns an empty string
        // and we continue with the unformatted output.
        // Not ideal, but better than panicking.
        formatted = hir.output;
    }

    if config.plain {
        println!("{}", formatted);
    } else {
        let printer = PrettyPrinter::default().language("rust").build()?;
        let header = config.input.to_owned().unwrap_or(env::current_dir()?);
        printer.string_with_header(formatted, header.to_string_lossy().to_string())?;
    }
    Ok(())
}

/// Run cargo-inspect on a file
fn inspect_file(config: &Config) -> Result<HIR, InspectError> {
    let input: &PathBuf = match &config.input {
        Some(input) => input,
        None => return Err(InspectError::Generic("No file to analyze".to_string())),
    };

    let input = match config.verbose {
        true => {
            // Create a temporary copy of the input file,
            // which contains comments for each input line
            // to avoid modifying the original input file.
            // This will be used as the input of rustc.
            let tmp = tmpfile()?;
            fs::copy(&input, &tmp)?;
            comment_file(&tmp)?;
            tmp
        }
        false => input.into(),
    };
    hir::from_file(&input, &config.unpretty)
}

/// Run cargo-inspect on a crate
fn inspect_crate(config: &Config) -> Result<HIR, InspectError> {
    if config.verbose {
        unimplemented!(
            "Verbose option doesn't work for crates yet. \
             See https://github.com/mre/cargo-inspect/issues/5"
        )
        // comment_crate()?;
    }
    hir::from_crate(&config.unpretty)
}

fn tmpfile() -> Result<PathBuf, InspectError> {
    let tmp_path = tempdir()?.into_path();
    let file_path = tmp_path.join("temp.rs");
    File::create(&file_path)?;
    Ok(file_path)
}
