// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod download;
mod scraper;
mod settings;
use gogoanime_scraper::get_anime_list_by_name;
use scraper::get_anime_episodes_and_download_the_episodes;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref DOWNLOAD_PROGRESS: Mutex<HashMap<String, f64>> = Mutex::new(HashMap::new());
}

fn update_download_progress(file_path: &str, progress: f64) {
    let mut progress_map = DOWNLOAD_PROGRESS.lock().unwrap();
    progress_map.insert(file_path.to_string(), progress);
}

#[derive(Serialize)]
struct AnimeInfo {
    url: String,
    title: String,
    img_url: String,
}

#[tauri::command]
async fn search_anime(name: &str) -> Result<Vec<AnimeInfo>, String> {
    let (urls, titles, img_urls) = get_anime_list_by_name(name.to_string()).await;
    let anime_list = titles
        .into_iter()
        .zip(urls)
        .zip(img_urls)
        .map(|((title, url), img_url)| AnimeInfo {
            url,
            title,
            img_url,
        })
        .collect();
    Ok(anime_list)
}

#[tauri::command]
async fn download_anime(anime_url_ending: &str, anime_name: &str) -> Result<(), String> {
    if anime_url_ending.is_empty() {
        return Err("Anime URL ending is empty".to_string());
    }
    get_anime_episodes_and_download_the_episodes(anime_url_ending.to_string(), anime_name)
        .await
        .unwrap();

    let notification = tauri::api::notification::Notification::new("Download Complete");

    notification
        .title("Download Complete")
        .body(format!("The download of {} is complete!", anime_name))
        .show()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn check_downloads() -> Result<serde_json::Value, String> {
    let anime_dir = dirs::video_dir().unwrap().join("Anime");

    let mut downloading = Vec::new();
    let mut downloaded = Vec::new();
    let mut ongoing = Vec::new(); // New vector for ongoing downloads
    let mut progress = std::collections::HashMap::new();

    if let Ok(entries) = fs::read_dir(&anime_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let folder_name = entry.file_name();
                    let folder_path = anime_dir.join(&folder_name);

                    let downloaded_episodes = count_mp4_files(&folder_path);
                    let anime_url_ending = folder_name
                        .to_string_lossy()
                        .trim()
                        .replace("(", "")
                        .replace(")", "")
                        .replace(" ", "-")
                        .replace("!", "")
                        .replace("/", "")
                        .replace(".", "")
                        .replace("'", "")
                        .to_lowercase()
                        .to_owned();

                    let total_episodes = scraper::get_how_many_episodes_there_are(anime_url_ending)
                        .await
                        .unwrap_or(0);

                    if (downloaded_episodes as u32) < total_episodes {
                        downloading.push(folder_name.to_string_lossy().into_owned());
                        progress.insert(
                            folder_name.to_string_lossy().into_owned(),
                            (downloaded_episodes, total_episodes),
                        );
                    } else if downloaded_episodes as u32 == total_episodes
                        && has_tmp_file(&folder_path)
                    {
                        ongoing.push(folder_name.to_string_lossy().into_owned());
                        progress.insert(
                            folder_name.to_string_lossy().into_owned(),
                            (downloaded_episodes, total_episodes),
                        );
                    } else if downloaded_episodes > 0 {
                        downloaded.push(folder_name.to_string_lossy().into_owned());
                        progress.insert(
                            folder_name.to_string_lossy().into_owned(),
                            (downloaded_episodes, total_episodes),
                        ); // Use downloaded_episodes as total_episodes
                    }
                }
            }
        }
    }

    Ok(serde_json::json!({
        "downloading": downloading,
        "downloaded": downloaded,
        "ongoing": ongoing, // Include ongoing in the response
        "progress": progress
    }))
}

fn has_tmp_file(dir: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(extension) = entry.path().extension() {
                        if extension == "tmp" {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn count_mp4_files(dir: &Path) -> usize {
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "mp4"))
                .count()
        })
        .unwrap_or(0)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_anime,
            download_anime,
            settings::get_filter_dub,
            settings::set_filter_dub,
            settings::get_filter_sub,
            settings::set_filter_sub,
            check_downloads
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
