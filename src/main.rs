use std::env;
use std::error::Error;
use std::fs;
use std::process;

#[derive(Debug)]
struct Config {
    input_file: String,
    output_directory: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = parse_config_from_env();
    let str_links = fs::read_to_string(config.input_file)?;
    let links = str_links
        .lines()
        .filter(|&l| !l.trim().is_empty())
        .collect::<Vec<&str>>();

    for link in links {
        let youtube_dl_output = format!("{}/%(title)s.%(ext)s", config.output_directory);
        process::Command::new("youtube-dl")
            .args(&["-f", "mp4", "-o", &youtube_dl_output, link])
            .output()?;
    }

    Ok(())
}

fn parse_config_from_env() -> Config {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Invalid arguments. Usage: youtube_downloader <input_file> <output_directory>");
        process::exit(1);
    }

    Config {
        input_file: args.remove(1),
        output_directory: args.remove(1),
    }
}

// OUTLINE:
// spawn child process for each link
// args input_file and output_directory
// handle invalid links & errors from youtube-dl
// async wait for output?
// choose each video format before downloading
// proper cli library?
