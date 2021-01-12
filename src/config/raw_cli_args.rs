use super::ConfigError;
use clap::{crate_version, App, Arg};

#[derive(Debug)]
pub struct RawCliArgs {
    pub help_message: String,
    pub from_file_path: Option<String>,
    pub output_dir: String,
    pub urls: Option<Vec<String>>,
    pub use_wrapper: bool,
}

const FROM_FILE_PATH_ARG: &str = "from-file-path";
const OUTPUT_DIR_ARG: &str = "output-dir";
const URL_ARG: &str = "url";
const USE_WRAPPER_ARG: &str = "wrapper";

pub fn parse() -> Result<RawCliArgs, ConfigError> {
    let mut app = App::new("you-dl")
        .version(crate_version!())
        .arg(
            Arg::new(URL_ARG)
                .value_name("URL")
                .index(1)
                .multiple(true)
                .about("Url(s) to download"),
        )
        .arg(
            Arg::new(USE_WRAPPER_ARG)
                .short('w')
                .long("wrapper")
                .long_about(
                    "\
Not all urls are currently supported by you-dl.
If you have \"youtube-dl\" installed on your machine, you can retry with the \"-w\" flag: \"you-dl -w <url>...\".
For more info, check \"github.com/ytdl-org/youtube-dl\".",
                )
                .takes_value(false),
        )
        .arg(
            Arg::new(FROM_FILE_PATH_ARG)
                .short('f')
                .long("from-file")
                .value_name("PATH")
                .about("Read the URLs from a text file (lines starting with `#` and `//` are ignored)")
                .takes_value(true),
        )
        .arg(
            Arg::new(OUTPUT_DIR_ARG)
                .default_value(".")
                .short('o')
                .long("output-dir")
                .value_name("PATH")
                .about("Change output directory")
                .takes_value(true),
        );

    let help_message = get_help_message(&mut app);
    let matches = app.get_matches();
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
    let use_wrapper = matches.is_present(USE_WRAPPER_ARG);

    Ok(RawCliArgs {
        help_message,
        from_file_path,
        output_dir,
        urls,
        use_wrapper,
    })
}

fn get_help_message(app: &mut clap::App) -> String {
    let mut bytes_vector = Vec::new();
    app.write_help(&mut bytes_vector)
        .expect("failed to create help message");
    String::from_utf8(bytes_vector).expect("failed to convert bytes_vector into String")
}
