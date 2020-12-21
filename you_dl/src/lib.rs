use async_compat::CompatExt;
use dialoguer::Select;
use qstring::QString;
use regex::Regex;
use reqwest;
use smol::{fs, io};
use std::convert::TryFrom;
use std::path::Path;

mod models;
mod utils;
pub mod wrapper;
pub use models::PlayerResponse;
pub use models::YouDlError;

// TODO LORIS: better use of YouDl Errors

// TODO LORIS: make ui nicer, with progress bar?

// TODO LORIS: suggest search with youtube-dl if no formats are found; check if binary is present

// TODO LORIS: check this one: https://tyrrrz.me/blog/reverse-engineering-youtube -> add to README.md

// TODO LORIS: remove initial outline
// TODO LORIS: publish to homebrew

pub async fn process_request(url: &str, output_dir: &str) -> Result<(), YouDlError> {
    let video_id = extract_video_id(url)?;
    let player_response = get_player_response(video_id).await?;
    let download_options = models::DownloadOptions::try_from(player_response)?;
    let chosen_option = ask_preferred_file_format(download_options);
    download(chosen_option, output_dir).await?;
    Ok(())
}

// TODO LORIS: into utils directory
fn extract_video_id(url: &str) -> Result<&str, YouDlError> {
    // source: https://stackoverflow.com/questions/3452546/how-do-i-get-the-youtube-video-id-from-a-url/27728417#27728417
    let video_id =
        Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/\w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*")
            .expect("valid regex expression")
            .captures(url)
            .ok_or(YouDlError::InvalidURL(url.to_owned()))?
            .get(1)
            .ok_or(YouDlError::InvalidURL(url.to_owned()))?
            .as_str();
    Ok(video_id)
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
    .map_err(|_| YouDlError::User("invalid output directory provided".to_owned()))?; // TODO LORIS: move this check when parsing args

    io::copy(&mut &*response_bytes, &mut output_file)
        .await
        .map_err(|e| YouDlError::Application(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_video_id_from_url() {
        let urls = [
            "//www.youtube-nocookie.com/embed/dQw4w9WgXcQ?rel=0",
            "http://www.youtube.com/user/Scobleizer#p/u/1/dQw4w9WgXcQ",
            "http://www.youtube.com/watch?v=dQw4w9WgXcQ&feature=channel",
            "http://www.youtube.com/watch?v=dQw4w9WgXcQ&playnext_from=TL&videos=osPknwzXEas&feature=sub",
            "http://www.youtube.com/ytscreeningroom?v=dQw4w9WgXcQ",
            "http://www.youtube.com/user/SilkRoadTheatre#p/a/u/2/dQw4w9WgXcQ",
            "http://youtu.be/dQw4w9WgXcQ",
            "http://www.youtube.com/watch?v=dQw4w9WgXcQ&feature=youtu.be",
            "http://youtu.be/dQw4w9WgXcQ",
            "http://www.youtube.com/user/Scobleizer#p/u/1/dQw4w9WgXcQ?rel=0",
            "http://www.youtube.com/watch?v=dQw4w9WgXcQ&feature=channel",
            "http://www.youtube.com/watch?v=dQw4w9WgXcQ&playnext_from=TL&videos=osPknwzXEas&feature=sub",
            "http://www.youtube.com/ytscreeningroom?v=dQw4w9WgXcQ",
            "http://www.youtube.com/embed/dQw4w9WgXcQ?rel=0",
            "http://www.youtube.com/watch?v=dQw4w9WgXcQ",
            "http://youtube.com/v/dQw4w9WgXcQ?feature=youtube_gdata_player",
            "http://youtube.com/vi/dQw4w9WgXcQ?feature=youtube_gdata_player",
            "http://youtube.com/?v=dQw4w9WgXcQ&feature=youtube_gdata_player",
            "http://www.youtube.com/watch?v=dQw4w9WgXcQ&feature=youtube_gdata_player",
            "http://youtube.com/?vi=dQw4w9WgXcQ&feature=youtube_gdata_player",
            "http://youtube.com/watch?v=dQw4w9WgXcQ&feature=youtube_gdata_player",
            "http://youtube.com/watch?vi=dQw4w9WgXcQ&feature=youtube_gdata_player",
            "http://youtu.be/dQw4w9WgXcQ?feature=youtube_gdata_player",
        ];

        for &url in &urls {
            let video_id = extract_video_id(url).unwrap();
            assert_eq!(video_id, "dQw4w9WgXcQ");
        }
    }
}
