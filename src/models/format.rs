//! Format and quality models

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FormatType {
    Audio,
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Format {
    pub quality: String,
    pub format_type: FormatType,
    pub file_extension: String,
    pub download_url: String,
    pub file_size: Option<u64>,
    pub bitrate: Option<u32>,
    pub codec: Option<String>,
}

impl Format {
    pub fn new(
        quality: String,
        format_type: FormatType,
        file_extension: String,
        download_url: String,
    ) -> Self {
        Self {
            quality,
            format_type,
            file_extension,
            download_url,
            file_size: None,
            bitrate: None,
            codec: None,
        }
    }
    
    /// Get human-readable quality description
    pub fn quality_description(&self) -> String {
        // TODO: Implement quality description formatting
        todo!("Implement quality description")
    }
    
    /// Check if format is high quality
    pub fn is_high_quality(&self) -> bool {
        // TODO: Implement quality assessment
        todo!("Implement quality assessment")
    }
}