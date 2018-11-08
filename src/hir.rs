use errors::InspectError;
use std::path::PathBuf;
use std::process::Command;

// TODO: This should probably not take a filename,
// but take a String as an input.
// Would make testing easier.
pub fn get_hir(path: PathBuf) -> Result<String, InspectError> {
    let output = Command::new("rustc")
        .arg("+nightly")
        .arg("-Zunpretty=hir")
        .arg(path)
        .output()?;
    let stderr = String::from_utf8(output.stderr)?;
    if !stderr.is_empty() {
        return Err(InspectError::Rustc(stderr));
    }
    Ok(String::from_utf8(output.stdout)?)
}
