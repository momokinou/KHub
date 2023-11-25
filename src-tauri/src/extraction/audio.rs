use crate::kore;

#[tauri::command]
pub async fn get_audio_path(app_handle: tauri::AppHandle) -> String {
    let config_path = kore::config::get_config_file_path(app_handle.clone());
    let file = std::fs::File::open(&config_path).unwrap();
    let configuration: kore::config::Configuration = serde_json::from_reader(file).unwrap();

    configuration.export_audio_path
}

#[tauri::command]
pub async fn set_audio_path(app_handle: tauri::AppHandle, new_path: String) -> String {
    let config_path = kore::config::get_config_file_path(app_handle.clone());
    let file = std::fs::File::open(&config_path).unwrap();
    let mut configuration: kore::config::Configuration = serde_json::from_reader(file).unwrap();

    configuration.export_audio_path = new_path;
    std::fs::write(
        config_path,
        serde_json::to_string_pretty(&configuration).unwrap(),
    )
    .unwrap();

    configuration.export_audio_path
}
