use crate::{extraction::types::MediaContent, kore};

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
    let subtitles_path = get_audio_path(app_handle);
    let mut subtitles: Vec<MediaContent> = vec![];

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
                subtitles.push(MediaContent { index, codec, lang });
            } else {
                eprintln!("Error while trying to convert to i8");
            }
        } else {
            eprintln!("Can't split on two parts");
        }
    }
    println!("{:?}", subtitles);

    let file_name: String = Path::new(&path)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .to_string();
    for subtitle in subtitles {
        let index: String = "0:".to_owned() + &subtitle.index;
        let extract_path: String =
            subtitles_path.clone() + "\\" + &file_name + "_" + &subtitle.lang + ".aac";
        println!("{:?}", extract_path);
        Command::new("ffmpeg")
            .args([
                "-i",
                &path,
                "-map",
                &index,
                "-f",
                "aac",
                "-c:s",
                "copy",
                &extract_path,
            ])
            .output()
            .expect("failed to execute process");
    }
}
