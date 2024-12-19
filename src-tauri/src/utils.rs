use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn sanitize_for_path(path: String) -> String {
    path.trim()
        .replace(":", "")
        .replace("/", "")
        .replace("\\", "")
        .replace("*", "")
        .replace("?", "")
        .replace("\"", "")
        .replace("<", "")
        .replace(">", "")
        .replace("|", "")
        .to_owned()
}

pub fn sanitize_from_path(path: String) -> String {
    path.trim()
        .replace("(", "")
        .replace(")", "")
        .replace(" ", "-")
        .replace("!", "")
        .replace("/", "")
        .replace(".", "")
        .replace("'", "")
        .replace(",", "")
        .to_lowercase()
        .to_owned()
}

pub fn has_tmp_file(dir: &Path) -> bool {
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

pub fn count_mp4_files(dir: &Path) -> usize {
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(Result::ok)
                .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "mp4"))
                .count()
        })
        .unwrap_or(0)
}

pub fn delete_if_date_hour_mismatch(file_path: &Path) -> Result<(), String> {
    // Get the current date and hour
    let now = SystemTime::now();
    let current_time = now
        .duration_since(UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    let current_date_hour = current_time / 3600; // Convert seconds to hours

    // Check if the file exists
    if file_path.exists() {
        // Get the file's last modified time
        let metadata = fs::metadata(file_path).map_err(|e| e.to_string())?;
        let modified_time = metadata.modified().map_err(|e| e.to_string())?;
        let modified_duration = modified_time
            .duration_since(UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs();

        let modified_date_hour = modified_duration / 3600; // Convert seconds to hours

        // Compare the current date hour with the modified date hour
        if current_date_hour != modified_date_hour {
            fs::remove_file(file_path).map_err(|e| e.to_string())?;
            println!("Deleted file: {:?}", file_path);
        }
    }

    Ok(())
}
