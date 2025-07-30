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
        if self.download_speed < 1024.0 {
            format!("{:.1} B/s", self.download_speed)
        } else if self.download_speed < 1024.0 * 1024.0 {
            format!("{:.1} KB/s", self.download_speed / 1024.0)
        } else if self.download_speed < 1024.0 * 1024.0 * 1024.0 {
            format!("{:.1} MB/s", self.download_speed / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB/s", self.download_speed / (1024.0 * 1024.0 * 1024.0))
        }
    }
    
    /// Format ETA for display
    pub fn eta_string(&self) -> String {
        if self.eta_seconds == 0 {
            "Unknown".to_string()
        } else if self.eta_seconds < 60 {
            format!("{}s", self.eta_seconds)
        } else if self.eta_seconds < 3600 {
            let minutes = self.eta_seconds / 60;
            let seconds = self.eta_seconds % 60;
            format!("{}m {}s", minutes, seconds)
        } else {
            let hours = self.eta_seconds / 3600;
            let minutes = (self.eta_seconds % 3600) / 60;
            format!("{}h {}m", hours, minutes)
        }
    }
    
    /// Format total size for display
    pub fn size_string(&self) -> String {
        Self::format_bytes(self.total_size)
    }
    
    /// Format downloaded size for display
    pub fn downloaded_string(&self) -> String {
        Self::format_bytes(self.downloaded_size)
    }
    
    /// Format bytes into human readable string
    pub fn format_bytes(bytes: u64) -> String {
        if bytes < 1024 {
            format!("{} B", bytes)
        } else if bytes < 1024 * 1024 {
            format!("{:.1} KB", bytes as f64 / 1024.0)
        } else if bytes < 1024 * 1024 * 1024 {
            format!("{:.1} MB", bytes as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}