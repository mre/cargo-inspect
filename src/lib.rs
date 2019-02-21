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

use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

/// Available configuration settings when using cargo-inspect as a library
pub mod config;

/// Contains all types defined for error handling
pub mod errors;

mod comment;
mod diff;
mod format;
mod hir;
mod tmpfile;

use prettyprint::PrettyPrinter;

pub use crate::config::{Config, Opt};
pub use crate::errors::InspectError;

use crate::comment::comment_file;
pub use crate::diff::diff;
use crate::format::format;
use crate::hir::HIR;
pub use crate::tmpfile::tmpfile;

/// inspect takes a Rust file or crate as an input and returns the desugared
/// output.
pub fn inspect(config: &Config) -> Result<(), InspectError> {
    let output = match &config.files {
        Some(files) => {
            let hir0 = inspect_file(
                PathBuf::from(files.0.clone()),
                config.verbose,
                config.unpretty.clone(),
            )?;
            let hir1 = inspect_file(
                PathBuf::from(files.1.clone()),
                config.verbose,
                config.unpretty.clone(),
            )?;
            diff(try_format(hir0.output)?, try_format(hir1.output)?)?
        }
        None => inspect_single(config)?,
    };

    if config.plain {
        println!("{}", output);
    } else if config.unpretty.starts_with("flowgraph") {
        if let Some(path) = &config.input {
            let input_path = PathBuf::from(path);

            // Extract the file name from our input path.
            let file_name = input_path
                .file_name()
                .ok_or(InspectError::Flowgraph(String::from(
                    "Invalid path found. The input path should be a file.",
                )))?;

            let mut output_path = PathBuf::from(file_name);
            let ext = &config.format;
            output_path.set_extension(&ext);

            // Create a temporary file to dump out the plain output
            let tmp_file_path = tmpfile()?;
            let mut file = File::create(&tmp_file_path)?;
            file.write_all(output.as_bytes())?;

            // For now setup the correct `dot` arguments to write to a png
            let output_str = output_path
                .to_str()
                .ok_or(InspectError::Flowgraph(String::from(
                    "Failed to convert output path to string.",
                )))?;

            let input_str = tmp_file_path
                .to_str()
                .ok_or(InspectError::Flowgraph(String::from(
                    "Failed to convert temporary path to string.",
                )))?;

            log::info!("Writing \"{}\"...", output_str);

            let args = [&format!("-T{}", ext), "-o", output_str, input_str];
            Command::new("dot")
                .args(&args)
                .spawn()
                .map_err(|e| InspectError::DotExec(e))?;
        }
    } else {
        let mut builder = PrettyPrinter::default();
        builder.language("rust");
        if let Some(theme) = &config.theme {
            builder.theme(theme.clone());
        }

        let printer = builder.build()?;
        let header = config.input.to_owned().unwrap_or(env::current_dir()?);
        printer.string_with_header(output, header.to_string_lossy().to_string())?;
    }
    Ok(())
}

/// Run inspection on a single file or crate. Return the compiler output
/// (preferably formatted with rustfmt)
pub fn inspect_single(config: &Config) -> Result<String, InspectError> {
    let hir = match config.input.clone() {
        Some(input) => inspect_file(input, config.verbose, config.unpretty.clone()),
        None => inspect_crate(config),
    }?;
    Ok(try_format(hir.output)?)
}

/// Run cargo-inspect on a file
fn inspect_file(input: PathBuf, verbose: bool, unpretty: String) -> Result<HIR, InspectError> {
    let input = match verbose {
        true => {
            // Create a temporary copy of the input file,
            // which contains comments for each input line
            // to avoid modifying the original input file.
            // This will be used as the input of rustc.
            let tmp = tmpfile()?;
            fs::copy(&input, &tmp)?;
            comment_file(&tmp)?;
            tmp
        }
        false => input.into(),
    };
    hir::from_file(&input, &unpretty)
}

/// Run cargo-inspect on a crate
fn inspect_crate(config: &Config) -> Result<HIR, InspectError> {
    if config.verbose {
        unimplemented!(
            "Verbose option doesn't work for crates yet. \
             See https://github.com/mre/cargo-inspect/issues/5"
        )
        // comment_crate()?;
    }
    hir::from_crate(&config.unpretty)
}

// TODO: This should really be more idiomatic;
// maybe by having a `Formatted` type and a `let fmt = Formatted::try_from(string);`
fn try_format(input: String) -> Result<String, InspectError> {
    let mut formatted = format(&input)?;
    if formatted.is_empty() {
        // In case of an error, rustfmt returns an empty string
        // and we continue with the unformatted output.
        // Not ideal, but better than panicking.
        formatted = input;
    }
    Ok(formatted)
}
