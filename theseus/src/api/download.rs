use std::process::exit;
use reqwest;
use tokio::fs::File as AsyncFile;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

async fn download_file(download_url: &str, local_filename: &str, open_cmd: &str, auto_update_supported: bool) -> Result<(), Box<dyn std::error::Error>> {
    let download_dir = dirs::download_dir().ok_or("[download_file] • Failed to determine download directory")?;
    let full_path = download_dir.join(local_filename);
    let response = reqwest::get(download_url).await?;
    let bytes = response.bytes().await?;
    let mut dest_file = AsyncFile::create(&full_path).await?;
    dest_file.write_all(&bytes).await?;
    println!("[download_file] • File downloaded to: {:?}", full_path);
    if (auto_update_supported) {
        let status = Command::new(open_cmd)
            .arg(full_path.to_str().unwrap_or_default())
            .status()
            .await
            .expect("[download_file] • Failed to execute 'open' command");

        if status.success() {
            println!("[download_file] • File opened successfully!");
        } else {
            eprintln!("[download_file] • Failed to open the file. Exit code: {:?}", status.code());
        }
    }
    Ok(())
}

pub async fn init_download(download_url: &str, local_filename: &str, open_cmd: &str, auto_update_supported: bool) {
    println!("[init_download] • Initialize downloading from • {:?}", download_url);
    println!("[init_download] • Save local file name • {:?}", local_filename);
    if let Err(e) = download_file(download_url, local_filename, open_cmd, auto_update_supported).await {
        eprintln!("[init_download] • An error occurred! Failed to download the file: {}", e);
    } else {
        println!("[init_download] • Code finishes without errors.");
        exit(0)
    }
}
