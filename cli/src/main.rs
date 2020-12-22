use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::thread;
use you_dl::{self, YouDlError};

mod config;

fn main() {
    smol::block_on(async_main())
}

async fn async_main() {
    let multi_bar = MultiProgress::new();
    let config = config::parse().await.unwrap(); // TODO LORIS: use error! macro
    let smol_tasks: Vec<_> = config
        .video_urls
        .iter()
        .map(|url| {
            let url = url.to_owned();
            let output_dir = config.output_dir.clone();
            let progress_bar = create_process_bar(&multi_bar);
            smol::spawn(process_request(
                url,
                output_dir,
                config.use_wrapper,
                progress_bar,
            ))
        })
        .collect();

    // Waits for all progress bars to report that they are finished.
    thread::spawn(move || {
        multi_bar.join().unwrap();
    });

    for result in futures::future::join_all(smol_tasks).await {
        if let Err(e) = result {
            eprintln!("{}", e) // TODO LORIS: use error! macro
        }
    }
}

fn create_process_bar(multi_bar: &MultiProgress) -> ProgressBar {
    let progress_bar = multi_bar.add(ProgressBar::new(0));
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed}] {bar:40.cyan/blue} {percent}% {wide_msg}")
            .progress_chars("##-"),
    );
    progress_bar
}

async fn process_request(
    url: String,
    output_dir: String,
    use_wrapper: bool,
    progress_bar: ProgressBar,
) -> Result<(), YouDlError> {
    if use_wrapper {
        you_dl::wrapper::process_request(&url, &output_dir).await
    } else {
        you_dl::process_request(&url, &output_dir, progress_bar).await
    }
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
