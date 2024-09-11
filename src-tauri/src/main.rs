// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gogoanime_scraper::*;
use scraper::get_anime_episodes_and_download_the_episodes;
use serde::Serialize;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct AnimeInfo {
    title: String,
    url: String,
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

    // Send notification
    tauri::api::notification::Notification::new("Download Complete")
        .title("Download Complete")
        .body(format!("The download of {} is complete!", anime_name))
        .show()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_filter_dub() -> Result<bool, String> {
    let path = Path::new("filter_dub.txt");
    if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Ok(content.trim() == "true")
    } else {
        Ok(false) // Default value if the file doesn't exist
    }
}

#[tauri::command]
async fn set_filter_dub(is_dub: bool) -> Result<(), String> {
    let path = Path::new("filter_dub.txt");
    fs::write(path, is_dub.to_string()).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_filter_sub() -> Result<bool, String> {
    let path = Path::new("filter_sub.txt");
    if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Ok(content.trim() == "true")
    } else {
        Ok(false) // Default value if the file doesn't exist
    }
}

#[tauri::command]
async fn set_filter_sub(is_sub: bool) -> Result<(), String> {
    let path = Path::new("filter_sub.txt");
    fs::write(path, is_sub.to_string()).map_err(|e| e.to_string())?;
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_anime,
            download_anime,
            get_filter_dub,
            set_filter_dub,
            get_filter_sub,
            set_filter_sub
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
