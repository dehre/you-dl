use std::{env, error, fmt};

#[derive(Debug)]
pub struct Config {
    pub input_file: String,
    pub output_directory: String,
}

#[derive(Debug)]
pub struct ParseConfigError;

impl fmt::Display for ParseConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid arguments. Usage: youtube_downloader <input_file> <output_directory>"
        )
    }
}

impl error::Error for ParseConfigError {}

pub fn parse_from_env() -> Result<Config, ParseConfigError> {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err(ParseConfigError);
    }

    Ok(Config {
        input_file: args.remove(1),
        output_directory: args.remove(1),
    })
}
