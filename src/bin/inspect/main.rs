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

use cargo_inspect::{config, errors::InspectError, inspect};
use std::process;
use structopt::StructOpt;

use prettyprint::PrettyPrinter;

fn run() -> Result<(), InspectError> {
    let config::Opt::Config(config) = config::Opt::from_args();

    // Handle list-themes command
    if config.list_themes {
        let printer = PrettyPrinter::default().build()?;
        let themes = printer.get_themes();
        eprintln!("Themes:");
        for (path, _) in themes {
            eprintln!("  {}", path);
        }
        return Ok(());
    }

    inspect(&config)
}

fn main() {
    env_logger::init();
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
