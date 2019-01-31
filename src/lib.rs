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

use std::env;
use std::fs;
use std::path::PathBuf;

/// Available configuration settings when using cargo-inspect as a library
pub mod config;

/// Contains all types defined for error handling
pub mod errors;

mod comment;
mod format;
mod hir;
mod diff;
mod tmpfile;

use prettyprint::PrettyPrinter;

pub use crate::config::{Config, Opt};
pub use crate::errors::InspectError;

pub use crate::tmpfile::tmpfile;
pub use crate::diff::diff;
use crate::comment::comment_file;
use crate::format::format;
use crate::hir::HIR;

/// inspect takes a Rust file or crate as an input and returns the desugared
/// output.
pub fn inspect(config: &Config) -> Result<(), InspectError> {
    let output = match &config.files {
        Some(files) => { 
            let hir0 = inspect_file(PathBuf::from(files.0.clone()), config.verbose, config.unpretty.clone())?;
            let hir1 = inspect_file(PathBuf::from(files.1.clone()), config.verbose, config.unpretty.clone())?;
            diff(hir0.output, hir1.output)?
        },
       None => inspect_single(config)?,
    };

    if config.plain {
        println!("{}", output);
    } else {
        let printer = PrettyPrinter::default().language("rust").build()?;
        let header = config.input.to_owned().unwrap_or(env::current_dir()?);
        printer.string_with_header(output, header.to_string_lossy().to_string())?;
    }
    Ok(())
}

/// Run inspection on a single file or crate. Return the compiler output
/// (preferably formatted with rustfmt)
pub fn inspect_single(config: &Config) -> Result<String, InspectError> {
    let hir = match config.input.clone() {
        Some(input) => inspect_file(input, config.verbose, config.unpretty.clone()),
        None => inspect_crate(config),
    }?;

    let mut formatted = format(&hir.output)?;
    if formatted.is_empty() {
        // In case of an error, rustfmt returns an empty string
        // and we continue with the unformatted output.
        // Not ideal, but better than panicking.
        formatted = hir.output;
    }
    Ok(formatted)
}

/// Run cargo-inspect on a file
fn inspect_file(input: PathBuf, verbose: bool, unpretty: String) -> Result<HIR, InspectError> {
    let input = match verbose {
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
    hir::from_file(&input, &unpretty)
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
