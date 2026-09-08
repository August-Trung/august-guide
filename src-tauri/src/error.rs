use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("File I/O error: {0}")]
    FileIO(String),

    #[error("Screen capture error: {0}")]
    ScreenCapture(String),

    #[error("Image processing error: {0}")]
    ImageProcessing(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Export error: {0}")]
    Export(String),

    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("OAuth error: {0}")]
    OAuth(String),

    #[error("Google Drive API error: {0}")]
    GDrive(String),

    #[error("Generic error: {0}")]
    Generic(String),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileIO(err.to_string())
    }
}

pub type AppResult<T> = std::result::Result<T, AppError>;

