//! Custom error types for the application

use thiserror::Error;

pub type Result<T> = std::result::Result<T, DownloaderError>;

#[derive(Error, Debug)]
pub enum DownloaderError {
    #[error("Invalid YouTube URL: {0}")]
    InvalidUrl(String),
    
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Video extraction failed: {0}")]
    ExtractionFailed(String),
    
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    
    #[error("File system error: {0}")]
    FileSystem(String),
    
    #[error("User cancelled operation")]
    UserCancelled,
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Insufficient disk space")]
    InsufficientSpace,
    
    #[error("Resume failed: {0}")]
    ResumeFailed(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl DownloaderError {
    /// Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            DownloaderError::Network(_) => true,
            DownloaderError::ResumeFailed(_) => true,
            DownloaderError::UserCancelled => false,
            DownloaderError::InvalidUrl(_) => false,
            DownloaderError::InsufficientSpace => false,
            _ => true,
        }
    }
    
    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        // TODO: Implement user-friendly error messages
        todo!("Implement user-friendly error messages")
    }
}