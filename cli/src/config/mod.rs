use config_error::ConfigError;
use raw_cli_args::parse as raw_parse;
use smol::{fs, process};
use std::path::Path;

mod config_error;
mod raw_cli_args;

#[derive(Debug)]
pub struct Config {
    pub video_urls: Vec<String>,
    pub output_dir: String,
    pub use_wrapper: bool,
}

pub async fn parse() -> Result<Config, ConfigError> {
    let raw_cli_args = raw_parse()?;

    if raw_cli_args.use_wrapper && !is_youtube_dl_available().await {
        return Err(ConfigError("youtube-dl is not available".to_owned()));
    }

    let output_dir_path = Path::new(&raw_cli_args.output_dir);
    if !output_dir_path.is_dir() {
        // TODO LORIS: use info! macro
        println!("creating directory \"{}\"...", output_dir_path.display());
        fs::create_dir_all(output_dir_path).await.unwrap();
    }

    if raw_cli_args.urls.is_none() && raw_cli_args.from_file_path.is_none() {
        return Err(ConfigError("no urls to be downloaded".to_owned()));
    };

    let mut video_urls = Vec::new();
    if let Some(mut urls) = raw_cli_args.urls {
        video_urls.append(&mut urls);
    }
    if let Some(file_path) = raw_cli_args.from_file_path {
        let mut urls_from_file = read_urls_from_file(&file_path).await?;
        video_urls.append(&mut urls_from_file);
    }

    Ok(Config {
        video_urls,
        output_dir: raw_cli_args.output_dir,
        use_wrapper: raw_cli_args.use_wrapper,
    })
}

async fn is_youtube_dl_available() -> bool {
    process::Command::new("youtube-dl")
        .args(&["-h"])
        .output()
        .await
        .map_or_else(
            |_err| false,
            |process_output| process_output.status.success(),
        )
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
