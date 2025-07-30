//! Core library for YouTube Downloader
//! Exports main modules for use in tests and potential future API

pub mod cli;
pub mod config;
pub mod downloader;
pub mod error;
pub mod extractor;
pub mod file_system;
pub mod models;
pub mod ui;
pub mod utils;

// Re-export commonly used types
pub use error::{DownloaderError, Result};
pub use models::{VideoInfo, Format, DownloadTask};