use smol::process;
use std::io;

pub fn get_title(link: &str) -> Result<String, io::Error> {
    let command = std::process::Command::new("youtube-dl")
        .args(&["--get-title", &link])
        .output()?;
    let title = String::from_utf8(command.stdout).unwrap();
    Ok(String::from(title.trim()))
}

pub fn get_available_file_formats(link: &str) -> Result<Vec<String>, io::Error> {
    let command = std::process::Command::new("youtube-dl")
        .args(&["-F", &link])
        .output()?;
    let stdout = String::from_utf8(command.stdout).unwrap();
    let available_file_formats = stdout
        .lines()
        .filter(|&line| !line.starts_with('['))
        .map(String::from)
        .collect();
    Ok(available_file_formats)
}

pub fn ask_preferred_file_format(
    title: &str,
    available_file_formats: &[String],
) -> Result<String, io::Error> {
    println!("Please choose the preferred file format for {}:", title);
    println!("\t{}", available_file_formats.join("\n\t"));
    // TODO LORIS: show some sort of prompt
    // println!("> ");
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice)?;
    Ok(String::from(user_choice.trim()))
}

pub async fn download_video(
    link: &str,
    title: &str,
    format: &str,
    output_dir: &str,
) -> Result<(), io::Error> {
    println!("Start downloading {} with format {}!\n\n", title, format);
    let file_path = format!("{}/%(title)s.%(ext)s", output_dir);
    let command = process::Command::new("youtube-dl")
        .args(&["-f", format, "-o", &file_path, link])
        .output()
        .await?;

    if !command.status.success() {
        let err = String::from_utf8(command.stderr).unwrap();
        eprintln!("Failed to download {}: {}", title, err);
        return Ok(());
    }

    println!("Successfully downloaded {}", title);
    Ok(())
}
