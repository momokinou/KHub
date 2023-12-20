use std::fs;
use std::{fs::File, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Configuration {
    pub export_subtitle_path: String,
    pub export_audio_path: String,
}

pub fn check_create_config_file(app_handle: &mut tauri::App) {
    let folder_path = app_handle
        .path_resolver()
        .app_config_dir()
        .unwrap_or(std::path::PathBuf::new())
        .to_string_lossy()
        .to_string();
    let config_folder_path = Path::new(&folder_path);

    let file_path = config_folder_path.to_string_lossy().to_string() + "\\config.json";
    let config_path = Path::new(&file_path);

    if config_path.exists() {
        println!("File exists!");
    } else {
        println!("File does not exist!");
        if !config_folder_path.exists() {
            let folder_display = config_folder_path.display();
            match fs::create_dir_all(&config_folder_path) {
                Err(why) => panic!("couldn't create {}: {}", folder_display, why),
                Ok(folder) => folder,
            }
        }
        let file_display = config_path.display();
        match File::create(&config_path) {
            Err(why) => panic!("couldn't create {}: {}", file_display, why),
            Ok(file) => file,
        };
        match fs::copy("../resources/config.json", config_path) {
            Err(why) => panic!("couldn't write to {}: {}", file_display, why),
            Ok(_) => println!("successfully wrote to {}", file_display),
        }
    }
}

pub fn get_config_file_path(app_handle: tauri::AppHandle) -> String {
    app_handle
        .path_resolver()
        .app_config_dir()
        .unwrap_or(std::path::PathBuf::new())
        .to_string_lossy()
        .to_string()
        + "\\config.json"
}

pub fn is_valid_path(path_str: &str) -> bool {
    let path = Path::new(path_str);

    path.exists() && (path.is_absolute() || path.is_relative())
}
