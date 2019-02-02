use std::process::Command;
use std::path::PathBuf;
pub use crate::errors::InspectError;
pub use crate::tmpfile::write_tmp;

use difference::{Difference, Changeset};
use std::io::Write;
use std::fs;

pub fn diff(text1: String, text2: String) -> Result<String, InspectError> {
    let Changeset { diffs, .. } = Changeset::new(&text1, &text2, "\n");

    let mut t = String::new();

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                t.reset().unwrap();
                writeln!(t, " {}", x);
            }
            Difference::Add(ref x) => {
                match diffs[i - 1] {
                    Difference::Rem(ref y) => {
                        t.fg(term::color::GREEN).unwrap();
                        write!(t, "+");
                        let Changeset { diffs, .. } = Changeset::new(y, x, " ");
                        for c in diffs {
                            match c {
                                Difference::Same(ref z) => {
                                    t.fg(term::color::GREEN).unwrap();
                                    write!(t, "{}", z);
                                    write!(t, " ");
                                }
                                Difference::Add(ref z) => {
                                    t.fg(term::color::WHITE).unwrap();
                                    t.bg(term::color::GREEN).unwrap();
                                    write!(t, "{}", z);
                                    t.reset().unwrap();
                                    write!(t, " ");
                                }
                                _ => (),
                            }
                        }
                        writeln!(t, "");
                    }
                    _ => {
                        t.fg(term::color::BRIGHT_GREEN).unwrap();
                        writeln!(t, "+{}", x);
                    }
                };
            }
            Difference::Rem(ref x) => {
                t.fg(term::color::RED).unwrap();
                writeln!(t, "-{}", x);
            }
        }
    }
    // t.reset().unwrap();
    // t.flush().unwrap();
    Ok(t)
}
