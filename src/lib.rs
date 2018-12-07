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

/// inspect takes a Rust file as an input and returns
/// the desugared output.
pub fn inspect(config: &Config) -> Result<(), InspectError> {
    let hir = match config.input {
        Some(_) => inspect_file(config),
        None => inspect_crate(config),
    }?;

    let formatted = format(hir.output)?;

    let printer = PrettyPrinter::default().language("rust").build()?;
    printer.string_with_header(formatted, hir.source)?;
    Ok(())
}

/// Run cargo-inspect on a file
fn inspect_file(config: &Config) -> Result<HIR, InspectError> {
    let input = match &config.input {
        Some(input) => input,
        None => return Err(InspectError::Generic("No file to analyze".to_string())),
    };
    if config.verbose {
        comment_file(&input)?;
    }
    hir::from_file(input.into(), &config.unpretty)
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
