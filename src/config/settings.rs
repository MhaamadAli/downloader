//! Application settings and configuration

use serde::{Deserialize, Serialize};
use crate::Result;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub default_output_directory: Option<PathBuf>,
    pub default_quality: Option<String>,
    pub max_concurrent_downloads: usize,
    pub chunk_size: usize,
    pub auto_resume: bool,
    pub confirm_large_downloads: bool,
    pub large_download_threshold: u64, // in bytes
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_output_directory: None,
            default_quality: None,
            max_concurrent_downloads: 4,
            chunk_size: 1024 * 1024, // 1MB chunks
            auto_resume: true,
            confirm_large_downloads: true,
            large_download_threshold: 100 * 1024 * 1024, // 100MB
        }
    }
}

impl Settings {
    /// Load settings from config file
    pub fn load() -> Result<Self> {
        // TODO: Implement settings loading from config file
        todo!("Implement settings loading")
    }
    
    /// Save settings to config file
    pub fn save(&self) -> Result<()> {
        // TODO: Implement settings saving
        todo!("Implement settings saving")
    }
    
    /// Get effective output directory
    pub fn get_output_directory(&self) -> Result<PathBuf> {
        // TODO: Implement output directory resolution
        todo!("Implement output directory resolution")
    }
}