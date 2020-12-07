use dialoguer::Select;
use futures::FutureExt;
use smol::process;

mod error;
mod file_format;
pub use error::Error;
use file_format::FileFormat;

pub async fn get_title(link: &str) -> Result<String, Error> {
    let process_output = process::Command::new("youtube-dl")
        .args(&["--get-title", &link])
        .output()
        .map(|result| result.map(handle_bad_exit_status)?)
        .await?;

    String::from_utf8(process_output.stdout)
        .map(|title| String::from(title.trim()))
        .map_err(Error::from)
}

pub async fn get_available_file_formats(link: &str) -> Result<Vec<FileFormat>, Error> {
    let process_output = process::Command::new("youtube-dl")
        .args(&["-F", &link])
        .output()
        .map(|result| result.map(handle_bad_exit_status)?)
        .await?;

    String::from_utf8(process_output.stdout)
        .map_err(Error::from)
        .and_then(|s| FileFormat::from_youtube_dl_stdout(&s))
}

pub async fn ask_preferred_file_format(
    title: &str,
    available_file_formats: &[FileFormat],
) -> Result<String, Error> {
    println!("Choose the file format for {}:", title);
    let chosen_index = Select::new()
        .items(available_file_formats)
        .default(0)
        .interact()?;

    available_file_formats
        .get(chosen_index)
        .map(|file_format| file_format.code.clone())
        .ok_or(Error::ApplicationError(format!(
            "Invalid file format chosen: index {} in len {}",
            chosen_index,
            available_file_formats.len()
        )))
}

pub async fn download_video(
    link: &str,
    title: &str,
    format: &str,
    output_dir: &str,
) -> Result<(), Error> {
    println!("Start downloading {}...", title);
    let file_path = format!("{}/%(title)s.%(ext)s", output_dir);
    process::Command::new("youtube-dl")
        .args(&["-f", format, "-o", &file_path, link])
        .output()
        .map(|result| result.map(handle_bad_exit_status)?)
        .await?;

    println!("Successfully downloaded {}", title);
    Ok(())
}

fn handle_bad_exit_status(process_output: process::Output) -> Result<process::Output, Error> {
    if !process_output.status.success() {
        let err = String::from_utf8(process_output.stderr)?;
        return Err(Error::YoutubeDlError(err));
    }
    Ok(process_output)
}
