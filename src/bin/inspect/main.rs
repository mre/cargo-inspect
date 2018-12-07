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

use failure::Fail;

use cargo_inspect::{errors::InspectError,config, inspect};
use std::process;
use structopt::StructOpt;

fn run() -> Result<(), InspectError> {
    let config::Opt::Config(config) = config::Opt::from_args();
    inspect(&config)
}

fn main() {
    if let Err(err) = run() {
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
