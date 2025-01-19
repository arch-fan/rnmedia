use args::Args;
use clap::Parser;
use media::Media;
use std::error::Error;
use std::fs;
use std::path::Path;

mod args;
mod media;

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let media = Media::from_path(&args.path, args.season, args.offset, &args.media)?;

    let transformed = media.transform_media();

    for (original, new) in transformed {
        if args.apply {
            let old_path = Path::new(&args.path).join(&original);
            let new_path = Path::new(&args.path).join(&new);
            fs::rename(old_path, new_path)?;
        } else {
            println!(
                "{} -> {}",
                color(original.as_str(), "1;96"),
                color(new.as_str(), "1;34")
            );
        }
    }

    Ok(())
}

fn color(str: &str, ansi: &str) -> String {
    format!("\x1b[{}m{}\x1b[0m", ansi, str)
}
