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
            AliasError::AlreadyExists(alias) => write!(f, "Alias '{}' already exists", alias),
            AliasError::NoSymbol() => write!(f, "Alias should start with @"),
            AliasError::InvalidPath => write!(f, "Invalid path"),
            AliasError::NotFound(alias) => write!(f, "Alias '{}' not found", alias),
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

fn validate_alias_name(name: &str) -> Result<(), AliasError> {
    if !name.starts_with("@") {
        return Err(AliasError::NoSymbol());
    }
    Ok(())
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

    pub fn get_alias_names(&self) -> Vec<String> {
        self.aliases.keys().cloned().collect()
    }

    pub fn add_alias(&mut self, alias: String, path: PathBuf) -> Result<String, AliasError> {
        validate_alias_name(&alias)?;

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

    pub fn remove_alias(&mut self, alias: String) -> Result<String, AliasError> {
        validate_alias_name(&alias)?;

        match self.aliases.remove(&alias) {
            Some(_) => Ok(alias),
            None => Err(AliasError::NotFound(alias)),
        }
    }

    pub fn rename_alias(
        &mut self,
        old_name: String,
        new_name: String,
    ) -> Result<(String, String), AliasError> {
        validate_alias_name(&old_name)?;
        validate_alias_name(&new_name)?;

        let path = self
            .aliases
            .get(&old_name)
            .ok_or(AliasError::NotFound(old_name.to_string()))?
            .clone();

        if self.aliases.contains_key(&new_name) {
            return Err(AliasError::AlreadyExists(new_name));
        }

        self.aliases.remove(&old_name);
        self.aliases.insert(new_name.clone(), path);

        Ok((old_name, new_name))
    }

    pub fn update_path_alias(
        &mut self,
        name: String,
        new_path: String,
    ) -> Result<(String, String), AliasError> {
        validate_alias_name(&name)?;

        match self.aliases.get_mut(&name) {
            Some(path) => {
                let absolute_path = fs::canonicalize(PathBuf::from(new_path.clone()))
                    .map_err(|_| AliasError::InvalidPath)?;
                *path = absolute_path;
                Ok((name, new_path))
            }
            None => Err(AliasError::NotFound(name.to_string())),
        }
    }

    pub fn change_path(&mut self, path: PathBuf) -> () {
        self.path = path
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let json = serde_json::to_string_pretty(self).map_err(ConfigError::JsonError)?;
        fs::write(PathBuf::from("./config.json"), json).map_err(|_| ConfigError::SaveFailed)
    }

    pub fn save_with_warning(&self) {
        if let Err(e) = self.save() {
            eprintln!("{}", format!("Warning: {e}").yellow())
        }
    }

    pub fn load() -> Result<Self, ConfigError> {
        let path = PathBuf::from("./config.json");
        let content = fs::read_to_string(path).map_err(|_| ConfigError::LoadFailed)?;
        let mut config: Config = serde_json::from_str(&content).map_err(ConfigError::JsonError)?;

        for (_, path) in config.aliases.iter_mut() {
            if !path.is_absolute() {
                if let Ok(absolute) = fs::canonicalize(&path) {
                    *path = absolute;
                }
            }
        }

        Ok(config)
    }
}
