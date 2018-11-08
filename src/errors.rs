use std::io::Error as IoError;
use std::string::FromUtf8Error;

#[derive(Debug, Fail)]
pub enum InspectError {
    #[fail(display = "{}", _0)]
    Io(#[fail(cause)] IoError),
    #[fail(display = "{}", _0)]
    Rustc(String),
    #[fail(display = "{}", _0)]
    Utf8(#[fail(cause)] FromUtf8Error),
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
