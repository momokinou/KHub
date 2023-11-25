use crate::kore;

use std::{path::Path, process::Command};

#[derive(Debug)]
struct MediaContent {
    index: String,
    codec: String,
    lang: String,
}

#[tauri::command]
pub fn get_subtitle_path(app_handle: tauri::AppHandle) -> String {
    let config_path = kore::config::get_config_file_path(app_handle.clone());
    let file = std::fs::File::open(&config_path).unwrap();
    let configuration: kore::config::Configuration = serde_json::from_reader(file).unwrap();

    configuration.export_subtitle_path
}

#[tauri::command]
pub fn set_subtitle_path(app_handle: tauri::AppHandle, new_path: String) -> String {
    let config_path = kore::config::get_config_file_path(app_handle.clone());
    let file = std::fs::File::open(&config_path).unwrap();
    let mut configuration: kore::config::Configuration = serde_json::from_reader(file).unwrap();

    configuration.export_subtitle_path = new_path;
    std::fs::write(
        config_path,
        serde_json::to_string_pretty(&configuration).unwrap(),
    )
    .unwrap();

    configuration.export_subtitle_path
}

#[tauri::command]
pub fn get_all_subtitle(app_handle: tauri::AppHandle, path: String) {
    let subtitles_path = get_subtitle_path(app_handle);
    let mut subtitles: Vec<MediaContent> = vec![];

    let cmd = Command::new("ffprobe")
        .args([
            "-loglevel",
            "error",
            "-select_streams",
            "s",
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
                let lang = remove_netflix_forced_narrative(lang_str.join(""));
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
        let extension = get_extension_by_codec(&subtitle.codec);
        if !extension.is_empty() {
            let index: String = "0:".to_owned() + &subtitle.index;
            let extract_path: String =
                subtitles_path.clone() + "\\" + &file_name + "_" + &subtitle.lang + "." + extension;
            println!("{:?}", extract_path);
            Command::new("ffmpeg")
                .args([
                    "-i",
                    &path,
                    "-map",
                    &index,
                    "-f",
                    extension,
                    "-c:s",
                    "copy",
                    &extract_path,
                ])
                .output()
                .expect("failed to execute process");
        } else {
            eprintln!("codec not supported")
        }
    }
}

fn remove_netflix_forced_narrative(lang: String) -> String {
    lang.replace(" (Forced Narrative)", "")
}

fn get_extension_by_codec(codec: &str) -> &'static str {
    match codec.to_lowercase().as_str() {
        "ass" => "ass",
        "dvb_subtitle" => "",
        "dvd_subtitle" => "",
        "eia_608" => "", // Les closed captions ne sont généralement pas stockées dans des fichiers séparés
        "hdmv_pgs_subtitle" => "",
        "jacosub" => "jss",
        "microdvd" => "sub",
        "mov_text" => "mov",
        "mpl2" => "mpl",
        "pjs" => "pjs",
        "realtext" => "rt",
        "sami" => "smi",
        "stl" => "stl",
        "subrip" => "srt",
        "subviewer" => "sub",
        "subviewer1" => "sub",
        "text" => "txt",
        "vplayer" => "txt",
        "webvtt" => "vtt",
        "xsub" => "", // XSUB est généralement inclus dans le fichier vidéo
        _ => "",      // Cas par défaut si le codec n'est pas reconnu
    }
}
