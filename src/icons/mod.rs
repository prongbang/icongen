pub(crate) mod config;
pub(crate) mod generate;

use std::process::{Command, exit};
use crate::logger;

pub struct Dimension {
    pub width: String,
    pub height: String,
}

pub fn get_dimension(src_file: &String) -> Dimension {
    let src_width = Command::new("sips")
        .arg("-g")
        .arg("pixelWidth")
        .arg(src_file)
        .output()
        .expect("Failed to execute command")
        .stdout;

    let src_height = Command::new("sips")
        .arg("-g")
        .arg("pixelHeight")
        .arg(src_file)
        .output()
        .expect("Failed to execute command")
        .stdout;

    let src_width_str = String::from_utf8_lossy(&src_width);
    let src_height_str = String::from_utf8_lossy(&src_height);

    let src_width = src_width_str.trim().split_whitespace().last().unwrap_or_default();
    let src_height = src_height_str.trim().split_whitespace().last().unwrap_or_default();

    return Dimension {
        width: src_width.to_string(),
        height: src_height.to_string(),
    };
}

pub fn check_dimension(src_file: &String) {
    let size = get_dimension(src_file);
    if size.width.is_empty() {
        logger::error(&format!("The {} file is not an icons. Please check.", src_file));
        exit(-1);
    }

    if size.width != size.height {
        logger::warn("Unequal height and width in source icons may cause deformation.");
    }
}
