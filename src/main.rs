use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{process, thread};
use you_dl::{self, YouDlError};

mod config;

fn main() {
    smol::block_on(async_main())
}

async fn async_main() {
    let multi_bar = MultiProgress::new();
    let config = config::parse().await.unwrap_or_else(|e| {
        you_dl::error!("{}", e);
        process::exit(1);
    });
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
            you_dl::failed!("{}", e)
        }
    }
}

fn create_process_bar(multi_bar: &MultiProgress) -> ProgressBar {
    let progress_bar = multi_bar.add(ProgressBar::new(0));
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("{prefix:.green} {bar:40.cyan/blue} {percent}% {wide_msg}")
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
