//! `cargo inspect`
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

extern crate cargo_inspect;
extern crate structopt;

use cargo_inspect::{config, inspect};
use structopt::StructOpt;

fn main() {
    let config::Opt::Config(config) = config::Opt::from_args();
    inspect(config).expect("Cannot print result")
}
