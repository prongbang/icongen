mod logger;
mod command;
mod icons;
mod filex;
mod argument;

use clap::Parser;
use crate::argument::Args;

fn main() {
    let args = Args::parse();

    let src_file = &args.src;
    let dst_path = &args.dst;
    let icons_path = &args.icons;

    // Check sips command
    command::check("sips");

    // Check source file
    filex::check_exists(src_file);

    // Check width and height
    icons::check_dimension(src_file);

    // Create destination directory
    filex::create_dir(dst_path);

    // Icon sizes
    let icon_sizes = icons::config::combine_icon_sizes(icons_path);

    // Generate icons
    icons::generate::execute(&args, icon_sizes);
}