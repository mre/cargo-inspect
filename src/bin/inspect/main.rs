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
extern crate failure;

use failure::Fail;

use cargo_inspect::{config, inspect};
use std::process;
use structopt::StructOpt;

fn main() {
    let config::Opt::Config(config) = config::Opt::from_args();
    if let Err(err) = inspect(config) {
        eprintln!("Command failed:\n{}\n", err);

        for cause in Fail::iter_causes(&err) {
            println!("{}", cause);
        }

        if let Some(backtrace) = err.backtrace() {
            eprintln!("Backtrace: {:?}", backtrace);
        }

        process::exit(1);
    }
}
