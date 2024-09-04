// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gogoanime_scraper::*;
use reqwest::Client;
use serde::Serialize;
use std::fs::File;
use std::io::copy;

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
async fn download_anime(url: &str) -> Result<(), String> {
    let client = Client::new();
    let response = client.get(url).send().await.map_err(|e| e.to_string())?;
    let mut file = File::create("downloaded_anime.mp4").map_err(|e| e.to_string())?;
    let mut content = response.bytes().await.map_err(|e| e.to_string())?;
    copy(&mut content.as_ref(), &mut file).map_err(|e| e.to_string())?;
    println!("Downloaded anime to {:?}", content);
    Ok(())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![search_anime, download_anime])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
