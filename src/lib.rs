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
extern crate prettyprint;
extern crate syntect;

mod errors;
mod format;
mod hir;

use prettyprint::PrettyPrinter;
use std::path::PathBuf;

pub use errors::InspectError;
use format::format;
use hir::get_hir;

/// inspect takes a path to a Rust file as an input and returns
/// the desugared output.
pub fn inspect(path: PathBuf, unpretty: String) -> Result<(), InspectError> {
    let hir = get_hir(path.as_path(), unpretty)?;
    let formatted = format(hir)?;

    let printer = PrettyPrinter::default().language("rust").build()?;
    printer.string_with_header(formatted, path.to_string_lossy().to_string())?;
    Ok(())
}
