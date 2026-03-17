use crate::{config::ConfigError, list::ListError, manager::ManagerError};

pub enum AppError {
    ConfigError(ConfigError),
    ListError(ListError),
    ManagerError(ManagerError),
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::ConfigError(e)
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

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ConfigError(e) => e.fmt(f),
            AppError::ListError(e) => e.fmt(f),
            AppError::ManagerError(e) => e.fmt(f),
        }
    }
}
