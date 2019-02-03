use ansi_term::Colour::{Green, Red, White};
use difference::{Changeset, Difference};

pub use crate::errors::InspectError;

/// Compare two strings and return a Github-style diff output
pub fn diff(text1: String, text2: String) -> Result<String, InspectError> {
    let Changeset { diffs, .. } = Changeset::new(&text1, &text2, "\n");

    let mut t = String::new();

    for i in 0..diffs.len() {
        match diffs[i] {
            Difference::Same(ref x) => {
                t.push_str(&format!("\n{}", x));
            }
            Difference::Add(ref x) => {
                match diffs[i - 1] {
                    Difference::Rem(ref y) => {
                        t.push_str(&Green.paint("\n+").to_string());
                        let Changeset { diffs, .. } = Changeset::new(y, x, " ");
                        for c in diffs {
                            match c {
                                Difference::Same(ref z) => {
                                    t.push_str(&Green.paint(z).to_string());
                                    t.push_str(&Green.paint(" ").to_string());
                                }
                                Difference::Add(ref z) => {
                                    t.push_str(&White.on(Green).paint(z).to_string());
                                    t.push_str(" ");
                                }
                                _ => (),
                            }
                        }
                        t.push_str("");
                    }
                    _ => {
                        t.push_str(&Green.paint(format!("\n+{}", x).to_string()));
                    }
                };
            }
            Difference::Rem(ref x) => {
                t.push_str(&Red.paint(format!("\n-{}", x)).to_string());
            }
        }
    }
    Ok(t)
}
