// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod extraction;
mod kore;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            kore::config::check_create_config_file(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            extraction::subtitles::get_subtitle_path,
            extraction::subtitles::set_subtitle_path,
            extraction::subtitles::get_all_subtitle,
            extraction::audio::get_audio_path,
            extraction::audio::set_audio_path,
            extraction::audio::get_all_audio,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
