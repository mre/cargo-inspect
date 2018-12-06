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

#[macro_use]
extern crate failure;

extern crate structopt;

extern crate indicatif;
extern crate prettyprint;
extern crate syntect;

/// Available configuration settings when using cargo-inspect as a library
pub mod config;

/// Contains all types defined for error handling
pub mod errors;
mod format;
mod hir;

use prettyprint::PrettyPrinter;

pub use crate::config::{Config, Opt};
pub use crate::errors::InspectError;
use crate::format::format;

/// inspect takes a Rust file as an input and returns
/// the desugared output.
pub fn inspect(config: Config) -> Result<(), InspectError> {
    let hir = hir::get_hir(config.input, config.unpretty)?;
    let formatted = format(hir.output)?;

    let printer = PrettyPrinter::default().language("rust").build()?;
    printer.string_with_header(formatted, hir.source)?;
    Ok(())
}
