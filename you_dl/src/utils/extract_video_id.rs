use crate::YouDlError;
use regex::Regex;

pub fn extract_video_id(url: &str) -> Result<&str, YouDlError> {
    // source: https://stackoverflow.com/questions/3452546/how-do-i-get-the-youtube-video-id-from-a-url/27728417#27728417
    let video_id =
        Regex::new(r"^.*(?:(?:youtu\.be/|v/|vi/|u/\w/|embed/)|(?:(?:watch)?\?v(?:i)?=|\&v(?:i)?=))([^#\&\?]*).*")
            .expect("valid regex expression")
            .captures(url)
            .ok_or(YouDlError::InvalidURL(url.to_owned()))?
            .get(1)
            .ok_or(YouDlError::InvalidURL(url.to_owned()))?
            .as_str();
    Ok(video_id)
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
