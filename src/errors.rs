use std::io::Error as IoError;
use std::string::FromUtf8Error;

#[derive(Debug, Fail)]
/// Custom type for inspection errors
pub enum InspectError {
    /// Error during I/O handling
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] IoError),
    /// Error when invoking rustc
    #[fail(display = "{}", _0)]
    Rustc(String),
    /// Error when converting data to UTF8
    #[fail(display = "{}", _0)]
    Utf8(#[fail(cause)] FromUtf8Error),
    /// Error while invoking rustfmt
    #[fail(display = "{}", _0)]
    Rustfmt(String),
}

impl From<IoError> for InspectError {
    fn from(e: IoError) -> Self {
        InspectError::Io(e)
    }
}

impl From<FromUtf8Error> for InspectError {
    fn from(e: FromUtf8Error) -> Self {
        InspectError::Utf8(e)
    }
}
