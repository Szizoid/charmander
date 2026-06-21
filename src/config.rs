use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterEntry {
    pub symbol: String,
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum OutputMethod {
    Wtype,
    WlCopy,
    Ydotool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub output_method: OutputMethod,
    pub max_results: usize,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub characters: Vec<CharacterEntry>,
}

pub fn load(path: &PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
