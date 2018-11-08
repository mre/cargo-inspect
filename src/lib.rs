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
