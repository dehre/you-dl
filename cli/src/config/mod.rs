mod cli_args;
mod config_error;

use cli_args::parse_cli_args;
use config_error::ConfigError;
use smol::fs;

#[derive(Debug)]
pub struct Config {
    pub video_urls: Vec<String>,
    pub output_dir: String,
    pub use_wrapper: bool,
}

pub async fn parse() -> Result<Config, ConfigError> {
    let cli_args = parse_cli_args()?;

    if cli_args.urls.is_none() && cli_args.from_file_path.is_none() {
        return Err(ConfigError(String::from("no urls to be downloaded")));
    };

    let mut video_urls = Vec::new();
    if let Some(mut urls) = cli_args.urls {
        video_urls.append(&mut urls);
    }
    if let Some(file_path) = cli_args.from_file_path {
        let mut urls_from_file = read_urls_from_file(&file_path).await?;
        video_urls.append(&mut urls_from_file);
    }

    Ok(Config {
        video_urls,
        output_dir: cli_args.output_dir,
        use_wrapper: cli_args.use_wrapper,
    })
}

async fn read_urls_from_file(file_path: &str) -> Result<Vec<String>, ConfigError> {
    let file_contents = fs::read_to_string(file_path).await.map_err(|err| {
        ConfigError(format!(
            "could not read contents from {}: {}",
            file_path, err
        ))
    })?;

    Ok(file_contents
        .lines()
        .filter(|&l| !l.trim().is_empty())
        .map(|s| s.to_owned())
        .collect())
}
