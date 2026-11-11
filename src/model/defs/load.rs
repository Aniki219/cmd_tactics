use serde::de::DeserializeOwned;

// missing file -> fail loudly at startup.
// check cross-references within the defs, validate fk ids

pub fn load_defs<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(String::from("data/defs/") + path)?;
    Ok(toml::from_str(&text)?)
}
