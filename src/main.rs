use futures::future;
use indicatif::{MultiProgress, ProgressBar};
use std::process;
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
            let progress_bar = multi_bar.add(ProgressBar::new(0));
            smol::spawn(process_request(
                url,
                output_dir,
                progress_bar,
                config.use_wrapper,
            ))
        })
        .collect();

    multi_bar.join().unwrap(); // request the draw instructions from the remote progress bars
    for result in future::join_all(smol_tasks).await {
        if let Err(e) = result {
            you_dl::failed!("{}", e)
        }
    }
}

async fn process_request(
    url: String,
    output_dir: String,
    progress_bar: ProgressBar,
    use_wrapper: bool,
) -> Result<(), YouDlError> {
    if use_wrapper {
        you_dl::wrapper::process_request(&url, &output_dir).await
    } else {
        you_dl::process_request(&url, &output_dir, progress_bar).await
    }
}
