mod config;

use you_dl::{self, YouDlError};

fn main() {
    smol::block_on(async_main())
}

async fn async_main() {
    let config = config::parse().await.unwrap_or_else(|e| panic!(e));
    let smol_tasks: Vec<_> = config
        .video_urls
        .iter()
        .map(|url| {
            let url = url.to_owned();
            let output_dir = config.output_dir.clone();
            smol::spawn(process_request(url, output_dir))
        })
        .collect();

    for result in futures::future::join_all(smol_tasks).await {
        if let Err(e) = result {
            eprintln!("{}", e)
        }
    }
}

async fn process_request(url: String, output_dir: String) -> Result<(), YouDlError> {
    let title = you_dl::get_title(&url).await?;
    let available_file_formats = you_dl::get_available_file_formats(&url).await?;
    let chosen_file_format =
        you_dl::ask_preferred_file_format(&title, &available_file_formats).await?;
    you_dl::download_video(&url, &title, &chosen_file_format, &output_dir).await?;
    Ok(())
}

// OUTLINE:
// spawn child process for each link
// args input_file and output_directory
// handle invalid links & errors from youtube-dl
// async wait for output?
// choose each video format before downloading
// proper cli library
// cursor to choose file format
// lib fn return type synonyms instead of strings
// allow either --link or --from-file args
// rename project you_dl
// move stuff for lib and stuff for main in separate directories: https://stackoverflow.com/questions/26946646/rust-package-with-both-a-library-and-a-binary
// remove unwraps and Box<dyn Error> from async_main
// colorize stdout for readability, maybe setting verbosity level w/ logger
