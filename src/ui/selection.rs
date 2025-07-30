//! Format and quality selection user interface

use crate::models::{Format, FormatType, VideoInfo};
use crate::Result;
use console::{style, Term};

pub struct SelectionUI {
    term: Term,
}

impl SelectionUI {
    pub fn new() -> Self {
        Self {
            term: Term::stdout(),
        }
    }
    
    /// Display video information
    pub fn display_video_info(&self, video_info: &VideoInfo) -> Result<()> {
        // TODO: Implement video information display
        todo!("Implement video info display")
    }
    
    /// Interactive format type selection
    pub fn select_format_type(&self) -> Result<FormatType> {
        // TODO: Implement format type selection UI
        todo!("Implement format type selection")
    }
    
    /// Interactive quality selection
    pub fn select_quality(&self, formats: &[Format]) -> Result<usize> {
        // TODO: Implement quality selection UI
        todo!("Implement quality selection")
    }
    
    // TODO: Add helper methods for formatting output
    fn format_duration(&self, seconds: u64) -> String {
        todo!("Implement duration formatting")
    }
    
    fn format_file_size(&self, bytes: u64) -> String {
        todo!("Implement file size formatting")
    }
}