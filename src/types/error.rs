use std::fmt;
use std::convert::From;
use std::io;

use crate::types::FileType;
use crate::types::ResRefError;
use crate::files::x2da::types::X2daError as E2da;

#[derive(Debug)]
pub enum Error
{
    DumbError,
    IoError(io::Error),
    X2daError(E2da),
    PathAlreadyExists(String),
    InvalidFileTypeForErf(FileType),
    ResRefError(ResRefError),
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)
        -> fmt::Result
    {
        match self {
            Error::DumbError =>
                write!(f, "Ahhhh!!!! Dumb Error? How could this be?!"),
            Error::PathAlreadyExists(path) =>
                write!(f, "Path <{}> already exists.", path),
            Error::IoError(e) =>
                write!(f, "IO error <{}>", e),
            Error::InvalidFileTypeForErf(file_type) =>
                write!(
                    f,
                    "Erf file type must be ERF, HAK, SAV, MOD. You tried to use <{}>.",
                    file_type.as_str_ref()
                ),
            Error::X2daError(e) =>
                write!(f, "{}", e),
            Error::ResRefError(e) =>
                write!(f, "{}", e)
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error)
        -> Self
    {
        Error::IoError(e)
    }
}

impl From<E2da> for Error {
    fn from(e: E2da)
        -> Self
    {
        Error::X2daError(e)
    }
}

impl From<ResRefError> for Error {
    fn from(e: ResRefError)
        -> Self
    {
        Error::ResRefError(e)
    }
}

impl std::error::Error for Error {}