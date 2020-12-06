use dialoguer::Select;
use smol::process;

mod error;
pub use error::Error;

pub async fn get_title(link: &str) -> Result<String, Error> {
    let command = process::Command::new("youtube-dl")
        .args(&["--get-title", &link])
        .output()
        .await?;
    let title = String::from_utf8(command.stdout)?;
    Ok(String::from(title.trim()))
}

pub async fn get_available_file_formats(link: &str) -> Result<Vec<String>, Error> {
    let command = process::Command::new("youtube-dl")
        .args(&["-F", &link])
        .output()
        .await?;
    let command_stdout = String::from_utf8(command.stdout)?;
    let available_file_formats = command_stdout
        .lines()
        .filter(|&line| !line.starts_with('['))
        .map(String::from)
        .collect();
    Ok(available_file_formats)
}

pub async fn ask_preferred_file_format(
    title: &str,
    available_file_formats: &[String],
) -> Result<String, Error> {
    println!("Choose the file format for {}:", title);
    let chosen_format = loop {
        let chosen_line_index = Select::new()
            .items(available_file_formats)
            .default(1)
            .interact()?;
        let chosen_line =
            available_file_formats
                .get(chosen_line_index)
                .ok_or(Error::ApplicationError(String::from(
                    "failed to get chosen line",
                )))?;
        let chosen_format =
            chosen_line
                .split_whitespace()
                .next()
                .ok_or(Error::ApplicationError(String::from(
                    "failed to get format from chosen line",
                )))?;
        if let Ok(_) = chosen_format.parse::<i32>() {
            break chosen_format;
        }
        println!("Invalid selection");
    };

    Ok(String::from(chosen_format))
}

pub async fn download_video(
    link: &str,
    title: &str,
    format: &str,
    output_dir: &str,
) -> Result<(), Error> {
    println!("Start downloading {}...", title);
    let file_path = format!("{}/%(title)s.%(ext)s", output_dir);
    let command = process::Command::new("youtube-dl")
        .args(&["-f", format, "-o", &file_path, link])
        .output()
        .await?;

    if !command.status.success() {
        let err = String::from_utf8(command.stderr)?;
        eprintln!("Failed to download {}: {}", title, err); // TODO LORIS ?
        return Ok(());
    }

    println!("Successfully downloaded {}", title);
    Ok(())
}
