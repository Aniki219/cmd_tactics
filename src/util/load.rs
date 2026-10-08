use serde::de::DeserializeOwned;

pub fn load_toml<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    Ok(toml::from_str(&text)?)
}
