use dialoguer::Select;
use smol::process;

mod error;
mod file_format;
pub use error::Error;
use file_format::FileFormat;

pub async fn get_title(link: &str) -> Result<String, Error> {
    let command = process::Command::new("youtube-dl")
        .args(&["--get-title", &link])
        .output()
        .await?;

    String::from_utf8(command.stdout)
        .map(|title| String::from(title.trim()))
        .map_err(Error::from)
}

pub async fn get_available_file_formats(link: &str) -> Result<Vec<FileFormat>, Error> {
    let command = process::Command::new("youtube-dl")
        .args(&["-F", &link])
        .output()
        .await?;

    String::from_utf8(command.stdout)
        .map_err(Error::from)
        .and_then(|s| FileFormat::from_youtube_dl_stdout(&s))
}

// TODO LORIS: better stdout, as table
pub async fn ask_preferred_file_format(
    title: &str,
    available_file_formats: &[FileFormat],
) -> Result<String, Error> {
    println!("Choose the file format for {}:", title);
    let chosen_format = loop {
        let chosen_format_index = Select::new()
            .items(available_file_formats)
            .default(1)
            .interact()?;

        if chosen_format_index != 0 {
            break available_file_formats[chosen_format_index].code.clone(); // TODO LORIS: error handling, no need to clone
        }
        println!("Invalid selection");
    };

    Ok(chosen_format)
}

// TODO LORIS: return successful titles and failures instead of printing here
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

    println!("Successfully downloaded {}", title); // TODO LORIS: send up title ?
    Ok(())
}
