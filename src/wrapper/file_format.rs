use crate::utils::FILE_SIZE_SUFFIXES;
use crate::YouDlError;
use std::fmt;

pub struct FileFormat {
    pub itag: String,
    pub file_extension: String,
    pub video_resolution: String,
    pub file_size: String,
}

impl FileFormat {
    pub fn from_youtube_dl_stdout(youtube_dl_stdout: &str) -> Result<Vec<FileFormat>, YouDlError> {
        youtube_dl_stdout
            .lines()
            .filter(|&line| line.starts_with(|c: char| c.is_numeric()))
            .map(Self::parse_line)
            .collect()
    }

    fn parse_line(line: &str) -> Result<FileFormat, YouDlError> {
        let mut words_iter = line.split_whitespace();
        let (itag, file_extension, video_resolution) =
            (words_iter.next(), words_iter.next(), words_iter.next());

        let file_size = words_iter.last();
        let file_size = extract_option_str(file_size).map(|size| {
            if is_valid_file_size(&size) {
                size
            } else {
                String::from("unknown")
            }
        })?;

        Ok(FileFormat {
            itag: extract_option_str(itag)?,
            file_extension: extract_option_str(file_extension)?,
            video_resolution: extract_option_str(video_resolution)?,
            file_size,
        })
    }
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<6}{:<8}{:<11}{}",
            self.itag, self.file_extension, self.video_resolution, self.file_size
        )
    }
}

fn extract_option_str(optional_str: Option<&str>) -> Result<String, YouDlError> {
    optional_str
        .map(|s| s.to_owned())
        .ok_or(YouDlError::Application(
            "failed to parse file_format".to_owned(),
        ))
}

// the stdout for `youtube-dl -F <link>` doesn't always specify the file size
fn is_valid_file_size(file_size: &str) -> bool {
    FILE_SIZE_SUFFIXES
        .iter()
        .any(|suffix| file_size.ends_with(suffix))
}

//
// EXAMPLE FILE FORMATS FROM YOUTUBE-DL
//
// format code  extension  resolution note
// 139          m4a        audio only DASH audio   49k , m4a_dash container, mp4a.40.5@ 48k (22050Hz)
// 251          webm       audio only DASH audio   96k , webm_dash container, opus @160k (48000Hz)
// 140          m4a        audio only DASH audio  130k , m4a_dash container, mp4a.40.2@128k (44100Hz)
// 394          mp4        256x144    144p   71k , av01.0.00M.08, 30fps, video only, 132.74KiB
// 278          webm       256x144    DASH video   95k , webm_dash container, vp9, 30fps, video only
// 160          mp4        256x144    DASH video  108k , mp4_dash container, avc1.4d400b, 30fps, video only
// 395          mp4        426x240    240p  151k , av01.0.00M.08, 30fps, video only, 244.46KiB
// 242          webm       426x240    DASH video  220k , webm_dash container, vp9, 30fps, video only
// 133          mp4        426x240    DASH video  242k , mp4_dash container, avc1.4d400c, 30fps, video only
// 396          mp4        640x360    360p  294k , av01.0.01M.08, 30fps, video only, 472.37KiB
// 243          webm       640x360    DASH video  405k , webm_dash container, vp9, 30fps, video only
// 397          mp4        854x480    480p  512k , av01.0.04M.08, 30fps, video only, 840.61KiB
// 134          mp4        640x360    DASH video  594k , mp4_dash container, avc1.4d401e, 30fps, video only
// 244          webm       854x480    DASH video  752k , webm_dash container, vp9, 30fps, video only
// 398          mp4        1280x720   720p 1015k , av01.0.05M.08, 30fps, video only, 1.54MiB
// 135          mp4        854x480    DASH video 1155k , mp4_dash container, avc1.4d4014, 30fps, video only
// 247          webm       1280x720   DASH video 1505k , webm_dash container, vp9, 30fps, video only
// 136          mp4        1280x720   DASH video 2310k , mp4_dash container, avc1.4d4016, 30fps, video only
// 18           mp4        640x360    360p  483k , avc1.42001E, 30fps, mp4a.40.2@ 96k (44100Hz), 1011.71KiB
// 22           mp4        1280x720   720p 1472k , avc1.64001F, 30fps, mp4a.40.2@192k (44100Hz) (best)
