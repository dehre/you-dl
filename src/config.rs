use clap::{App, Arg};
use smol::fs;
use std::error::Error;
use std::fmt;

pub struct Config {
    pub video_urls: Vec<String>,
    pub output_dir: String,
}

#[derive(Debug)]
pub struct ParseConfigError(String);

impl fmt::Display for ParseConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid program arguments provided: {}", self.0)
    }
}

impl Error for ParseConfigError {}

pub async fn parse() -> Result<Config, ParseConfigError> {
    let cli_args = parse_cli_args()?;
    if cli_args.urls.is_none() && cli_args.from_file_path.is_none() {
        return Err(ParseConfigError(String::from("no urls to be downloaded")));
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
    })
}

async fn read_urls_from_file(file_path: &str) -> Result<Vec<String>, ParseConfigError> {
    let file_contents = fs::read_to_string(file_path).await.map_err(|err| {
        ParseConfigError(format!(
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

const FROM_FILE_ARG: &str = "from-file";
const OUTPUT_DIR_ARG: &str = "output-dir";
const URL_ARG: &str = "url";

#[derive(Debug)]
struct CliArgs {
    from_file_path: Option<String>,
    output_dir: String,
    urls: Option<Vec<String>>,
}

fn parse_cli_args() -> Result<CliArgs, ParseConfigError> {
    let matches = App::new("youtube_downloader")
        .arg(
            Arg::new(URL_ARG)
                .value_name("URL")
                .index(1)
                .multiple(true)
                .about("Url(s) to download"),
        )
        .arg(
            Arg::new(FROM_FILE_ARG)
                .short('f')
                .long("from-file")
                .value_name("PATH")
                .about("File containing URLs to download, one URL per line")
                .takes_value(true),
        )
        .arg(
            Arg::new(OUTPUT_DIR_ARG)
                .default_value(".")
                .short('o')
                .long("output-dir")
                .value_name("PATH")
                .about("Output directory")
                .takes_value(true),
        )
        .get_matches();

    let from_file_path = matches.value_of(FROM_FILE_ARG).map(|s| s.to_owned());
    let output_dir = matches.value_of(OUTPUT_DIR_ARG).unwrap().to_owned();
    let urls = matches
        .values_of(URL_ARG)
        .map(|values| values.collect::<Vec<&str>>())
        .map(|urls| {
            urls.iter()
                .map(|&url| url.to_owned())
                .collect::<Vec<String>>()
        });

    Ok(CliArgs {
        from_file_path,
        output_dir,
        urls,
    })
}
