use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

pub enum ConfigError {
    JsonError(serde_json::Error),
    SaveFailed,
    LoadFailed,
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::JsonError(e) => write!(f, "JSON error: {e}"),
            ConfigError::SaveFailed => write!(f, "Failed to save config"),
            ConfigError::LoadFailed => write!(f, "Failed to load config"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    path: PathBuf,
}

impl Config {
    pub fn new() -> Self {
        Config {
            path: PathBuf::from("./todo_data.json"),
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn change_path(&mut self, path: PathBuf) -> () {
        self.path = path
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let json = serde_json::to_string_pretty(self).map_err(ConfigError::JsonError)?;
        fs::write(PathBuf::from("./config.json"), json).map_err(|_| ConfigError::SaveFailed)
    }

    pub fn load() -> Result<Self, ConfigError> {
        let path = PathBuf::from("./config.json");
        let content = fs::read_to_string(path).map_err(|_| ConfigError::LoadFailed)?;
        serde_json::from_str(&content).map_err(ConfigError::JsonError)
    }
}
