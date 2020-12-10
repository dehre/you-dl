use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseConfigError(pub String);

impl fmt::Display for ParseConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid program arguments provided: {}", self.0)
    }
}

impl Error for ParseConfigError {}
