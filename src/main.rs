use smol::process as smol_process;
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
    let str_links = fs::read_to_string(&config.input_file)?;
    let links = str_links
        .lines()
        .filter(|&l| !l.trim().is_empty())
        .collect::<Vec<&str>>();

    smol::block_on(async move {
        let smol_tasks: Vec<_> = links
            .iter()
            .map(|&link| {
                smol::spawn(download_video(
                    String::from(link),
                    config.output_directory.clone(),
                ))
            })
            .collect();
        futures::future::join_all(smol_tasks).await;
    });

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

// TODO LORIS: return Result<(), Box<dyn Error>> ?
async fn download_video(link: String, output_directory: String) {
    println!("START DOWNLOADING");
    let youtube_dl_output = format!("{}/%(title)s.%(ext)s", output_directory);
    let output = smol_process::Command::new("youtube-dl")
        .args(&["-f", "mp4", "-o", &youtube_dl_output, &link])
        .output()
        .await
        .unwrap();

    if !output.status.success() {
        eprintln!("Failed to download: {}", link);
        return;
    }

    let raw_title = smol_process::Command::new("youtube-dl")
        .args(&["--get-title", &link])
        .output()
        .await
        .unwrap()
        .stdout;
    let title = String::from_utf8(raw_title).unwrap();
    println!("Successfully downloaded: {}", title.trim());
}

// OUTLINE:
// spawn child process for each link
// args input_file and output_directory
// handle invalid links & errors from youtube-dl
// async wait for output?
// choose each video format before downloading
// proper cli library?
