//! URL and input validation utilities

use crate::Result;
use crate::error::DownloaderError;
use regex::Regex;
use std::sync::OnceLock;

pub struct UrlValidator;

impl UrlValidator {
    /// Get YouTube URL regex patterns (compiled once and cached)
    fn get_patterns() -> &'static (Regex, Regex, Regex) {
        static PATTERNS: OnceLock<(Regex, Regex, Regex)> = OnceLock::new();
        PATTERNS.get_or_init(|| {
            let youtube_pattern = Regex::new(
                r"^https?://(www\.)?(youtube\.com/watch\?v=|youtu\.be/|youtube\.com/embed/|youtube\.com/v/)([a-zA-Z0-9_-]{11})"
            ).expect("YouTube URL regex should be valid");
            
            let video_id_pattern = Regex::new(
                r"[a-zA-Z0-9_-]{11}"
            ).expect("Video ID regex should be valid");
            
            let playlist_pattern = Regex::new(
                r"[&?]list=([a-zA-Z0-9_-]+)"
            ).expect("Playlist regex should be valid");
            
            (youtube_pattern, video_id_pattern, playlist_pattern)
        })
    }
    
    /// Validate YouTube URL format
    pub fn is_valid_youtube_url(url: &str) -> bool {
        let (youtube_pattern, _, _) = Self::get_patterns();
        youtube_pattern.is_match(url)
    }
    
    /// Extract video ID from YouTube URL
    pub fn extract_video_id(url: &str) -> Result<String> {
        let (youtube_pattern, video_id_pattern, _) = Self::get_patterns();
        
        // First check if it's a valid YouTube URL
        if !youtube_pattern.is_match(url) {
            return Err(DownloaderError::InvalidUrl(url.to_string()));
        }
        
        // Extract video ID using different patterns
        if let Some(captures) = youtube_pattern.captures(url) {
            if let Some(video_id) = captures.get(3) {
                let id = video_id.as_str();
                if video_id_pattern.is_match(id) {
                    return Ok(id.to_string());
                }
            }
        }
        
        // Fallback: try to find video ID pattern anywhere in the URL
        if let Some(captures) = video_id_pattern.captures(url) {
            if let Some(video_id) = captures.get(0) {
                return Ok(video_id.as_str().to_string());
            }
        }
        
        Err(DownloaderError::InvalidUrl(format!("Could not extract video ID from: {}", url)))
    }
    
    /// Normalize YouTube URL to standard format
    pub fn normalize_url(url: &str) -> Result<String> {
        let video_id = Self::extract_video_id(url)?;
        Ok(format!("https://www.youtube.com/watch?v={}", video_id))
    }
    
    /// Validate file path for output
    pub fn is_valid_output_path(path: &str) -> bool {
        // Check for invalid characters in filename
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
        
        // Check for reserved names on Windows
        let reserved_names = [
            "CON", "PRN", "AUX", "NUL",
            "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8", "COM9",
            "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9"
        ];
        
        if path.is_empty() || path.len() > 255 {
            return false;
        }
        
        // Check for invalid characters
        if path.chars().any(|c| invalid_chars.contains(&c) || c.is_control()) {
            return false;
        }
        
        // Check for reserved names (Windows compatibility)
        let path_upper = path.to_uppercase();
        if reserved_names.iter().any(|&name| path_upper.starts_with(name)) {
            return false;
        }
        
        // Check for paths ending with space or period (Windows compatibility)
        if path.ends_with(' ') || path.ends_with('.') {
            return false;
        }
        
        true
    }
    
    /// Sanitize filename by removing/replacing invalid characters
    pub fn sanitize_filename(filename: &str) -> String {
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*', '/', '\\'];
        
        let mut sanitized = filename
            .chars()
            .map(|c| {
                if invalid_chars.contains(&c) || c.is_control() {
                    '_' 
                } else { 
                    c 
                }
            })
            .collect::<String>();
        
        // Trim spaces and periods from the end
        sanitized = sanitized.trim_end_matches([' ', '.']).to_string();
        
        // Ensure it's not empty
        if sanitized.is_empty() {
            sanitized = "video".to_string();
        }
        
        // Truncate if too long (leave room for extension)
        if sanitized.len() > 200 {
            sanitized.truncate(200);
        }
        
        sanitized
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_youtube_urls() {
        let valid_urls = [
            "https://www.youtube.com/watch?v=dQw4w9WgXcQ",
            "https://youtube.com/watch?v=dQw4w9WgXcQ",
            "https://youtu.be/dQw4w9WgXcQ",
            "https://www.youtube.com/embed/dQw4w9WgXcQ",
            "https://www.youtube.com/v/dQw4w9WgXcQ",
        ];
        
        for url in &valid_urls {
            assert!(UrlValidator::is_valid_youtube_url(url), "Should be valid: {}", url);
        }
    }
    
    #[test]
    fn test_invalid_youtube_urls() {
        let invalid_urls = [
            "https://example.com",
            "not-a-url",
            "https://vimeo.com/123456",
            "https://youtube.com/watch?v=invalidid",
            "https://youtube.com/watch?v=",
        ];
        
        for url in &invalid_urls {
            assert!(!UrlValidator::is_valid_youtube_url(url), "Should be invalid: {}", url);
        }
    }
    
    #[test]
    fn test_video_id_extraction() {
        let test_cases = [
            ("https://www.youtube.com/watch?v=dQw4w9WgXcQ", "dQw4w9WgXcQ"),
            ("https://youtu.be/dQw4w9WgXcQ", "dQw4w9WgXcQ"),
            ("https://www.youtube.com/embed/dQw4w9WgXcQ", "dQw4w9WgXcQ"),
        ];
        
        for (url, expected_id) in &test_cases {
            let result = UrlValidator::extract_video_id(url);
            assert!(result.is_ok(), "Should extract ID from: {}", url);
            assert_eq!(result.unwrap(), *expected_id);
        }
    }
}