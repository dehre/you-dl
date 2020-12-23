use std::error;
use std::fmt;

#[derive(Debug)]
pub enum YouDlError {
    YoutubeDl(String),
    Application(String),
    InvalidURL(String),
    Undownloadable(String, String),
    User(String),
    InvalidResponse(String),
}

impl fmt::Display for YouDlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YouDlError::YoutubeDl(msg) => write!(f, "youtube-dl Error: {}", msg.trim()),
            YouDlError::Application(msg) => write!(f, "Application Error: {}", msg.trim()),
            YouDlError::InvalidURL(url) => {
                write!(f, "Invalid URL Error: {}", url)
            }
            YouDlError::Undownloadable(video_id_or_title, msg) => {
                write!(
                    f,
                    "Undownloadable Error for `{}`: {}",
                    video_id_or_title, msg
                )
            }
            YouDlError::User(msg) => write!(f, "User Error: {}", msg.trim()),
            YouDlError::InvalidResponse(msg) => {
                write!(f, "Invalid Response Error: {}", msg.trim())
            }
        }
    }
}

impl error::Error for YouDlError {}
