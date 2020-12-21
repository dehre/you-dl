use async_compat::CompatExt;
use dialoguer::Select;
use qstring::QString;
use reqwest;
use smol::{fs, io};
use std::convert::TryFrom;
use std::path::Path;

mod models;
mod utils;
pub mod wrapper;
pub use models::PlayerResponse;
pub use models::YouDlError;

// TODO LORIS: make ui nicer, with progress bar?

// TODO LORIS: suggest search with youtube-dl if no formats are found; check if binary is present

// TODO LORIS: check this one: https://tyrrrz.me/blog/reverse-engineering-youtube -> add to README.md

// TODO LORIS: remove initial outline
// TODO LORIS: publish to homebrew

pub async fn process_request(url: &str, output_dir: &str) -> Result<(), YouDlError> {
    let video_id = utils::extract_video_id(url)?;
    let player_response = get_player_response(video_id).await?;
    let download_options = models::DownloadOptions::try_from(player_response)?;
    let chosen_option = ask_preferred_file_format(download_options);
    download(chosen_option, output_dir).await?;
    Ok(())
}

async fn get_player_response(video_id: &str) -> Result<PlayerResponse, YouDlError> {
    let get_video_info_url = format!(
        "https://www.youtube.com/get_video_info?video_id={}",
        video_id
    );

    let response_body = reqwest::get(&get_video_info_url)
        .compat()
        .await
        .map_err(|e| YouDlError::InvalidResponse(e.to_string()))?
        .text()
        .await
        .map_err(|e| YouDlError::InvalidResponse(e.to_string()))?;

    let player_response = QString::from(response_body.as_str())
        .get("player_response")
        .map(|s| s.to_owned())
        .ok_or(YouDlError::InvalidResponse(
            "missing value for player_response".to_owned(),
        ))?;

    // std::fs::write("player_response.json", player_response.as_bytes()).unwrap();
    serde_json::from_str::<PlayerResponse>(&player_response)
        .map_err(|e| YouDlError::InvalidResponse(e.to_string()))
}

fn ask_preferred_file_format(
    mut download_options: models::DownloadOptions,
) -> models::DownloadOption {
    println!(
        "Choose the file format for {}:",
        download_options.get_title().unwrap()
    );
    let chosen_index = Select::new()
        .items(&download_options.0)
        .default(0)
        .interact()
        .unwrap();

    download_options.0.remove(chosen_index)
}

async fn download(
    download_option: models::DownloadOption,
    output_dir: &str,
) -> Result<(), YouDlError> {
    let response = reqwest::get(&download_option.url)
        .compat()
        .await
        .map_err(|e| YouDlError::InvalidResponse(e.to_string()))?;
    let response_bytes = response.bytes().await.expect("valid response");
    let mut output_file = fs::File::create(Path::new(output_dir).join(format!(
        "{}.{}",
        &download_option.title, &download_option.file_extension
    )))
    .await
    .map_err(|e| YouDlError::Application(e.to_string()))?;

    io::copy(&mut &*response_bytes, &mut output_file)
        .await
        .map_err(|e| YouDlError::Application(e.to_string()))?;
    Ok(())
}
