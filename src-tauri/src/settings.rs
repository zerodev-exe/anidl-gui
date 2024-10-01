use serde::{Deserialize, Serialize};
use std::fs;

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

#[tauri::command]
pub async fn get_filter_dub() -> Result<bool, String> {
    let settings = load_settings()?;
    Ok(settings.filter_dub)
}

#[tauri::command]
pub async fn set_filter_dub(is_dub: bool) -> Result<(), String> {
    let mut settings = load_settings()?;
    settings.filter_dub = is_dub;
    save_settings(&settings)
}

#[tauri::command]
pub async fn get_filter_sub() -> Result<bool, String> {
    let settings = load_settings()?;
    Ok(settings.filter_sub)
}

#[tauri::command]
pub async fn set_filter_sub(is_sub: bool) -> Result<(), String> {
    let mut settings = load_settings()?;
    settings.filter_sub = is_sub;
    save_settings(&settings)
}
