use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

fn parse_tuple<T>(s: &str) -> Result<(T, T), Box<Error>>
where
    T: std::str::FromStr,
    T::Err: Error + 'static,
{
    let pos = s.find(',').ok_or_else(|| {
        format!(
            "Expected format: file1.rs,file2.rs. No `,` found in `{}`",
            s
        )
    })?;
    Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
}

/// Holds all configuration options when using
/// cargo-inspect as a library.
#[derive(StructOpt, Debug)]
pub struct Config {
    /// Input file
    #[structopt(name = "INPUT_FILE", parse(from_os_str))]
    pub input: Option<PathBuf>,

    /// Diff input files
    #[structopt(long = "diff", parse(try_from_str = "parse_tuple"))]
    pub files: Option<(String, String)>,

    /// rustc "unpretty" parameters
    #[structopt(long = "unpretty", default_value = "hir")]
    pub unpretty: String,

    /// Print the original code as a comment above the desugared code
    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,

    /// When this flag is specified we print out more detailed
    /// error messages for rustfmt
    #[structopt(long = "verbose-output")]
    pub verbose_output : bool,

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
