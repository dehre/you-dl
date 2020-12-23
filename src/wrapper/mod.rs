use crate::YouDlError;
use dialoguer::Select;
use file_format::FileFormat;
use smol::process;

mod file_format;

pub async fn process_request(url: &str, output_dir: &str) -> Result<(), YouDlError> {
    let title = get_title(&url).await?;
    let available_file_formats = get_available_file_formats(&url).await?;
    let chosen_itag = ask_preferred_itag(&title, &available_file_formats);
    download(&url, &title, &chosen_itag, &output_dir).await?;
    Ok(())
}

async fn get_title(url: &str) -> Result<String, YouDlError> {
    let process_output = process::Command::new("youtube-dl")
        .args(&["--get-title", &url])
        .output()
        .await
        .map_err(|e| YouDlError::YoutubeDl(e.to_string()))
        .and_then(handle_bad_exit_status)?;

    String::from_utf8(process_output.stdout)
        .map(|title| String::from(title.trim()))
        .map_err(|e| YouDlError::Application(e.to_string()))
}

async fn get_available_file_formats(url: &str) -> Result<Vec<FileFormat>, YouDlError> {
    let process_output = process::Command::new("youtube-dl")
        .args(&["-F", &url])
        .output()
        .await
        .map_err(|e| YouDlError::YoutubeDl(e.to_string()))
        .and_then(handle_bad_exit_status)?;

    String::from_utf8(process_output.stdout)
        .map_err(|e| YouDlError::Application(e.to_string()))
        .and_then(|s| FileFormat::from_youtube_dl_stdout(&s))
}

fn ask_preferred_itag(title: &str, available_file_formats: &[FileFormat]) -> String {
    select!("choose the file format for `{}`:", title);
    let chosen_index = Select::new()
        .items(available_file_formats)
        .default(0)
        .interact()
        .unwrap();

    let itag = available_file_formats
        .get(chosen_index)
        .map(|file_format| file_format.itag.clone())
        .expect("chosen available itag");
    info!("chosen itag {} for `{}`", itag, title);
    itag
}

async fn download(url: &str, title: &str, itag: &str, output_dir: &str) -> Result<(), YouDlError> {
    info!("start downloading `{}`...", title);
    let file_path = format!("{}/%(title)s.%(ext)s", output_dir);
    process::Command::new("youtube-dl")
        .args(&["-f", itag, "-o", &file_path, &url])
        .output()
        .await
        .map_err(|e| YouDlError::YoutubeDl(e.to_string()))
        .and_then(handle_bad_exit_status)?;

    success!("successfully downloaded `{}`", title);
    Ok(())
}

fn handle_bad_exit_status(process_output: process::Output) -> Result<process::Output, YouDlError> {
    if !process_output.status.success() {
        let err = String::from_utf8(process_output.stderr)
            .map_err(|e| YouDlError::Application(e.to_string()))?;
        return Err(YouDlError::YoutubeDl(err));
    }
    Ok(process_output)
}
