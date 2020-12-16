use super::YouDlError;
use std::fmt;

pub struct FileFormat {
    pub code: String,
    pub extension: String,
    pub resolution: String,
    pub size: String,
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
        let (code, extension, resolution) =
            (words_iter.next(), words_iter.next(), words_iter.next());
        let size = words_iter.last();

        let extract =
            |optional_str: Option<&str>| -> Result<String, YouDlError> {
                optional_str.and_then(|s| Some(String::from(s))).ok_or(
                    YouDlError::ApplicationError("failed to parse file format".to_owned()),
                )
            };

        Ok(FileFormat {
            code: extract(code)?,
            extension: extract(extension)?,
            resolution: extract(resolution)?,
            size: extract(size)?,
        })
    }
}

impl fmt::Display for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:6}{:8}{:12}{}",
            self.code, self.extension, self.resolution, self.size
        )
    }
}

//
// EXAMPLE FILE FORMATS FROM YOUTUBE-DL
//
// format code  extension  resolution note
// > 249          webm       audio only tiny   43k , opus @ 50k (48000Hz), 47.46KiB
//   250          webm       audio only tiny   56k , opus @ 70k (48000Hz), 60.53KiB
//   251          webm       audio only tiny  109k , opus @160k (48000Hz), 118.15KiB
//   140          m4a        audio only tiny  127k , m4a_dash container, mp4a.40.2@128k (44100Hz), 138.59KiB
//   160          mp4        256x144    144p   58k , avc1.4d400c, 30fps, video only, 61.01KiB
//   278          webm       256x144    144p   97k , webm container, vp9, 30fps, video only, 103.89KiB
//   133          mp4        426x240    240p  216k , avc1.4d4015, 30fps, video only, 228.76KiB
//   242          webm       426x240    240p  221k , vp9, 30fps, video only, 235.50KiB
//   134          mp4        640x360    360p  318k , avc1.4d401e, 30fps, video only, 338.08KiB
//   243          webm       640x360    360p  426k , vp9, 30fps, video only, 456.43KiB
//   135          mp4        854x480    480p  666k , avc1.4d401f, 30fps, video only, 706.17KiB
//   244          webm       854x480    480p  789k , vp9, 30fps, video only, 845.16KiB
//   136          mp4        1280x720   720p 1334k , avc1.4d401f, 30fps, video only, 1.39MiB
//   247          webm       1280x720   720p 1640k , vp9, 30fps, video only, 1.68MiB
