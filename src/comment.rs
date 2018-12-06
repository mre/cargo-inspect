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
        .filter_map(Result::ok)
        .filter(|line| !is_comment(&line))
        .map(|line| {
            format!("{}\n{}\n", comment(&line), line)
        }) 
        .collect();

    fs::write(path, commented)?;
    Ok(())
}

fn is_comment(line: &str) -> bool {
    line.starts_with("//")
}

fn comment(line: &str) -> String {
    match is_comment(line) {
        true => line.to_string(),
        false => format!("// {}", line.trim())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_comment() {
        assert_eq!(is_comment("a test"), false);
        assert_eq!(is_comment("// a test"), true);
        assert_eq!(is_comment("/// doc test"), true);
        assert_eq!(is_comment("!// doc test"), true);
    }

    #[test]
    fn test_comment() {
        assert_eq!(comment("a test"), "// a test");
        assert_eq!(comment("// a test"), "// a test");
        assert_eq!(comment("/// doc test"), "doc test");
    }
}