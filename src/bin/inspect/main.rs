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

use cargo_inspect::inspect;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
enum Opt {
    #[structopt(name = "inspect")]
    Inspect {
        /// Input file
        #[structopt(name = "INPUT_FILE", parse(from_os_str))]
        input: PathBuf,

        /// rustc "unpretty" parameters
        #[structopt(long = "unpretty", default_value = "hir")]
        unpretty: String,
    },
}

fn main() {
    let Opt::Inspect { input, unpretty } = Opt::from_args();
    let output = inspect(input, unpretty).expect("Cannot fetch formatted result");
    println!("{}", output);
}
