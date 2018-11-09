use std::fs::File;
use std::io::Error;

fn main() -> Result<(), Error> {
    let file = File::open("file.txt")?;
    Ok(())
}
