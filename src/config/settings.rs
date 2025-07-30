//! Application settings and configuration

use serde::{Deserialize, Serialize};
use crate::Result;
use crate::error::DownloaderError;
use std::path::PathBuf;
use std::fs;
use log::{debug, warn};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    /// Default directory for downloaded videos
    pub default_output_directory: Option<PathBuf>,
    /// Preferred video quality (e.g., "1080p", "720p", "best", "worst")
    pub default_quality: Option<String>,
    /// Maximum number of concurrent chunk downloads
    pub max_concurrent_downloads: usize,
    /// Size of each download chunk in bytes
    pub chunk_size: usize,
    /// Automatically resume interrupted downloads
    pub auto_resume: bool,
    /// Ask for confirmation before downloading large files
    pub confirm_large_downloads: bool,
    /// Threshold for considering a download "large" (in bytes)
    pub large_download_threshold: u64,
    /// Maximum retries for failed downloads
    pub max_retries: u32,
    /// Timeout for network requests in seconds
    pub request_timeout: u64,
    /// Whether to prefer audio-only downloads by default
    pub prefer_audio_only: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            default_output_directory: None, // Will be resolved to ~/Downloads/YouTube
            default_quality: Some("best".to_string()),
            max_concurrent_downloads: 4,
            chunk_size: 1024 * 1024, // 1MB chunks - good balance of speed and memory usage
            auto_resume: true,
            confirm_large_downloads: true,
            large_download_threshold: 100 * 1024 * 1024, // 100MB
            max_retries: 3,
            request_timeout: 30,
            prefer_audio_only: false,
        }
    }
}

impl Settings {
    /// Load settings from config file, falling back to defaults
    pub fn load() -> Result<Self> {
        match Self::load_from_file() {
            Ok(settings) => {
                debug!("Loaded settings from config file");
                Ok(settings)
            }
            Err(e) => {
                debug!("Failed to load config file, using defaults: {}", e);
                Ok(Self::default())
            }
        }
    }
    
    /// Load settings from the config file
    fn load_from_file() -> Result<Self> {
        let config_path = Self::get_config_path()?;
        
        if !config_path.exists() {
            return Err(DownloaderError::Configuration("Config file does not exist".to_string()));
        }
        
        let config_content = fs::read_to_string(&config_path)?;
        let settings: Settings = toml::from_str(&config_content)?;
        
        Ok(settings)
    }
    
    /// Save current settings to config file
    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let config_content = toml::to_string_pretty(self)
            .map_err(|e| DownloaderError::Configuration(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(&config_path, config_content)?;
        debug!("Settings saved to: {}", config_path.display());
        
        Ok(())
    }
    
    /// Get the path to the config file
    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| DownloaderError::Configuration("Could not find config directory".to_string()))?;
        
        let app_config_dir = config_dir.join("downloader");
        Ok(app_config_dir.join("config.toml"))
    }
    
    /// Get effective output directory (resolves default if not set)
    pub fn get_output_directory(&self) -> Result<PathBuf> {
        if let Some(ref custom_dir) = self.default_output_directory {
            return Ok(custom_dir.clone());
        }
        
        // Default to ~/Downloads/YouTube
        let downloads_dir = dirs::download_dir()
            .ok_or_else(|| DownloaderError::Configuration("Could not find downloads directory".to_string()))?;
        
        Ok(downloads_dir.join("YouTube"))
    }
    
    /// Create a sample config file with comments
    pub fn create_sample_config() -> Result<()> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            return Err(DownloaderError::Configuration("Config file already exists".to_string()));
        }
        
        // Ensure config directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let sample_config = r#"# YouTube Downloader Configuration
# All settings are optional - remove any line to use the default value

# Directory where videos will be downloaded
# If not specified, defaults to ~/Downloads/YouTube
# default_output_directory = "/path/to/your/downloads"

# Preferred video quality: "best", "worst", "1080p", "720p", "480p", etc.
# If not specified, defaults to "best"
default_quality = "best"

# Maximum number of concurrent chunk downloads (1-8 recommended)
# Higher values = faster downloads but more CPU/memory usage
max_concurrent_downloads = 4

# Size of each download chunk in bytes (1MB = 1048576)
# Larger chunks = fewer HTTP requests but more memory usage
chunk_size = 1048576

# Automatically resume interrupted downloads
auto_resume = true

# Ask for confirmation before downloading large files
confirm_large_downloads = true

# Threshold for considering a download "large" in bytes (100MB = 104857600)
large_download_threshold = 104857600

# Maximum retries for failed downloads
max_retries = 3

# Timeout for network requests in seconds
request_timeout = 30

# Prefer audio-only downloads by default
prefer_audio_only = false
"#;
        
        fs::write(&config_path, sample_config)?;
        println!("Sample configuration created at: {}", config_path.display());
        
        Ok(())
    }
    
    /// Validate settings and return warnings for problematic values
    pub fn validate(&self) -> Vec<String> {
        let mut warnings = Vec::new();
        
        if self.max_concurrent_downloads == 0 {
            warnings.push("max_concurrent_downloads cannot be 0, using 1".to_string());
        } else if self.max_concurrent_downloads > 8 {
            warnings.push("max_concurrent_downloads > 8 may cause issues with some servers".to_string());
        }
        
        if self.chunk_size < 64 * 1024 {
            warnings.push("chunk_size < 64KB may result in too many HTTP requests".to_string());
        } else if self.chunk_size > 10 * 1024 * 1024 {
            warnings.push("chunk_size > 10MB may use excessive memory".to_string());
        }
        
        if self.request_timeout < 5 {
            warnings.push("request_timeout < 5 seconds may cause timeouts on slow connections".to_string());
        }
        
        if let Some(ref output_dir) = self.default_output_directory {
            if !output_dir.exists() {
                warnings.push(format!("Output directory does not exist: {}", output_dir.display()));
            }
        }
        
        warnings
    }
    
    /// Get effective max concurrent downloads (ensuring it's at least 1)
    pub fn effective_max_concurrent_downloads(&self) -> usize {
        std::cmp::max(1, self.max_concurrent_downloads)
    }
    
    /// Get effective chunk size (ensuring it's reasonable)
    pub fn effective_chunk_size(&self) -> usize {
        std::cmp::max(64 * 1024, std::cmp::min(10 * 1024 * 1024, self.chunk_size))
    }
}