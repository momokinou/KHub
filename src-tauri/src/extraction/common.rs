use std::path::Path;

pub fn is_mkv_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    path.extension().map_or(false, |ext| ext == "mkv")
}
