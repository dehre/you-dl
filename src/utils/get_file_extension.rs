// Source: https://github.com/ytdl-org/youtube-dl/blob/1a95953867412bc7a785f21f6bff5145b2b13fd0/youtube_dl/extractor/youtube.py#L392
const PAIRS: [(i32, &str); 24] = [
    (5, "flv"),
    (6, "flv"),
    (13, "3gp"),
    (17, "3gp"),
    (18, "mp4"),
    (22, "mp4"),
    (34, "flv"),
    (35, "flv"),
    (36, "3gp"),
    (37, "mp4"),
    (38, "mp4"),
    (43, "webm"),
    (44, "webm"),
    (45, "webm"),
    (46, "webm"),
    (59, "mp4"),
    (78, "mp4"),
    (82, "mp4"),
    (83, "mp4"),
    (84, "mp4"),
    (85, "mp4"),
    (100, "webm"),
    (101, "webm"),
    (102, "webm"),
];

pub fn get_file_extension(itag: i32) -> Option<&'static str> {
    PAIRS.iter().find(|&item| item.0 == itag).map(|item| item.1)
}
