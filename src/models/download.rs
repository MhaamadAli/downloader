//! Download task and progress models

use crate::models::{VideoInfo, Format};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct DownloadTask {
    pub video_info: VideoInfo,
    pub selected_format: Format,
    pub output_path: PathBuf,
    pub progress: DownloadProgress,
}

impl DownloadTask {
    pub fn new(video_info: VideoInfo, selected_format: Format, output_path: PathBuf) -> Self {
        Self {
            video_info,
            selected_format,
            output_path,
            progress: DownloadProgress::new(),
        }
    }
    
    /// Generate output filename
    pub fn generate_filename(&self) -> String {
        // TODO: Implement smart filename generation
        todo!("Implement filename generation")
    }
}

#[derive(Debug, Clone)]
pub struct DownloadProgress {
    pub total_size: u64,
    pub downloaded_size: u64,
    pub download_speed: f64,
    pub eta_seconds: u64,
    pub is_complete: bool,
}

impl DownloadProgress {
    pub fn new() -> Self {
        Self {
            total_size: 0,
            downloaded_size: 0,
            download_speed: 0.0,
            eta_seconds: 0,
            is_complete: false,
        }
    }
    
    /// Calculate completion percentage
    pub fn percentage(&self) -> f64 {
        if self.total_size == 0 {
            return 0.0;
        }
        (self.downloaded_size as f64 / self.total_size as f64) * 100.0
    }
    
    /// Format download speed for display
    pub fn speed_string(&self) -> String {
        // TODO: Implement speed formatting
        todo!("Implement speed formatting")
    }
    
    /// Format ETA for display
    pub fn eta_string(&self) -> String {
        // TODO: Implement ETA formatting
        todo!("Implement ETA formatting")
    }
}