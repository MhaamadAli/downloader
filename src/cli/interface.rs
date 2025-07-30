//! Interactive command-line interface components

use crate::models::{Format, FormatType};
use crate::Result;
use std::io::{self, Write};

pub struct InteractiveInterface;

impl InteractiveInterface {
    pub fn new() -> Self {
        Self
    }
    
    /// Present format selection to user (Audio vs Video)
    pub fn select_format_type(&self) -> Result<FormatType> {
        // TODO: Implement interactive format type selection
        todo!("Implement format type selection UI")
    }
    
    /// Present quality selection to user
    pub fn select_quality(&self, available_formats: &[Format]) -> Result<Format> {
        // TODO: Implement interactive quality selection
        todo!("Implement quality selection UI")
    }
    
    /// Confirm download for large files
    pub fn confirm_large_download(&self, file_size: u64) -> Result<bool> {
        // TODO: Implement download confirmation for large files
        todo!("Implement large download confirmation")
    }
    
    // TODO: Add helper methods for user input
    fn get_user_input(&self, prompt: &str) -> Result<String> {
        todo!("Implement user input helper")
    }
}