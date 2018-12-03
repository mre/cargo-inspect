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

#[macro_use]
extern crate structopt;

extern crate prettyprint;
extern crate syntect;

/// Available configuration settings when using cargo-inspect as a library
pub mod config;

mod errors;
mod format;
mod hir;

use prettyprint::PrettyPrinter;

pub use config::{Config, Opt};
pub use errors::InspectError;
use format::format;
use hir::get_hir;

/// inspect takes a Rust file as an input and returns
/// the desugared output.
pub fn inspect(config: Config) -> Result<(), InspectError> {
    let hir = get_hir(config.input.as_path(), config.unpretty)?;
    let formatted = format(hir)?;

    let printer = PrettyPrinter::default().language("rust").build()?;
    printer.string_with_header(formatted, config.input.to_string_lossy().to_string())?;
    Ok(())
}
