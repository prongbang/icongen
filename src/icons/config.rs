use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::filex;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sizes {
    pub icons: HashMap<String, u32>,
}

pub fn icon_sizes() -> Sizes {
    let icons: HashMap<String, u32> = [
        ("Icon-16".to_string(), 16),
        ("Icon-16@2x".to_string(), 32),
        ("Icon-32".to_string(), 32),
        ("Icon-32@2x".to_string(), 64),
        ("Icon-128".to_string(), 128),
        ("Icon-128@2x".to_string(), 256),
        ("Icon-256".to_string(), 256),
        ("Icon-256@2x".to_string(), 256),
        ("Icon-512".to_string(), 512),
        ("Icon-512@2x".to_string(), 1024),
        ("Icon-20".to_string(), 20),
        ("Icon-20@2x".to_string(), 40),
        ("Icon-20@3x".to_string(), 60),
        ("Icon-29".to_string(), 29),
        ("Icon-29@2x".to_string(), 58),
        ("Icon-29@3x".to_string(), 87),
        ("Icon-38".to_string(), 38),
        ("Icon-38@2x".to_string(), 76),
        ("Icon-38@3x".to_string(), 114),
        ("Icon-40".to_string(), 40),
        ("Icon-40@2x".to_string(), 80),
        ("Icon-40@3x".to_string(), 120),
        ("Icon-60".to_string(), 60),
        ("Icon-60@2x".to_string(), 120),
        ("Icon-60@3x".to_string(), 180),
        ("Icon-64".to_string(), 64),
        ("Icon-64@2x".to_string(), 128),
        ("Icon-64@3x".to_string(), 192),
        ("Icon-68".to_string(), 68),
        ("Icon-68@2x".to_string(), 136),
        ("Icon-76".to_string(), 76),
        ("Icon-76@2x".to_string(), 152),
        ("Icon-83.5@2x".to_string(), 167),
        ("Icon-1024".to_string(), 1024),
        ("Icon-24@2x".to_string(), 48),
        ("Icon-27.5@2x".to_string(), 55),
        ("Icon-86@2x".to_string(), 172),
        ("Icon-98@2x".to_string(), 196),
        ("Icon-108@2x".to_string(), 216),
        ("Icon-44@2x".to_string(), 88),
        ("Icon-50@2x".to_string(), 100),
    ]
        .iter()
        .cloned()
        .collect();

    return Sizes { icons };
}

pub fn load_icon_sizes(file_path: &str) -> Sizes {
    // Read the YAML file content
    let file_content = filex::read_file(file_path).expect("File read error.");

    // Deserialize the YAML content into the Sizes struct
    let sizes: Sizes = serde_yaml::from_str(&file_content).expect("YAML deserialization error.");

    sizes
}

pub fn combine_icon_sizes(file_path: &Option<String>) -> Sizes {
    let mut icons = icon_sizes();

    if let Some(file_path) = file_path {
        let new_icons = load_icon_sizes(file_path.as_str());
        for icon in new_icons.icons.iter() {
            icons.icons.insert(icon.0.clone(), icon.1.clone());
        }
    }

    return icons;
}