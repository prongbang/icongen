use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process::exit;
use crate::logger;

pub fn create_dir(dst_path: &String) {
    if !Path::new(dst_path).exists() {
        fs::create_dir_all(dst_path).expect("Unable to create destination directory.");
    }
}

pub fn check_exists(src_file: &String) {
    if !Path::new(src_file).exists() {
        logger::error(&format!("Source file {} doesn't exist. Please check.", src_file));
        exit(-1);
    }
}

pub fn read_file(file_path: &str) -> std::io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}