use std::fs;

#[allow(dead_code)]
pub fn check_folder(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

#[allow(dead_code)]
pub fn create_folder(path: &str) -> Result<(), std::io::Error> {
    fs::create_dir(path)
}
