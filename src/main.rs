use args::Args;
use clap::Parser;
use indexmap::IndexMap;
use std::error::Error;
use std::fs;
use std::path::Path;

mod args;

/// Transform a list of original names into a `HashMap` mapping
/// old names -> new names with the pattern:
///   `{media_name} - S{season:02}E{episode:02}.{extension}`
fn transform_names(
    media_name: &str,
    season: i32,
    original_names: &[String],
    offset: i32,
) -> IndexMap<String, String> {
    let mut mapping = IndexMap::new();

    for (i, name) in original_names.iter().enumerate() {
        // If there's a '.', take everything after the last '.' as extension.
        let file_type = match name.rfind('.') {
            Some(pos) => &name[pos + 1..],
            None => name, // No '.' => treat entire name as extension
        };

        let episode_num = offset + (i as i32) + 1;
        let new_name = format!(
            "{} - S{:02}E{:02}.{}",
            media_name, season, episode_num, file_type
        );

        mapping.insert(name.clone(), new_name);
    }

    mapping
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut entries: Vec<String> = fs::read_dir(&args.path)?
        .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
        .collect();

    // Sort the filenames as the Python code does with `sorted(os.listdir(path))`
    entries.sort();

    // Transform the names
    let transformed = transform_names(&args.media_name, args.season, &entries, args.offset);

    // Either rename or print
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
