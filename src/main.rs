use smol::fs;
use smol::process;
use std::error::Error;
use std::io;
mod config;

fn main() {
    if let Err(e) = smol::block_on(async_main()) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    let config = config::parse_from_env()?;
    let str_links = fs::read_to_string(&config.input_file).await?;
    let links: Vec<&str> = str_links
        .lines()
        .filter(|&l| !l.trim().is_empty())
        .collect();

    let smol_tasks: Vec<_> = links
        .iter()
        .map(|&link| {
            smol::spawn(download_video(
                String::from(link),
                config.output_directory.clone(),
            ))
        })
        .collect();

    // TODO LORIS: collect io::Errors here and send them up?
    futures::future::join_all(smol_tasks).await;

    Ok(())
}

//
// The function download_video returns an error only if this specific application has an error, not if youtube-dl failed.
//
async fn download_video(link: String, output_directory: String) -> Result<(), io::Error> {
    println!("START DOWNLOADING");
    let youtube_dl_output = format!("{}/%(title)s.%(ext)s", output_directory);
    let output = process::Command::new("youtube-dl")
        .args(&["-f", "mp4", "-o", &youtube_dl_output, &link])
        .output()
        .await?;

    if !output.status.success() {
        eprintln!("Failed to download: {}", link);
        return Ok(());
    }

    let raw_title = process::Command::new("youtube-dl")
        .args(&["--get-title", &link])
        .output()
        .await?
        .stdout;
    let title = String::from_utf8(raw_title).unwrap();
    println!("Successfully downloaded {}", title);

    Ok(())
}

// OUTLINE:
// spawn child process for each link
// args input_file and output_directory
// handle invalid links & errors from youtube-dl
// async wait for output?
// choose each video format before downloading
// proper cli library?
