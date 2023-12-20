use crate::{
    extraction::{common::is_mkv_file, types::MediaContent},
    kore,
};

use std::{path::Path, process::Command};

#[tauri::command]
pub fn get_audio_path(app_handle: tauri::AppHandle) -> String {
    let config_path = kore::config::get_config_file_path(app_handle.clone());
    let file = std::fs::File::open(&config_path).unwrap();
    let configuration: kore::config::Configuration = serde_json::from_reader(file).unwrap();

    configuration.export_audio_path
}

#[tauri::command]
pub fn set_audio_path(app_handle: tauri::AppHandle, new_path: String) -> String {
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

#[tauri::command]
pub fn get_all_audio(app_handle: tauri::AppHandle, path: String) {
    if !is_mkv_file(&path) || kore::config::is_valid_path(&path) {
        panic!("The file is not a MKV");
    }
    let audio_path = get_audio_path(app_handle);
    let mut audios: Vec<MediaContent> = vec![];

    let cmd = Command::new("ffprobe")
        .args([
            "-loglevel",
            "error",
            "-select_streams",
            "a",
            "-show_entries",
            "stream=index,codec_name:stream_tags=language",
            "-of",
            "csv=p=0",
            &path,
        ])
        .output()
        .expect("failed to execute process");
    let stdout_str = String::from_utf8_lossy(&cmd.stdout);
    let mut vec: Vec<&str> = stdout_str.split("\r\n").collect();
    vec.pop();
    println!("{:?}", vec);

    let vec_iter = vec.iter();
    for element in vec_iter {
        if let Some((index_str, rest)) = element
            .split(',')
            .collect::<Vec<&str>>()
            .as_slice()
            .split_first()
        {
            let (codec_str, lang_str) = rest.split_at(1);
            // Convert to i8 and String
            if let (Ok(index), codec) = (index_str.parse::<String>(), codec_str.join("")) {
                let lang = lang_str.join("");
                audios.push(MediaContent { index, codec, lang });
            } else {
                eprintln!("Error while trying to convert to i8");
            }
        } else {
            eprintln!("Can't split on two parts");
        }
    }
    println!("{:?}", audios);

    let file_name: String = Path::new(&path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    for audio in audios {
        let index: String = "0:".to_owned() + &audio.index;
        println!("index: {:?}", index);
        println!("path: {:?}", audio_path);
        println!("file_name: {:?}", file_name);
        println!("lang: {:?}", audio.lang);
        let extract_path: String =
            audio_path.clone() + "\\" + &file_name + "_" + &audio.lang + ".aac";
        println!("Extract path: {:?}", extract_path);
        let cmd = Command::new("ffmpeg")
            .args([
                "-i",
                &path,
                "-vn",
                "-map",
                &index,
                "-c",
                "copy",
                &extract_path,
            ])
            .output()
            .expect("failed to execute process");
        println!("{:?}", cmd);
    }
}
