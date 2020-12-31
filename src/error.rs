use core::fmt;
use std::error::Error;
use std::fmt::Debug;

#[derive(Debug)]
pub enum ErrorKind {
    JsonError(serde_json::Error),
    RequestError(reqwest::Error),
    GenericError,
}

impl Error for ErrorKind {}
impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <dyn Debug>::fmt(self, f)
    }
}
