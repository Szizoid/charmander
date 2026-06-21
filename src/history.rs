use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct History {
    #[serde(default)]
    pub counts: HashMap<String, u32>,
}

impl History {
    pub fn load(path: &PathBuf) -> Self {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };
        toml::from_str(&content).unwrap_or_default()
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_count(&self, symbol: &str) -> u32 {
        *self.counts.get(symbol).unwrap_or(&0)
    }

    pub fn increment(&mut self, symbol: &str) {
        let count = self.counts.entry(symbol.to_string()).or_insert(0);
        *count += 1;
    }
}
