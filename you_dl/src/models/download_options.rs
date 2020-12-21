use crate::utils;
use crate::{PlayerResponse, YouDlError};
use std::convert::TryFrom;
use std::fmt;

pub struct DownloadOption {
    pub video_id: String,
    pub title: String,
    pub file_extension: String,
    pub itag: i32,
    pub url: String,
    pub quality_label: String,
    pub file_size: String,
    pub mime_type: String,
}

impl fmt::Display for DownloadOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<5}{:<6}{:<10}{}",
            self.itag, self.quality_label, self.file_size, self.mime_type
        )
    }
}

// DownloadOptions always wraps at least one DownloadOption
pub struct DownloadOptions(pub Vec<DownloadOption>);

impl DownloadOptions {
    pub fn get_title(&self) -> Result<&str, YouDlError> {
        Ok(&self.0.get(0).expect("at least one option available").title)
    }
}

impl TryFrom<PlayerResponse> for DownloadOptions {
    type Error = YouDlError;

    fn try_from(player_response: PlayerResponse) -> Result<Self, Self::Error> {
        let video_id = player_response.video_details.video_id;
        let title = player_response.video_details.title.replace("+", " ");
        let streaming_data =
            player_response
                .streaming_data
                .ok_or(YouDlError::UndownloadableVideo(
                    (&title).clone(),
                    "missing value for streaming_data".to_owned(),
                ))?;

        if streaming_data.formats.len() == 0 {
            return Err(YouDlError::UndownloadableVideo(
                title.to_owned(),
                "no options available to download".to_owned(),
            ));
        }

        let mut download_options =
            Vec::<DownloadOption>::with_capacity(streaming_data.formats.len());
        for format in streaming_data.formats.into_iter() {
            let file_extension = utils::get_file_extension(format.itag)
                .unwrap_or("")
                .to_owned();
            let url = format.url.ok_or(YouDlError::UndownloadableVideo(
                (&title).clone(),
                "missing value for url".to_owned(),
            ))?;
            let approx_duration_ms = format.approx_duration_ms.parse::<i32>().map_err(|_| {
                YouDlError::InvalidResponse(
                    "approx_duration_ms cannot be parsed into integer".to_owned(),
                )
            })?;
            let file_size_bytes = format.bitrate * (approx_duration_ms / 1000) / 8;
            let file_size = utils::format_file_size(file_size_bytes);

            download_options.push(DownloadOption {
                video_id: (&video_id).clone(),
                title: (&title).clone(),
                file_extension,
                itag: format.itag,
                url,
                quality_label: format.quality_label,
                file_size,
                mime_type: format.mime_type,
            });
        }

        Ok(DownloadOptions(download_options))
    }
}
