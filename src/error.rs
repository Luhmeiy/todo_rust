use crate::{
    config::{AliasError, ConfigError},
    list::ListError,
    manager::ManagerError,
};

pub enum AppError {
    ConfigError(ConfigError),
    AliasError(AliasError),
    ListError(ListError),
    ManagerError(ManagerError),
    String(String),
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::ConfigError(e)
    }
}

impl From<AliasError> for AppError {
    fn from(e: AliasError) -> Self {
        AppError::AliasError(e)
    }
}

impl From<ListError> for AppError {
    fn from(e: ListError) -> Self {
        AppError::ListError(e)
    }
}

impl From<ManagerError> for AppError {
    fn from(e: ManagerError) -> Self {
        AppError::ManagerError(e)
    }
}

impl From<String> for AppError {
    fn from(e: String) -> Self {
        AppError::String(e)
    }
}

impl From<&str> for AppError {
    fn from(e: &str) -> Self {
        AppError::String(e.to_string())
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ConfigError(e) => e.fmt(f),
            AppError::AliasError(e) => e.fmt(f),
            AppError::ListError(e) => e.fmt(f),
            AppError::ManagerError(e) => e.fmt(f),
            AppError::String(e) => write!(f, "{}", e),
        }
    }
}
