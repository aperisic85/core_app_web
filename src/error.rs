use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to parse request: {0}")]
    ParseError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unexpected error: {0}")]
    Unexpected(String),
}
