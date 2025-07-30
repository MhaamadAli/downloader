//! Video information model

use serde::{Deserialize, Serialize};
use crate::models::Format;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoInfo {
    pub title: String,
    pub duration: String,
    pub available_formats: Vec<Format>,
    pub thumbnail_url: String,
    pub video_id: String,
    pub uploader: Option<String>,
    pub upload_date: Option<String>,
}

impl VideoInfo {
    pub fn new(title: String, duration: String, video_id: String) -> Self {
        Self {
            title,
            duration,
            video_id,
            available_formats: Vec::new(),
            thumbnail_url: String::new(),
            uploader: None,
            upload_date: None,
        }
    }
    
    /// Add available format to video info
    pub fn add_format(&mut self, format: Format) {
        self.available_formats.push(format);
    }
    
    /// Get formats by type
    pub fn get_formats_by_type(&self, format_type: &crate::models::FormatType) -> Vec<&Format> {
        // TODO: Implement format filtering by type
        todo!("Implement format filtering")
    }
}