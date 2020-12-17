use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Format {
    pub itag: i32,
    pub url: Option<String>,
    #[serde(rename(deserialize = "qualityLabel"))]
    pub quality_label: String,
    #[serde(rename(deserialize = "mimeType"))]
    pub mime_type: String,
}

#[derive(Deserialize, Debug)]
pub struct StreamingData {
    #[serde(rename(deserialize = "formats"))]
    pub formats: Vec<Format>,
}

#[derive(Deserialize, Debug)]
pub struct VideoDetails {
    #[serde(rename(deserialize = "videoId"))]
    pub video_id: String,
    #[serde(rename(deserialize = "title"))]
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct PlayerResponse {
    #[serde(rename(deserialize = "streamingData"))]
    pub streaming_data: Option<StreamingData>,
    #[serde(rename(deserialize = "videoDetails"))]
    pub video_details: VideoDetails,
}
