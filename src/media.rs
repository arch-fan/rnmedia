use std::{fs, io, path::Path};

use indexmap::IndexMap;

pub struct Media {
    pub chapters: Vec<String>,
    pub to_season: i32,
    pub to_offset: i32,
    pub to_title: String,
}

impl Media {
    pub fn from_path(
        path: impl AsRef<Path>,
        season: i32,
        offset: i32,
        title: &str,
    ) -> io::Result<Self> {
        let mut entries: Vec<String> = fs::read_dir(path)?
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect();

        entries.sort();

        Ok(Media {
            chapters: entries,
            to_offset: offset,
            to_season: season,
            to_title: title.to_string(),
        })
    }

    /// Transform a list of original names into a `HashMap` mapping
    /// old names -> new names with the pattern:
    ///   `{media_name} - S{season:02}E{episode:02}.{extension}`
    pub fn transform_media(&self) -> IndexMap<String, String> {
        let mut mapping = IndexMap::new();

        for (i, name) in self.chapters.iter().enumerate() {
            // If there's a '.', take everything after the last '.' as extension.
            let file_type = match name.rfind('.') {
                Some(pos) => &name[pos + 1..],
                None => name, // No '.' => treat entire name as extension
            };

            let episode_num = self.to_offset + (i as i32) + 1;
            let new_name = format!(
                "{} - S{:02}E{:02}.{}",
                self.to_title, self.to_season, episode_num, file_type
            );

            mapping.insert(name.clone(), new_name);
        }

        mapping
    }
}
