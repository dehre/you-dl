use super::ConfigError;
use clap::{App, Arg};

#[derive(Debug)]
pub struct CliArgs {
    pub from_file_path: Option<String>,
    pub output_dir: String,
    pub urls: Option<Vec<String>>,
}

const FROM_FILE_PATH_ARG: &str = "from-file-path";
const OUTPUT_DIR_ARG: &str = "output-dir";
const URL_ARG: &str = "url";

pub fn parse_cli_args() -> Result<CliArgs, ConfigError> {
    let matches = App::new("youtube_downloader")
        .arg(
            Arg::new(URL_ARG)
                .value_name("URL")
                .index(1)
                .multiple(true)
                .about("Url(s) to download"),
        )
        .arg(
            Arg::new(FROM_FILE_PATH_ARG)
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

    let from_file_path = matches.value_of(FROM_FILE_PATH_ARG).map(|s| s.to_owned());
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
