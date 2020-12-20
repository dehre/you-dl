use async_compat::CompatExt;
use dialoguer::Select;
use qstring::QString;
use regex::Regex;
use reqwest;
use smol::{fs, io};
use std::fmt;
use std::path::Path;

mod models;
mod utils;
pub mod wrapper;
pub use models::PlayerResponse;
pub use models::YouDlError;

// TODO LORIS: video title should not have `+` as separators -> check why, if there are edge cases
// TODO LORIS: size = averageBitrate * (approxDurationMs/1000) / 8 -> and estimations as youtube-dl does
// TODO LORIS: utils directory
// TODO LORIS: impl From<Format> for DownloadableFormat, or create your own Deserializer instead
// TODO LORIS: UndownloadableError should include the reason in second field

// TODO LORIS: make ui nicer, with progress bar?

// TODO LORIS: suggest search with youtube-dl if no formats are found; check if binary is present

// TODO LORIS: check this one: https://tyrrrz.me/blog/reverse-engineering-youtube -> add to README.md

// TODO LORIS: remove initial outline
// TODO LORIS: publish to homebrew

struct DownloadableFormat {
    itag: i32,
    url: String,
    quality_label: String,
    approx_size_bytes: String,
    mime_type: String,
}

impl fmt::Display for DownloadableFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<5}{:<6}{:<12}{}",
            self.itag,
            self.quality_label,
            utils::format_file_size(&self.approx_size_bytes),
            self.mime_type
        )
    }
}

pub async fn process_request(url: &str, output_dir: &str) -> Result<(), YouDlError> {
    let video_id = extract_video_id(url)?;
    let player_response = get_player_response(video_id).await?;
    let title = &player_response.video_details.title.replace("+", " ");
    let downloadable_formats = extract_downloadable_formats(&player_response);
    if downloadable_formats.len() == 0 {
        return Err(YouDlError::UndownloadableError(title.clone()));
    };
    let chosen_format = ask_preferred_file_format(title, &downloadable_formats);
    let file_extension = utils::get_file_extension(chosen_format.itag).unwrap_or("");
    download(&chosen_format.url, output_dir, title, file_extension).await?;
    Ok(())
}

fn extract_video_id(url: &str) -> Result<&str, YouDlError> {
    // source: https://stackoverflow.com/questions/3452546/how-do-i-get-the-youtube-video-id-from-a-url/27728417#27728417
    let video_id =
        Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/\w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*")
            .expect("valid regex expression")
            .captures(url)
            .ok_or(YouDlError::InvalidURLError(url.to_owned()))?
            .get(1)
            .ok_or(YouDlError::InvalidURLError(url.to_owned()))?
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
        .map_err(|e| YouDlError::YoutubeAPIError(e.to_string()))?
        .text()
        .await
        .map_err(|e| YouDlError::YoutubeAPIError(e.to_string()))?;

    let player_response = QString::from(response_body.as_str())
        .get("player_response")
        .map(|s| s.to_owned())
        .ok_or(YouDlError::YoutubeAPIError(
            "missing player_response".to_owned(),
        ))?;

    std::fs::write("player_response.json", player_response.as_bytes()).unwrap();

    serde_json::from_str::<PlayerResponse>(&player_response)
        .map_err(|e| YouDlError::YoutubeAPIError(e.to_string()))
}

fn extract_downloadable_formats(player_response: &PlayerResponse) -> Vec<DownloadableFormat> {
    if player_response.streaming_data.is_none() {
        return Vec::new();
    }
    let formats = &player_response.streaming_data.as_ref().unwrap().formats;
    formats
        .iter()
        .filter(|&format| format.url.is_some())
        .map(|format| format.clone())
        .map(|format| {
            let approx_duration_ms = format.approx_duration_ms.parse::<i32>().unwrap(); // TODO LORIS
            let approx_size_bytes = format.bitrate * (approx_duration_ms / 1000) / 8;
            let approx_size_bytes = approx_size_bytes.to_string();
            return DownloadableFormat {
                itag: format.itag,
                url: format.url.unwrap(),
                quality_label: format.quality_label,
                approx_size_bytes,
                mime_type: format.mime_type,
            };
        })
        .collect()
}

fn ask_preferred_file_format<'a>(
    title: &str,
    downloadable_formats: &'a [DownloadableFormat],
) -> &'a DownloadableFormat {
    println!("Choose the file format for {}:", title);
    let chosen_index = Select::new()
        .items(downloadable_formats)
        .default(0)
        .interact()
        .unwrap();

    downloadable_formats
        .get(chosen_index)
        .expect("chosen item within range of options")
}

async fn download(
    url: &str,
    output_dir: &str,
    output_file_name: &str,
    file_extension: &str,
) -> Result<(), YouDlError> {
    let response = reqwest::get(url).compat().await.map_err(|e| {
        YouDlError::YoutubeAPIError(format!("invalid download_url {}: {}", url, e.to_string()))
    })?;
    let response_bytes = response.bytes().await.expect("valid response");

    let mut file = fs::File::create(
        Path::new(output_dir).join(format!("{}.{}", output_file_name, file_extension)),
    )
    .await
    .map_err(|_| YouDlError::UserError("invalid output directory provided".to_owned()))?;

    io::copy(&mut &*response_bytes, &mut file).await?;
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
