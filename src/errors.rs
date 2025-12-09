use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Network error: {0}")]
    Network(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
