use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ConfigError(pub String);

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid program arguments provided: {}", self.0)
    }
}

impl Error for ConfigError {}
