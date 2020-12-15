use qstring::QString;
use reqwest;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::{fs, io, process};

// Nice article: https://medium.com/javascript-in-plain-english/make-your-own-youtube-downloader-626133572429
// use ffmpeg to merge video to audio: https://davidwalsh.name/combine-audio-video

#[derive(Deserialize, Debug)]
struct Format {
    itag: i32,
    url: String,
}

#[derive(Deserialize, Debug)]
struct StreamingData {
    #[serde(rename(deserialize = "formats"))]
    formats: Vec<Format>,

    #[serde(rename(deserialize = "adaptiveFormats"))]
    adaptive_formats: Vec<Format>,
}

#[derive(Deserialize, Debug)]
struct VideoDetails {
    #[serde(rename(deserialize = "videoId"))]
    video_id: String,

    #[serde(rename(deserialize = "title"))]
    title: String,
}

#[derive(Deserialize, Debug)]
struct YoutubeJSON {
    #[serde(rename(deserialize = "streamingData"))]
    streaming_data: StreamingData,

    #[serde(rename(deserialize = "videoDetails"))]
    video_details: VideoDetails,
}

fn main() -> Result<(), Box<dyn Error>> {
    let id = "W6-7jKGxNYk";
    let video_url = format!("https://www.youtube.com/get_video_info?video_id={}", id);

    let response = reqwest::blocking::get(&video_url)?;
    let response_body = response.text()?;
    let json_video_info = QString::from(response_body.as_str())
        .get("player_response")
        .and_then(|s| Some(s.to_owned()))
        .unwrap();

    // fs::write("result.json", &json_video_info)?;

    let youtube_json: YoutubeJSON = match serde_json::from_str(&json_video_info) {
        Ok(v) => v,
        Err(e) => {
            panic!("{}", format!("Failed to serialize: {}", e));
        }
    };

    // println!("{:#?}", youtube_json);

    let title = youtube_json.video_details.title.replace("+", " ");
    let formats = youtube_json.streaming_data.adaptive_formats;

    let video_download_url = &formats
        .iter()
        .find(|&format| format.itag == 278)
        .unwrap()
        .url;
    println!("Video download url: {:?}", video_download_url);

    let mut video_response = reqwest::blocking::get(video_download_url.as_str())?;
    let video_title = format!("{}_video.webm", title);
    let mut video_file = fs::File::create(&video_title).unwrap();
    io::copy(&mut video_response, &mut video_file).unwrap();

    let audio_download_url = &formats
        .iter()
        .find(|&format| format.itag == 140)
        .unwrap()
        .url;
    println!("Audio download url: {:?}", video_download_url);

    let mut audio_response = reqwest::blocking::get(audio_download_url.as_str())?;
    let audio_title = format!("{}_audio.webm", title);
    let mut audio_new_file = fs::File::create(&audio_title).unwrap();
    io::copy(&mut audio_response, &mut audio_new_file).unwrap();

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

    Ok(())
}
