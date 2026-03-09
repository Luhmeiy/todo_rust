use crate::{list::ListError, manager::ManagerError};

pub enum AppError {
    ListError(ListError),
    ManagerError(ManagerError),
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
            AppError::ListError(e) => e.fmt(f),
            AppError::ManagerError(e) => e.fmt(f),
        }
    }
}
