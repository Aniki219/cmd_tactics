use serde::de::DeserializeOwned;

pub fn load_save<T: DeserializeOwned>(path: &str) -> Result<Option<T>, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(String::from("data/persistent/") + path)?;
    Ok(toml::from_str(&text)?)
}

// write_save

// The main concern is not corrupting the save if the program dies mid-write.
// Write to a temp file, then rename, because rename is atomic on common filesystems:

// rust
// pub fn write_save(path: &Path, save: &SaveData) -> Result<(), SaveError> {
//     let text = toml::to_string_pretty(save)?;
//     if let Some(dir) = path.parent() {
//         fs::create_dir_all(dir)?;
//     }
//     let tmp = path.with_extension("toml.tmp");
//     fs::write(&tmp, text)?;
//     fs::rename(&tmp, path)?;
//     Ok(())
// }
