use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::string;

#[derive(Debug)]
pub enum Error {
    YoutubeDlError(String),
    ApplicationError(String),
    UserError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::YoutubeDlError(msg) => write!(f, "youtube-dl error: {}", msg.trim()),
            Error::ApplicationError(msg) => write!(f, "application error: {}", msg.trim()),
            Error::UserError(msg) => write!(f, "user error: {}", msg.trim()),
        }
    }
}

impl StdError for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::YoutubeDlError(err.to_string())
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::ApplicationError(err.to_string())
    }
}
