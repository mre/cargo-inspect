use std::path::PathBuf;
use structopt::StructOpt;

/// Holds all configuration options when using
/// cargo-inspect as a library.
#[derive(StructOpt, Debug)]
pub struct Config {
    /// Input file
    #[structopt(name = "INPUT_FILE", parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// rustc "unpretty" parameters
    #[structopt(long = "unpretty", default_value = "hir")]
    pub unpretty: String,

    /// Print the original code as a comment above the desugared code
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// Don't highlight output
    #[structopt(long = "plain")]
    pub plain: bool,
}

/// The structopt enum, which serves as an adapter so that the config options
/// can be passed to cargo-inspect via cargo
#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
pub enum Opt {
    /// Config options are wrapped in an enum variant,
    /// so that they can be destructured easily when calling
    /// structops Opt::from_args().
    #[structopt(name = "inspect")]
    Config(Config),
}
