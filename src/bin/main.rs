extern crate cargo_inspect;

extern crate structopt;

use cargo_inspect::inspect;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    /// Input file
    #[structopt(name = "INPUT_FILE", parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let output = inspect(opt.input).expect("Cannot fetch formatted result");
    println!("{}", output);
}
