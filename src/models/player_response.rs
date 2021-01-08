use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct PlayerResponse {
    #[serde(rename(deserialize = "streamingData"))]
    pub streaming_data: Option<StreamingData>,
    #[serde(rename(deserialize = "videoDetails"))]
    pub video_details: VideoDetails,
}

#[derive(Deserialize, Debug)]
pub struct VideoDetails {
    #[serde(rename(deserialize = "videoId"))]
    pub video_id: String,
    #[serde(rename(deserialize = "title"))]
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct StreamingData {
    #[serde(rename(deserialize = "formats"))]
    pub formats: Vec<Format>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Format {
    pub itag: i32,
    pub url: Option<String>,
    // TODO LORIS: remove quality_label field
    #[serde(rename(deserialize = "qualityLabel"))]
    pub quality_label: String,
    #[serde(rename(deserialize = "width"))]
    pub width: i32,
    #[serde(rename(deserialize = "height"))]
    pub height: i32,
    #[serde(rename(deserialize = "mimeType"))]
    pub mime_type: String,
    #[serde(rename(deserialize = "bitrate"))]
    pub bitrate: i32,
    #[serde(rename(deserialize = "approxDurationMs"))]
    pub approx_duration_ms: String,
}
