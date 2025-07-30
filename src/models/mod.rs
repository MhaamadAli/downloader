//! Data models and structures

pub mod video;
pub mod format;
pub mod download;

pub use video::VideoInfo;
pub use format::{Format, FormatType};
pub use download::{DownloadTask, DownloadProgress};