use std::error::Error;
use std::fs;
use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    let links = fs::read_to_string("random_videos.txt")?;
    let links = links.lines().collect::<Vec<&str>>();

    for link in links {
        process::Command::new("youtube-dl")
            .args(&["-f", "mp4", "-o", "output/%(title)s.%(ext)s", link])
            .output()?;
    }

    Ok(())
}

// OUTLINE:
// spawn child process for each link
// args input_file and output_directory
// handle invalid links & errors from annie
// async?
// choose each video format before downloading
