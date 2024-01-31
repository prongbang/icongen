pub fn info(message: &str) {
    println!("[INFO] {}", message);
}

pub fn error(message: &str) {
    eprintln!("[ERROR] {}", message);
}

pub fn warn(message: &str) {
    println!("[WARNING] {}", message);
}
