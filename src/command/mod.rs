use std::process::{Command, exit};
use crate::logger;

pub fn check(command: &str) {
    let mut cmd = Command::new(command);

    // Redirect standard output and standard error to /dev/null
    cmd
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null());

    if let Err(_) = cmd.status() {
        logger::error(&format!("Install command {} first, please.", command));
        exit(1);
    }
}