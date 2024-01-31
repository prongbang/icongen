use std::path::Path;
use std::process::Command;
use crate::argument::Args;
use crate::icons::config::Sizes;
use crate::logger;

pub fn execute(args: &Args, icon_sizes: Sizes) {
    let src_file = &args.src;
    let dst_path = &args.dst;

    let srgb_profile = "/System/Library/ColorSync/Profiles/sRGB Profile.icc";

    for icon in icon_sizes.icons.iter() {
        let name = String::from(icon.0);
        let size = icon.1.to_string();

        logger::info(&format!("Generate {}.png", &name));

        let mut sips_command = Command::new("sips");
        sips_command.arg("-z")
            .arg(&size)
            .arg(&size)
            .arg(src_file)
            .arg("--out")
            .arg(format!("{}/{}.png", dst_path, &name));

        if Path::new(srgb_profile).exists() {
            sips_command.arg("--matchTo").arg(srgb_profile);
        }

        // Redirect standard output and standard error to /dev/null
        sips_command
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());

        if let Err(err) = sips_command.status() {
            logger::error(&format!("Sips command execution failed: {}", err));
        }
    }

    logger::info(&format!("Generate successfully in directory: {}", dst_path));
}
