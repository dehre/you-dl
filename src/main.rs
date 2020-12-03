use smol::fs;
use std::error::Error;
use std::io;
use youtube_downloader as you_dl;

mod config;

fn main() {
    if let Err(e) = smol::block_on(async_main()) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn async_main() -> Result<(), Box<dyn Error>> {
    let config = config::parse();
    let str_links = fs::read_to_string(&config.from_file).await?;
    let links: Vec<&str> = str_links
        .lines()
        .filter(|&l| !l.trim().is_empty())
        .collect();

    let smol_tasks: Vec<_> = links
        .iter()
        .map(|&link| {
            let link = String::from(link);
            let output_dir = String::from(config.output_dir.to_str().unwrap());
            smol::spawn(process_request(link, output_dir))
        })
        .collect();

    // TODO LORIS: collect io::Errors here and send them up?
    futures::future::join_all(smol_tasks).await;

    Ok(())
}

async fn process_request(link: String, output_dir: String) -> Result<(), io::Error> {
    let title = you_dl::get_title(&link)?;
    let available_file_formats = you_dl::get_available_file_formats(&link)?;
    let chosen_file_format = you_dl::ask_preferred_file_format(&title, &available_file_formats)?;
    you_dl::download_video(&link, &title, &chosen_file_format, &output_dir).await?;
    Ok(())
}

// OUTLINE:
// spawn child process for each link
// args input_file and output_directory
// handle invalid links & errors from youtube-dl
// async wait for output?
// choose each video format before downloading
// proper cli library?
// cursor to choose file format?
