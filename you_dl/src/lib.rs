use dialoguer::Select;
use qstring::QString;
use regex::Regex;
use reqwest;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;

mod models;
pub mod wrapper;
pub use models::PlayerResponse;
pub use models::YouDlError;

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

pub fn process_request(url: &str) -> Result<(), Box<dyn Error>> {
    let video_id = extract_video_id(url)?;
    let player_response = get_player_response(video_id)?;
    let downloadable_formats = extract_downloadable_formats(&player_response);
    if downloadable_formats.len() == 0 {
        // TODO LORIS: remove Box::new
        return Err(Box::new(YouDlError::UndownloadableError(
            player_response.video_details.title,
        )));
    };
    let chosen_format =
        ask_preferred_file_format(&player_response.video_details.title, &downloadable_formats)?;
    download(&chosen_format.url, &player_response.video_details.title)?;
    Ok(())
}

// TODO LORIS: less silly implementation
fn extract_video_id(url: &str) -> Result<&str, Box<dyn Error>> {
    // source: https://stackoverflow.com/questions/3452546/how-do-i-get-the-youtube-video-id-from-a-url/27728417#27728417
    let video_id =
        Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/\w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*")
            .expect("valid regex expression")
            .captures(url)
            .ok_or("Failed to capture video_id")?
            .get(1)
            .ok_or("Failed to capture video_id")?
            .as_str();
    Ok(video_id)
}

fn get_player_response(video_id: &str) -> Result<PlayerResponse, Box<dyn Error>> {
    let get_video_info_url = format!(
        "https://www.youtube.com/get_video_info?video_id={}",
        video_id
    );
    let response_body = reqwest::blocking::get(&get_video_info_url)?.text()?;
    let player_response = QString::from(response_body.as_str())
        .get("player_response")
        .map(|s| s.to_owned())
        .ok_or("Could not get player_response from youtube's API")?;

    serde_json::from_str::<PlayerResponse>(&player_response)
        .map_err(|e| Box::new(e) as Box<dyn Error>)
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
) -> Result<&'a DownloadableFormat, Box<dyn Error>> {
    println!("Choose the file format for {}:", title);
    let chosen_index = Select::new()
        .items(downloadable_formats)
        .default(0)
        .interact()?;

    let chosen_format = downloadable_formats
        .get(chosen_index)
        .expect("chosen item within range of options");
    Ok(chosen_format)
}

fn download(url: &str, destination_file_name: &str) -> Result<(), Box<dyn Error>> {
    let mut response = reqwest::blocking::get(url)?;
    let mut new_file = fs::File::create(destination_file_name)?;
    io::copy(&mut response, &mut new_file)?;
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
