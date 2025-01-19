use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to the directory containing media files
    pub path: String,

    /// Actually apply the new filenames (if not provided, just prints preview)
    #[arg(short, long)]
    pub apply: bool,

    /// Season number
    #[arg(short, long)]
    pub season: i32,

    /// Media name/title
    #[arg(short = 'm', long = "media-name")]
    pub media_name: String,

    /// Episode offset (default 0)
    #[arg(short, long, default_value_t = 0)]
    pub offset: i32,
}
