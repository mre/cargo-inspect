use crate::errors::InspectError;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// Add comments to the inspected code. Copy each code line as a comment above
/// the line. This is kind of a crude but effective method. In the long run,
/// this might be a nice feature for rustc however.
pub fn comment_file(path: &Path) -> Result<PathBuf, InspectError> {
    let file = File::open(path)?;
    let commented: String = BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|line| line.trim().to_string())
        .map(|line| {
            let line = if line.is_empty() || is_comment(&line) || is_block(&line) {
                // Don't touch non-code lines.
                line
            } else {
                // Duplicate line (one commented, one uncommented)
                format!("{}\n{}", add_marker("before: ", &line), line)
            };
            format!("{}\n", line)
        })
        .collect();

    // Write the result into a temporary file to avoid mutating the input file.
    // NamedTempFile is is NOT secure/reliable in the presence of a pathological
    // temporary file cleaner.
    let mut tmpfile = NamedTempFile::new()?;
    tmpfile.write_all(commented.as_bytes())?;
    Ok(tmpfile.path().to_path_buf())
}

fn is_comment(line: &str) -> bool {
    let markers = ["//", "//!", "/*", "///"];
    for marker in &markers {
        if line.starts_with(marker) {
            return true;
        }
    }
    false
}

fn is_block(line: &str) -> bool {
    let blocks = ["(", ")", "[", "]", "{", "}"];
    for block in &blocks {
        if &line == block {
            return true;
        }
    }
    false
}

fn add_marker(marker: &str, line: &str) -> String {
    format!("// {}{}", marker, line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_comment() {
        assert_eq!(is_comment("a test"), false);
        assert_eq!(is_comment("// a test"), true);
        assert_eq!(is_comment("/// doc test"), true);
        assert_eq!(is_comment("//! doc test"), true);
        assert_eq!(is_comment("/* doc test"), true);
    }

    #[test]
    fn test_add_marker() {
        assert_eq!(add_marker("", "a test"), "// a test");
        assert_eq!(add_marker("marker: ", "a test"), "// marker: a test");
    }

    #[test]
    fn test_is_block() {
        assert_eq!(is_block("("), true);
        assert_eq!(is_block("}"), true);
        assert_eq!(is_block("( something"), false);
        assert_eq!(is_block("{ something"), false);
    }
}
