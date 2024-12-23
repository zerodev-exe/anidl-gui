use reqwest::Client;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

lazy_static::lazy_static! {
    pub static ref DOWNLOAD_PROGRESS: Mutex<HashMap<String, f64>> = Mutex::new(HashMap::new());
}

fn update_download_progress(file_path: &str, progress: f64) {
    let mut progress_map = DOWNLOAD_PROGRESS.lock().unwrap();
    progress_map.insert(file_path.to_string(), progress);
}

fn create_dir(file_path: &str) {
    let videos_dir = dirs::video_dir()
        .ok_or("Could not find the Videos directory")
        .unwrap();
    let full_path = videos_dir.join("Anime").join(file_path);
    fs::create_dir_all(&full_path).expect("Couldn't create the path");
}

// Handles redirection and returns the final download link from a URL, with retry logic for failed requests
pub async fn handle_redirect_and_get_link(
    encoded_url: &str,
    file_path: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::builder()
        .redirect(reqwest::redirect::Policy::limited(5)) // Limit the number of redirects to 5
        .build()?;
    let mut current_url = encoded_url.to_string();

    let videos_dir = dirs::video_dir().ok_or("Could not find the Videos directory")?;
    let full_path = videos_dir.join("Anime").join(file_path);

    create_dir(full_path.to_str().expect("Couldn't create the path"));

    loop {
        let response = match client.get(&current_url).send().await {
            Ok(resp) => resp,
            Err(_) => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "HTTP request failed",
                )));
            }
        };

        if response.status().is_redirection() {
            if let Some(location) = response.headers().get(reqwest::header::LOCATION) {
                let new_url = location.to_str().unwrap().to_string();
                println!("Redirecting to: {}", new_url);
                current_url = new_url;
                continue;
            }
        } else if response.status().is_client_error() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "HTTP request failed",
            )));
        }

        // Return the final URL instead of downloading the content
        return Ok(current_url);
    }
}

// Asynchronously downloads content from a given URL and writes it to a file
async fn download_content(
    client: &Client,
    url: &str,
    full_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = client.get(url).send().await?;
    let total_size = response
        .content_length()
        .ok_or("Failed to get content length")?;

    let temp_file_path = create_temp_file_path(full_file_path);
    let mut file = File::create(&temp_file_path).await?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;

        // Calculate progress only if total_size is greater than 0
        if total_size > 0 {
            let progress = (downloaded as f64 / total_size as f64) * 100.0;
            update_download_progress(full_file_path, progress);
        }
    }

    tokio::fs::rename(&temp_file_path, full_file_path).await?;
    let file_size = fs::metadata(full_file_path).unwrap().len();
    if file_size > 0 {
        Ok(())
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Download failed",
        )))
    }
}

// Handles redirection and downloads the content from the final URL
pub async fn handle_redirect_and_download(
    encoded_url: &str,
    file_path: &str,
    anime_episode: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let final_url = handle_redirect_and_get_link(encoded_url, file_path).await?;
    let client = Client::new();
    let videos_dir = dirs::video_dir().ok_or("Could not find the Videos directory")?;
    let full_file_path = videos_dir.join("Anime").join(file_path).join(anime_episode);
    download_content(&client, &final_url, full_file_path.to_str().unwrap()).await?;
    Ok(())
}

fn create_temp_file_path(full_file_path: &str) -> String {
    format!("{}.tmp", full_file_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::remove_file;

    #[tokio::test]
    async fn test_download_content() {
        let client = Client::new();
        let url = "https://httpbin.org/bytes/1024";
        let full_file_path = "test_download.txt";
        let result = download_content(&client, url, full_file_path).await;
        assert!(result.is_ok());
        assert!(std::path::Path::new(full_file_path).exists());
        remove_file(full_file_path).await.unwrap();
    }
}
