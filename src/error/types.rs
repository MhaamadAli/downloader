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
    
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
    
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),
    
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
    
    #[error("Video not found or unavailable")]
    VideoNotFound,
    
    #[error("No suitable formats found")]
    NoFormatsFound,
    
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl DownloaderError {
    /// Check if error is recoverable (worth retrying)
    pub fn is_recoverable(&self) -> bool {
        match self {
            // Network errors are often temporary
            DownloaderError::Network(e) => {
                // Some network errors are permanent (404, 403), others are temporary
                !e.is_status() || e.status().map_or(true, |s| s.is_server_error())
            },
            // Resume failures can often be retried
            DownloaderError::ResumeFailed(_) => true,
            // These are permanent failures
            DownloaderError::UserCancelled => false,
            DownloaderError::InvalidUrl(_) => false,
            DownloaderError::InsufficientSpace => false,
            DownloaderError::VideoNotFound => false,
            DownloaderError::NoFormatsFound => false,
            // Everything else might be recoverable
            _ => true,
        }
    }
    
    /// Get user-friendly error message
    pub fn user_message(&self) -> String {
        match self {
            DownloaderError::InvalidUrl(url) => {
                format!("The URL '{}' is not a valid YouTube URL", url)
            },
            DownloaderError::VideoNotFound => {
                "Video not found or is not available (may be private or deleted)".to_string()
            },
            DownloaderError::NoFormatsFound => {
                "No downloadable formats found for this video".to_string()
            },
            DownloaderError::Network(e) if e.is_timeout() => {
                "Network timeout - please check your internet connection".to_string()
            },
            DownloaderError::Network(e) if e.is_connect() => {
                "Failed to connect to YouTube - please check your internet connection".to_string()
            },
            DownloaderError::InsufficientSpace => {
                "Not enough disk space to download this video".to_string()
            },
            DownloaderError::UserCancelled => {
                "Download cancelled by user".to_string()
            },
            _ => self.to_string(),
        }
    }
}