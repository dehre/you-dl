use qstring::QString;
use reqwest;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::{fs, io, process};

// TODO LORIS: find out what's the best audio to download for the selected video, and make sure Quicktime can always play it.

// Nice article: https://medium.com/javascript-in-plain-english/make-your-own-youtube-downloader-626133572429
// use ffmpeg to merge video to audio: https://davidwalsh.name/combine-audio-video
// useful ffmpeg commands: https://www.labnol.org/internet/useful-ffmpeg-commands/28490/
// useful stackoverflow for ffmpeg: https://stackoverflow.com/questions/11779490/how-to-add-a-new-audio-not-mixing-into-a-video-using-ffmpeg
// https://opensource.com/article/17/6/ffmpeg-convert-media-file-formats

#[derive(Deserialize, Debug)]
struct Format {
    itag: i32,
    url: Option<String>,
    #[serde(rename(deserialize = "qualityLabel"))]
    quality_label: String,
    #[serde(rename(deserialize = "mimeType"))]
    mime_type: String,
}

#[derive(Deserialize, Debug)]
struct StreamingData {
    #[serde(rename(deserialize = "formats"))]
    formats: Vec<Format>,
}

#[derive(Deserialize, Debug)]
struct VideoDetails {
    #[serde(rename(deserialize = "videoId"))]
    video_id: String,
    #[serde(rename(deserialize = "title"))]
    title: String,
}

#[derive(Deserialize, Debug)]
struct PlayerResponse {
    #[serde(rename(deserialize = "streamingData"))]
    streaming_data: Option<StreamingData>,
    #[serde(rename(deserialize = "videoDetails"))]
    video_details: VideoDetails,
}

fn main() -> Result<(), Box<dyn Error>> {
    let id = "0X5SjKh1n34";
    let player_response = get_player_response(id)?;
    // println!("{:#?}", player_response);

    let title = player_response.video_details.title.replace("+", " ");
    let StreamingData { formats } = player_response
        .streaming_data
        .ok_or("video not available for downloading")?;

    let video_title = format!("{}_video", title);
    download_format(&formats, 278, &video_title)?;
    let audio_title = format!("{}_audio", title);
    download_format(&formats, 140, &audio_title)?;

    // COMMAND: ffmpeg -i <video> -i <audio> -c:v copy -c:a copy <output>
    let output_title = format!("{}.mp4", title);
    process::Command::new("ffmpeg")
        .args(&[
            "-i",
            &video_title,
            "-i",
            &audio_title,
            "-c:v",
            "copy",
            "-c:a",
            "copy",
            &output_title,
        ])
        .output()?;

    clean_up_artifacts(&[&video_title, &audio_title])?;
    Ok(())
}

fn get_player_response(video_id: &str) -> Result<PlayerResponse, Box<dyn Error>> {
    let url = format!(
        "https://www.youtube.com/get_video_info?video_id={}",
        video_id
    );
    let response_body = reqwest::blocking::get(&url)?.text()?;
    let player_response = QString::from(response_body.as_str())
        .get("player_response")
        .map(|s| s.to_owned())
        .ok_or("Could not find player_response")?;

    // fs::write("raw_player_response.json", &player_response)?;

    serde_json::from_str::<PlayerResponse>(&player_response)
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}

fn download_format(formats: &[Format], itag: i32, file_name: &str) -> Result<(), Box<dyn Error>> {
    let &download_url = &formats
        .iter()
        .find(|&format| format.itag == itag)
        .expect("Could not find requested itag")
        .url
        .as_ref()
        .ok_or("video not available for downloading")?;

    // println!("download url: {:?}", download_url);

    let mut http_response = reqwest::blocking::get(download_url.as_str())?;
    let mut new_file = fs::File::create(file_name)?;
    io::copy(&mut http_response, &mut new_file)?;

    Ok(())
}

fn clean_up_artifacts(file_names: &[&str]) -> Result<(), Box<dyn Error>> {
    for &file_name in file_names {
        fs::remove_file(file_name)?;
    }
    Ok(())
}
