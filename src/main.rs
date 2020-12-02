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
            smol::spawn(process_request(
                String::from(link),
                config.output_directory.clone(),
            ))
        })
        .collect();

    // TODO LORIS: collect io::Errors here and send them up?
    futures::future::join_all(smol_tasks).await;

    Ok(())
}

async fn process_request(link: String, output_directory: String) -> Result<(), io::Error> {
    let title = get_title(&link)?;
    let available_file_formats = get_available_file_formats(&link)?;
    let chosen_file_format = ask_preferred_file_format(&title, &available_file_formats)?;
    download_video(&link, &title, &chosen_file_format, &output_directory).await?;
    Ok(())
}

fn get_title(link: &str) -> Result<String, io::Error> {
    let command = std::process::Command::new("youtube-dl")
        .args(&["--get-title", &link])
        .output()?;
    let title = String::from_utf8(command.stdout).unwrap();
    Ok(String::from(title.trim()))
}

fn get_available_file_formats(link: &str) -> Result<Vec<String>, io::Error> {
    let command = std::process::Command::new("youtube-dl")
        .args(&["-F", &link])
        .output()?;
    let stdout = String::from_utf8(command.stdout).unwrap();
    let available_file_formats = stdout
        .lines()
        .filter(|&line| !line.starts_with('['))
        .map(String::from)
        .collect();
    Ok(available_file_formats)
}

fn ask_preferred_file_format(
    title: &str,
    available_file_formats: &[String],
) -> Result<String, io::Error> {
    println!("Please choose the preferred file format for {}:", title);
    println!("\t{}", available_file_formats.join("\n\t"));
    // TODO LORIS: show some sort of prompt
    // println!("> ");
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice)?;
    Ok(String::from(user_choice.trim()))
}

async fn download_video(
    link: &str,
    title: &str,
    format: &str,
    output_directory: &str,
) -> Result<(), io::Error> {
    println!("Start downloading {} with format {}!\n\n", title, format);
    let file_path = format!("{}/%(title)s.%(ext)s", output_directory);
    let command = process::Command::new("youtube-dl")
        .args(&["-f", format, "-o", &file_path, link])
        .output()
        .await?;

    if !command.status.success() {
        let err = String::from_utf8(command.stderr).unwrap();
        eprintln!("Failed to download {}: {}", title, err);
        return Ok(());
    }

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
// cursor to choose file format?
