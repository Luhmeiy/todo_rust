use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub enum ConfigError {
    JsonError(serde_json::Error),
    SaveFailed,
    LoadFailed,
}

pub enum AliasError {
    AlreadyExists(String),
    NoSymbol(),
    InvalidPath,
    NotFound(String),
    Empty,
}

impl std::fmt::Display for AliasError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AliasError::AlreadyExists(name) => write!(f, "Alias '{}' already exists", name),
            AliasError::NoSymbol() => write!(f, "Alias should start with @"),
            AliasError::InvalidPath => write!(f, "Invalid path"),
            AliasError::NotFound(name) => write!(f, "Alias '{}' not found", name),
            AliasError::Empty => write!(f, "No aliases found"),
        }
    }
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
    aliases: HashMap<String, PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            path: PathBuf::from("./todo_data.json"),
            aliases: HashMap::from([("@todo".to_string(), PathBuf::from("./todo_data.json"))]),
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }

    pub fn get_path_from_alias(&self, alias: &str) -> Result<&Path, AliasError> {
        match self.aliases.get(alias) {
            Some(path) => Ok(path),
            None => Err(AliasError::NotFound(alias.to_string())),
        }
    }

    pub fn add_alias(&mut self, alias: String, path: PathBuf) -> Result<String, AliasError> {
        if !alias.starts_with("@") {
            return Err(AliasError::NoSymbol());
        }

        if self.aliases.contains_key(&alias) {
            return Err(AliasError::AlreadyExists(alias.clone()));
        }

        let absolute_path = fs::canonicalize(&path).map_err(|_| AliasError::InvalidPath)?;

        match self.aliases.insert(alias.clone(), absolute_path) {
            Some(_) => Err(AliasError::AlreadyExists(alias)),
            None => Ok(alias),
        }
    }

    pub fn list_alias(&self) -> Result<(), AliasError> {
        if self.aliases.is_empty() {
            return Err(AliasError::Empty);
        }

        for (alias, path) in self.aliases.iter() {
            let alias_str = format!("{alias}:");
            println!("{} {path:?}", alias_str.cyan())
        }

        Ok(())
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
        let mut config: Config = serde_json::from_str(&content).map_err(ConfigError::JsonError)?;

        for (_alias, path) in config.aliases.iter_mut() {
            if !path.is_absolute() {
                if let Ok(absolute) = fs::canonicalize(&path) {
                    *path = absolute;
                }
            }
        }

        Ok(config)
    }
}
