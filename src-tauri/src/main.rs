// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod download;
use download::*;
use gogoanime_scraper::*;
use scraper::get_anime_episodes_and_download_the_episodes;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
struct Settings {
    filter_dub: bool,
    filter_sub: bool,
}

fn get_settings_path() -> std::path::PathBuf {
    let home_dir = dirs::home_dir().expect("Could not find home directory");
    home_dir.join("settings.toml")
}

fn load_settings() -> Result<Settings, String> {
    let path = get_settings_path();
    if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        toml::de::from_str(&content).map_err(|e| e.to_string())
    } else {
        Ok(Settings {
            filter_dub: false,
            filter_sub: false,
        }) // Default values
    }
}

fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = get_settings_path();
    let content = toml::ser::to_string(settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())
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
async fn get_filter_dub() -> Result<bool, String> {
    let settings = load_settings()?;
    Ok(settings.filter_dub)
}

#[tauri::command]
async fn set_filter_dub(is_dub: bool) -> Result<(), String> {
    let mut settings = load_settings()?;
    settings.filter_dub = is_dub;
    save_settings(&settings)
}

#[tauri::command]
async fn get_filter_sub() -> Result<bool, String> {
    let settings = load_settings()?;
    Ok(settings.filter_sub)
}

#[tauri::command]
async fn set_filter_sub(is_sub: bool) -> Result<(), String> {
    let mut settings = load_settings()?;
    settings.filter_sub = is_sub;
    save_settings(&settings)
}

#[tauri::command]
fn check_downloads() -> Result<serde_json::Value, String> {
    let home_dir = dirs::home_dir().ok_or("Could not find home directory")?;
    let anime_dir = home_dir.join("Videos").join("Anime");

    let mut downloading = Vec::new();
    let mut downloaded = Vec::new();

    if let Ok(entries) = fs::read_dir(&anime_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let folder_name = entry.file_name();
                    let folder_path = anime_dir.join(&folder_name);

                    if has_tmp_file(&folder_path) {
                        downloading.push(folder_name.to_string_lossy().into_owned());
                    } else {
                        downloaded.push(folder_name.to_string_lossy().into_owned());
                    }
                }
            }
        }
    }

    Ok(serde_json::json!({
        "downloading": downloading,
        "downloaded": downloaded
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

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_anime,
            download_anime,
            get_filter_dub,
            set_filter_dub,
            get_filter_sub,
            set_filter_sub,
            check_downloads
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
