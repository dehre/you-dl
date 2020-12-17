use dialoguer::Select;
use qstring::QString;
use regex::Regex;
use reqwest;
use std::fmt;
use std::fs;
use std::io;
use std::path::Path;

mod models;
pub mod wrapper;
pub use models::PlayerResponse;
pub use models::YouDlError;

// TODO LORIS: make everything async
// TODO LORIS: extract file format for downloaded video

// TODO LORIS: impl From<Format> for DownloadableFormat
struct DownloadableFormat {
    itag: i32,
    url: String,
    quality_label: String,
    mime_type: String,
}

impl fmt::Display for DownloadableFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:6}{:8}{}",
            self.itag, self.quality_label, self.mime_type
        )
    }
}

pub fn process_request(url: &str, output_dir: &str) -> Result<(), YouDlError> {
    let video_id = extract_video_id(url)?;
    let player_response = get_player_response(video_id)?;
    let title = &player_response.video_details.title;
    let downloadable_formats = extract_downloadable_formats(&player_response);
    if downloadable_formats.len() == 0 {
        return Err(YouDlError::UndownloadableError(title.clone()));
    };
    let chosen_format = ask_preferred_file_format(title, &downloadable_formats);
    download(&chosen_format.url, output_dir, title)?;
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

fn get_player_response(video_id: &str) -> Result<PlayerResponse, YouDlError> {
    let get_video_info_url = format!(
        "https://www.youtube.com/get_video_info?video_id={}",
        video_id
    );
    let response_body = reqwest::blocking::get(&get_video_info_url)
        .and_then(|response| response.text())
        .map_err(|e| YouDlError::YoutubeAPIError(e.to_string()))?;
    let player_response = QString::from(response_body.as_str())
        .get("player_response")
        .map(|s| s.to_owned())
        .ok_or(YouDlError::YoutubeAPIError(
            "missing player_response".to_owned(),
        ))?;

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
        .map(|format| DownloadableFormat {
            itag: format.itag,
            url: format.url.unwrap(),
            quality_label: format.quality_label,
            mime_type: format.mime_type,
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

fn download(url: &str, output_dir: &str, output_file_name: &str) -> Result<(), YouDlError> {
    let mut response = reqwest::blocking::get(url).map_err(|e| {
        YouDlError::YoutubeAPIError(format!("invalid download_url {}: {}", url, e.to_string()))
    })?;
    let mut file = fs::File::create(Path::new(output_dir).join(output_file_name))
        .map_err(|_| YouDlError::UserError("invalid output directory provided".to_owned()))?;

    io::copy(&mut response, &mut file)?;
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
