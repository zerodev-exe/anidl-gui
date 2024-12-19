use crate::{download, utils};
use futures::future::join_all;
use gogoanime_scraper::{parser, CAT_URL, URL};
use scraper::{Html, Selector};
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufWriter, Write};
use std::path::{PathBuf, Path};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task;

// Lazy initialization of a shared HTTP client with cookie support
lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = reqwest::Client::builder()
        .cookie_store(true)
        .build()
        .unwrap();
}

// Trait defining the HTTP client operations
trait HttpClient {
    async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>>;
    async fn post(
        &self,
        url: &str,
        form: &[(&str, &str)],
    ) -> Result<(), Box<dyn std::error::Error>>;
}

// Implementation of the HttpClient trait for reqwest::Client
impl HttpClient for reqwest::Client {
    async fn get(&self, url: &str) -> Result<String, Box<dyn std::error::Error>> {
        let response = self.get(url).send().await?.text().await?;
        Ok(response)
    }

    async fn post(
        &self,
        url: &str,
        form: &[(&str, &str)],
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.post(url).form(form).send().await?;
        Ok(())
    }
}

// Function to retrieve CSRF token from the login page
async fn get_csrf_token<T: HttpClient>(client: &T) -> Result<String, Box<dyn std::error::Error>> {
    let login_page = client.get(&format!("{}{}", URL, "login.html")).await?;
    let document = Html::parse_document(&login_page);
    let selector = Selector::parse("meta[name='csrf-token']")?;
    let csrf_token = document
        .select(&selector)
        .next()
        .and_then(|element| element.value().attr("content"))
        .ok_or("CSRF token not found")?;
    Ok(csrf_token.to_string())
}

// Function to perform login using the CSRF token
async fn login<T: HttpClient>(
    client: &T,
    csrf_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let form_data = [
        ("email", "ritosis807@exeneli.com"),
        ("password", "'%dWU}ZdBJ8LzAy"),
        ("_csrf", csrf_token),
    ];
    client
        .post(&format!("{}{}", URL, "login.html"), &form_data)
        .await?;
    Ok(())
}

// Main function to fetch anime episodes
pub async fn get_anime_episodes_and_download_the_episodes(
    anime_url_ending: String,
    path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = initialize_client();
    let sanitized_path = utils::sanitize_for_path(path.to_string());

    fetch_login_page(&client).await?;
    let csrf_token = get_csrf_token(&client).await?;
    login(&client, &csrf_token).await?;

    let mut episode_number: u32 = 1;
    let mut tasks = vec![];
    let semaphore = Arc::new(Semaphore::new(4));
    let videos_dir = dirs::video_dir().ok_or("Could not find the Videos directory")?;
    let full_path = videos_dir.join("Anime").join(sanitized_path);

    loop {
        let anime_episode = format!("EP-{:02}.mp4", episode_number);
        let full_file_path = full_path.join(&anime_episode);

        if process_existing_file(full_file_path.to_str().unwrap())? {
            episode_number += 1;
            continue;
        }

        let episode_url = format!("{}/{}-episode-{}", URL, anime_url_ending, episode_number);
        let response = reqwest::get(&episode_url).await?;

        if response.status() != reqwest::StatusCode::OK {
            handle_non_ok_response(anime_url_ending.clone(), &full_path, anime_episode).await?;
            break;
        }

        let task = create_download_task(
            semaphore.clone(),
            episode_url,
            full_path.to_str().unwrap().to_string(),
            anime_episode,
        )
        .await;
        tasks.push(task);
        episode_number += 1;
    }

    handle_download_results(tasks).await?;
    Ok(())
}

async fn handle_non_ok_response(
    anime_url_ending: String,
    full_path: &Path,
    anime_episode: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_anime_episode = format!("{}.tmp", anime_episode);
    let tmp_file_path = full_path.join(tmp_anime_episode);

    if is_ongoing(anime_url_ending).await {
        let _ = std::fs::File::create(tmp_file_path);
    }
    Ok(())
}

async fn handle_download_results(
    tasks: Vec<tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let results = join_all(tasks).await;
    for result in results {
        if let Err(e) = result {
            println!("Error downloading episode: {}", e);
        }
    }
    Ok(())
}

async fn download_episode(
    episode_url: String,
    path: String,
    anime_episode: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = CLIENT.clone();
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 5;

    loop {
        let authenticated_content = client.get(&episode_url).send().await?.text().await?;
        let video_urls = parser::get_media_url(authenticated_content);
        let encoded_url = match video_urls.last() {
            Some(url) => url,
            None => {
                if retry_count >= MAX_RETRIES {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "No video URL found after multiple retries",
                    )));
                }
                retry_count += 1;
                continue;
            }
        };

        match download::handle_redirect_and_download(encoded_url, &path, anime_episode.clone())
            .await
        {
            Ok(_) => break,
            Err(_) => {
                if retry_count >= MAX_RETRIES {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Failed to handle redirect after multiple retries",
                    )));
                }
                retry_count += 1;
            }
        }
    }
    Ok(())
}

async fn fetch_login_page(client: &reqwest::Client) -> Result<(), reqwest::Error> {
    client
        .get(format!("{}{}", URL, "login.html"))
        .send()
        .await?;
    Ok(())
}

fn initialize_client() -> reqwest::Client {
    CLIENT.clone()
}

fn process_existing_file(full_file_path: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let path_to_file = std::path::Path::new(full_file_path);
    if path_to_file.exists() {
        let metadata = std::fs::metadata(full_file_path)?;
        if metadata.len() > 0 {
            return Ok(true);
        }
    }
    Ok(false)
}

async fn create_download_task(
    semaphore: Arc<Semaphore>,
    episode_url: String,
    path: String,
    anime_episode: String,
) -> tokio::task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>> {
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    let path_clone = path.clone();
    task::spawn(async move {
        let _permit = permit; // This ensures the semaphore is released when the task completes
        download_episode(episode_url, path_clone, anime_episode).await
    })
}

pub async fn get_how_many_episodes_there_are(
    anime_url_ending: String,
    dotfile_path: PathBuf,
) -> Result<Option<usize>, Box<dyn std::error::Error>> {
    let url = format!("{CAT_URL}{anime_url_ending}");
    let existing_entries = read_dotfile(&dotfile_path)?;

    if let Some((total_episodes, ongoing_status)) = existing_entries.get(&anime_url_ending) {
        if ongoing_status == "true" {
            return Ok(None);
        }
        return Ok(Some(total_episodes.parse()?));
    }

    let body = reqwest::get(&url).await?.text().await?;
    let total_episodes: usize = parser::get_total_number_of_episodes(body)?;

    write_to_dotfile(&dotfile_path, &anime_url_ending, total_episodes)?;
    Ok(Some(total_episodes))
}

fn read_dotfile(
    dotfile_path: &PathBuf,
) -> Result<std::collections::HashMap<String, (String, String)>, Box<dyn std::error::Error>> {
    let mut existing_entries = std::collections::HashMap::new();
    if let Ok(file) = File::open(dotfile_path) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() == 3 {
                existing_entries.insert(
                    parts[0].to_string(),
                    (parts[1].to_string(), parts[2].to_string()),
                );
            }
        }
    }
    Ok(existing_entries)
}

fn write_to_dotfile(
    dotfile_path: &PathBuf,
    anime_url_ending: &str,
    total_episodes: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dotfile_path)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}:{}:false", anime_url_ending, total_episodes)?;
    writer.flush()?;
    Ok(())
}

pub async fn is_ongoing(anime_url_ending: String) -> bool {
    let body = reqwest::get(format!("{CAT_URL}{anime_url_ending}"))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    parser::is_anime_ongoing(&body)
}
