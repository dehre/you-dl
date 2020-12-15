use qstring::QString;
use reqwest;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::{fs, io, process};

// Nice article: https://medium.com/javascript-in-plain-english/make-your-own-youtube-downloader-626133572429
// use ffmpeg to merge video to audio: https://davidwalsh.name/combine-audio-video

// TODO LORIS: not all videos can be downloaded

#[derive(Deserialize, Debug)]
struct Format {
    itag: i32,
    // TODO LORIS: not all formats have a url
    url: String,
}

#[derive(Deserialize, Debug)]
struct StreamingData {
    #[serde(rename(deserialize = "formats"))]
    formats: Vec<Format>,
    #[serde(rename(deserialize = "adaptiveFormats"))]
    // TODO LORIS: not always adaptive_formats is present
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
    let youtube_json = get_youtube_video_info(id)?;
    // println!("{:#?}", youtube_json);

    let title = youtube_json.video_details.title.replace("+", " ");
    let formats = youtube_json.streaming_data.adaptive_formats; // TODO LORIS: merge `formats` with `adaptive_formats`

    let video_title = format!("{}_video", title);
    download_format(&formats, 278, &video_title)?;
    let audio_title = format!("{}_audio", title);
    download_format(&formats, 140, &audio_title)?;

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

fn get_youtube_video_info(video_id: &str) -> Result<YoutubeJSON, Box<dyn Error>> {
    let url = format!(
        "https://www.youtube.com/get_video_info?video_id={}",
        video_id
    );
    let response_body = reqwest::blocking::get(&url)?.text()?;
    let player_response = QString::from(response_body.as_str())
        .get("player_response")
        .map(|s| s.to_owned())
        .ok_or("Could not find player_response")?;

    // fs::write("result.json", &json_video_info)?;

    serde_json::from_str::<YoutubeJSON>(&player_response).map_err(|e| Box::new(e) as Box<dyn Error>)
}

fn download_format(formats: &[Format], itag: i32, file_name: &str) -> Result<(), Box<dyn Error>> {
    let download_url = &formats
        .iter()
        .find(|&format| format.itag == itag)
        .ok_or("Could not find requested itag")?
        .url;
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
