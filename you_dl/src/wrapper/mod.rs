use super::YouDlError;
use dialoguer::Select;
use file_format::FileFormat;
use futures::FutureExt;
use smol::process;

mod file_format;

pub async fn process_request(url: &str, output_dir: &str) -> Result<(), YouDlError> {
    let title = get_title(&url).await?;
    let available_file_formats = get_available_file_formats(&url).await?;
    let chosen_file_format = ask_preferred_file_format(&title, &available_file_formats).await?;
    download_video(&url, &title, &chosen_file_format, &output_dir).await?;
    Ok(())
}

async fn get_title(url: &str) -> Result<String, YouDlError> {
    let process_output = process::Command::new("youtube-dl")
        .args(&["--get-title", &url])
        .output()
        .map(|result| result.map(handle_bad_exit_status)?)
        .await?;

    String::from_utf8(process_output.stdout)
        .map(|title| String::from(title.trim()))
        .map_err(YouDlError::from)
}

async fn get_available_file_formats(url: &str) -> Result<Vec<FileFormat>, YouDlError> {
    let process_output = process::Command::new("youtube-dl")
        .args(&["-F", &url])
        .output()
        .map(|result| result.map(handle_bad_exit_status)?)
        .await?;

    String::from_utf8(process_output.stdout)
        .map_err(YouDlError::from)
        .and_then(|s| FileFormat::from_youtube_dl_stdout(&s))
}

async fn ask_preferred_file_format(
    title: &str,
    available_file_formats: &[FileFormat],
) -> Result<String, YouDlError> {
    println!("Choose the file format for {}:", title);
    let chosen_index = Select::new()
        .items(available_file_formats)
        .default(0)
        .interact()?;

    available_file_formats
        .get(chosen_index)
        .map(|file_format| file_format.code.clone())
        .ok_or(YouDlError::ApplicationError(format!(
            "Invalid file format chosen: index {} in len {}",
            chosen_index,
            available_file_formats.len()
        )))
}

async fn download_video(
    url: &str,
    title: &str,
    format: &str,
    output_dir: &str,
) -> Result<(), YouDlError> {
    println!("Start downloading {}...", title);
    let file_path = format!("{}/%(title)s.%(ext)s", output_dir);
    process::Command::new("youtube-dl")
        .args(&["-f", format, "-o", &file_path, &url])
        .output()
        .map(|result| result.map(handle_bad_exit_status)?)
        .await?;

    println!("Successfully downloaded {}", title);
    Ok(())
}

fn handle_bad_exit_status(process_output: process::Output) -> Result<process::Output, YouDlError> {
    if !process_output.status.success() {
        let err = String::from_utf8(process_output.stderr)?;
        return Err(YouDlError::YoutubeDlError(err));
    }
    Ok(process_output)
}
