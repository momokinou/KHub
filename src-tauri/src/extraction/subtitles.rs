use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct Configuration {
    export_subtitle_path: String,
    export_audio_path: String,
}

fn get_config_path(app_handle: tauri::AppHandle) -> String {
    app_handle
        .path_resolver()
        .app_config_dir()
        .unwrap_or(std::path::PathBuf::new())
        .to_string_lossy()
        .to_string()
        + "\\config.json"
}

#[tauri::command]
pub async fn get_subtitle_path(app_handle: tauri::AppHandle) -> String {
    let config_path = get_config_path(app_handle.clone());
    let file = std::fs::File::open(&config_path).unwrap();
    let configuration: Configuration = serde_json::from_reader(file).unwrap();

    configuration.export_subtitle_path
}

#[tauri::command]
pub async fn set_subtitle_path(app_handle: tauri::AppHandle, new_path: String) -> String {
    let config_path = get_config_path(app_handle.clone());
    let file = std::fs::File::open(&config_path).unwrap();
    let mut configuration: Configuration = serde_json::from_reader(file).unwrap();

    configuration.export_subtitle_path = new_path;
    std::fs::write(
        config_path,
        serde_json::to_string_pretty(&configuration).unwrap(),
    )
    .unwrap();

    configuration.export_subtitle_path
}
