// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod download;
mod scraper;
mod settings;
mod utils;
use gogoanime_scraper::get_anime_list_by_name;
use scraper::get_anime_episodes_and_download_the_episodes;
use serde::Serialize;
use std::fs;

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
async fn retry_button(anime_name: &str) -> Result<(), String> {
    if anime_name.is_empty() {
        return Err("Anime URL ending is empty".to_string());
    }

    let anime = utils::sanitize_from_path(anime_name.to_string());

    download_anime(&anime, anime_name).await.unwrap();

    Ok(())
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

    let dotfile_path = anime_dir.join(".anime_episodes");
    utils::delete_if_date_hour_mismatch(&dotfile_path)?;

    let mut downloading = Vec::new();
    let mut downloaded = Vec::new();
    let mut ongoing = Vec::new();
    let mut progress = std::collections::HashMap::new();

    if let Ok(entries) = fs::read_dir(&anime_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let folder_name = entry.file_name().to_string_lossy().to_string();
                    let folder_path = anime_dir.join(&folder_name);

                    let downloaded_episodes = utils::count_mp4_files(&folder_path);
                    let anime_url_ending = utils::sanitize_from_path(folder_name.clone());

                    let total_episodes = scraper::get_how_many_episodes_there_are(
                        anime_url_ending,
                        dotfile_path.clone(),
                    )
                    .await
                    .expect("msg")
                    .unwrap_or(0);

                    if downloaded_episodes < total_episodes {
                        downloading.push(folder_name.clone());
                        progress.insert(folder_name.clone(), (downloaded_episodes, total_episodes));
                    } else if downloaded_episodes == total_episodes
                        && utils::has_tmp_file(&folder_path)
                    {
                        ongoing.push(folder_name.clone());
                        progress.insert(folder_name.clone(), (downloaded_episodes, total_episodes));
                    } else if downloaded_episodes > 0 {
                        downloaded.push(folder_name.clone());
                        progress.insert(folder_name.clone(), (downloaded_episodes, total_episodes));
                    }
                }
            }
        }
    }

    Ok(serde_json::json!({
        "downloading": downloading,
        "downloaded": downloaded,
        "ongoing": ongoing,
        "progress": progress
    }))
}

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_anime,
            download_anime,
            settings::get_filter_dub,
            settings::set_filter_dub,
            settings::get_filter_sub,
            settings::set_filter_sub,
            check_downloads,
            retry_button,
        ])
        .build(tauri::generate_context!("tauri.conf.json"))
        .expect("error while building tauri application");
    app.run(|_app_handle, event| {
        if let tauri::RunEvent::Updater(updater_event) = event {
            match updater_event {
                tauri::UpdaterEvent::UpdateAvailable {
                    body,
                    date,
                    version,
                } => {
                    println!("update available {} {:?} {}", body, date, version);
                }
                // Emitted when the download is about to be started.
                tauri::UpdaterEvent::Pending => {
                    println!("update is pending!");
                }
                tauri::UpdaterEvent::DownloadProgress {
                    chunk_length,
                    content_length,
                } => {
                    println!("downloaded {} of {:?}", chunk_length, content_length);
                }
                // Emitted when the download has finished and the update is about to be installed.
                tauri::UpdaterEvent::Downloaded => {
                    println!("update has been downloaded!");
                }
                // Emitted when the update was installed. You can then ask to restart the app.
                tauri::UpdaterEvent::Updated => {
                    println!("app has been updated");
                }
                // Emitted when the app already has the latest version installed and an update is not needed.
                tauri::UpdaterEvent::AlreadyUpToDate => {
                    println!("app is already up to date");
                }
                // Emitted when there is an error with the updater. We suggest to listen to this event even if the default dialog is enabled.
                tauri::UpdaterEvent::Error(error) => {
                    println!("failed to update: {}", error);
                }
            }
        }
    });
}
