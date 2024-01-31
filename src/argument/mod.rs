use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub src: String,

    #[arg(short, long)]
    pub dst: String,

    #[arg(short, long)]
    pub icons: Option<String>,
}