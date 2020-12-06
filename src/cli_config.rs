use clap::{App, Arg};
use std::path::PathBuf;

const FROM_FILE_ARG: &str = "from_file";
const OUTPUT_DIR_ARG: &str = "output_dir";

#[derive(Debug)]
pub struct Config {
    pub from_file: PathBuf,
    pub output_dir: PathBuf,
}

pub fn parse() -> Config {
    let matches = App::new("youtube_downloader")
        .arg(
            Arg::new(FROM_FILE_ARG)
                .required(true)
                .short('f')
                .long("from-file")
                .value_name("PATH")
                .about("File containing URLs to download, one URL per line")
                .takes_value(true),
        )
        .arg(
            Arg::new(OUTPUT_DIR_ARG)
                .default_value("./")
                .short('o')
                .long("output-dir")
                .value_name("PATH")
                .about("Output directory")
                .takes_value(true),
        )
        .get_matches();

    let from_file = String::from(matches.value_of(FROM_FILE_ARG).unwrap());
    let output_dir = String::from(matches.value_of(OUTPUT_DIR_ARG).unwrap());

    Config {
        from_file: PathBuf::from(from_file),
        output_dir: PathBuf::from(output_dir),
    }
}
