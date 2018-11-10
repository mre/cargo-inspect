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
extern crate syntect;

mod errors;
mod format;
mod highlight;
mod hir;

pub use errors::InspectError;
use format::format;
use highlight::highlight;
use hir::get_hir;
use std::path::PathBuf;

/// inspect takes a path to a Rust file as an input and returns
/// the desugared output.
pub fn inspect(path: PathBuf) -> Result<String, InspectError> {
    let hir = get_hir(path)?;
    let formatted = format(hir)?;
    highlight(formatted)
}
