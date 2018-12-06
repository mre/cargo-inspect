use crate::errors::InspectError;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Add comments to the inspected code. Copy each code line as a comment above
/// the line. This is kind of a crude but effective method. In the long run,
/// this might be a nice feature for rustc however.
pub fn comment_file(path: &PathBuf) -> Result<(), InspectError> {
    let file = File::open(path)?;
    let commented: String = BufReader::new(file)
        .lines()
        .map(|line| [comment(&line), line])
        .collect();

    fs::write(path, commented)?;
    Ok(())
}

fn comment(&line: String) -> String {
    "// ".push_str(line)
}
