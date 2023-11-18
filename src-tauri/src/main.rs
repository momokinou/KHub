// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;
use std::{fs::File, path::Path};

pub mod extraction;

fn config_file(app_handle: &mut tauri::App) {
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            config_file(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            extraction::subtitles::get_subtitle_path,
            extraction::subtitles::set_subtitle_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
