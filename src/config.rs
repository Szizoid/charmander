use std::fs;
use std::path::PathBuf;

use serde::Deserialize;

// Embedded at compile time so the binary ships with defaults and needs no separate data files.
const DEFAULT_CONFIG: &str = include_str!("../config/default.toml");

#[derive(Debug, Deserialize, Clone)]
pub struct CharacterEntry {
    pub symbol: String,
    pub name: String,
    pub tags: Vec<String>,
}

// serde requires function pointers, not string literals, for field-level defaults.
fn default_selection_indicator() -> String { "> ".to_string() }
fn default_no_selection_indicator() -> String { "  ".to_string() }

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Settings {
    pub max_results: usize,
    // Defaults allow old configs without these fields to still load cleanly.
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

pub fn restore_default(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if path.exists() {
        let backup = path.with_extension("toml.bak");
        fs::rename(path, &backup)?;
        println!("Backup saved to {}", backup.display());
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, DEFAULT_CONFIG)?;
    println!("Default config written to {}", path.display());
    Ok(())
}
