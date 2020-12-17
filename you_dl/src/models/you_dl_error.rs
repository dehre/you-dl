use std::error;
use std::fmt;
use std::io;
use std::string;

#[derive(Debug)]
pub enum YouDlError {
    YoutubeDlError(String),
    ApplicationError(String),
    UndownloadableError(String),
    UserError(String),
}

impl fmt::Display for YouDlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YouDlError::YoutubeDlError(msg) => write!(f, "Youtube-dl error: {}", msg.trim()),
            YouDlError::ApplicationError(msg) => write!(f, "Application error: {}", msg.trim()),
            YouDlError::UndownloadableError(video_id_or_title) => {
                write!(
                    f,
                    "No formats available to be downloaded for: {}",
                    video_id_or_title
                )
            }
            YouDlError::UserError(msg) => write!(f, "User error: {}", msg.trim()),
        }
    }
}

impl error::Error for YouDlError {}

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
