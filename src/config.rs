use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

const DEFAULT_CONFIG: &str = include_str!("../config/default.toml");

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterEntry {
    pub symbol: String,
    pub name: String,
    pub tags: Vec<String>,
}

fn default_selection_indicator() -> String { "> ".to_string() }
fn default_no_selection_indicator() -> String { "  ".to_string() }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub max_results: usize,
    #[serde(default = "default_selection_indicator")]
    pub selection_indicator: String,
    #[serde(default = "default_no_selection_indicator")]
    pub no_selection_indicator: String,
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

pub fn create_default_if_missing(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if !path.exists() {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, DEFAULT_CONFIG)?;
    }
    Ok(())
}
