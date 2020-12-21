use cli_args::parse_cli_args;
use config_error::ConfigError;
use smol::fs;
use std::path::Path;

mod cli_args;
mod config_error;

#[derive(Debug)]
pub struct Config {
    pub video_urls: Vec<String>,
    pub output_dir: String,
    pub use_wrapper: bool,
}

pub async fn parse() -> Result<Config, ConfigError> {
    let cli_args = parse_cli_args()?;

    let output_dir_path = Path::new(&cli_args.output_dir);
    if !output_dir_path.is_dir() {
        println!("creating directory \"{}\"...", output_dir_path.display());
        fs::create_dir_all(output_dir_path).await.unwrap();
    }

    if cli_args.urls.is_none() && cli_args.from_file_path.is_none() {
        return Err(ConfigError("no urls to be downloaded".to_owned()));
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
        .filter(|&l| !l.starts_with("#") && !l.starts_with("//"))
        .map(|s| s.to_owned())
        .collect())
}
