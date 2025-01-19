use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    /// Path to the directory containing media files
    pub path: String,

    /// Actually apply the new filenames (if not provided, just prints preview)
    #[arg(short, long)]
    pub apply: bool,

    /// Season number
    #[arg(short, long)]
    pub season: i32,

    /// Media title
    #[arg(short, long)]
    pub media: String,

    /// Episode offset
    #[arg(short, long, default_value_t = 0)]
    pub offset: i32,
}
