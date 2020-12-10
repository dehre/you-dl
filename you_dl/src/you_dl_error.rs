use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::string;

#[derive(Debug)]
pub enum YouDlError {
    YoutubeDlError(String),
    ApplicationError(String),
    UserError(String),
}

impl fmt::Display for YouDlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YouDlError::YoutubeDlError(msg) => write!(f, "Youtube-dl error: {}", msg.trim()),
            YouDlError::ApplicationError(msg) => write!(f, "Application error: {}", msg.trim()),
            YouDlError::UserError(msg) => write!(f, "User error: {}", msg.trim()),
        }
    }
}

impl StdError for YouDlError {}

impl From<io::Error> for YouDlError {
    fn from(err: io::Error) -> Self {
        YouDlError::YoutubeDlError(err.to_string())
    }
}

impl From<string::FromUtf8Error> for YouDlError {
    fn from(err: string::FromUtf8Error) -> Self {
        YouDlError::ApplicationError(err.to_string())
    }
}
