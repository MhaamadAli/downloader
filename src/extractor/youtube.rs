//! YouTube-specific video information extraction

use crate::models::{VideoInfo, Format, FormatType};
use crate::utils::{UrlValidator, NetworkUtils};
use crate::Result;
use crate::error::DownloaderError;
use reqwest::Client;
use regex::Regex;
use serde_json::Value;
use log::{debug, warn, error};
use std::sync::OnceLock;

pub struct YouTubeExtractor {
    client: Client,
}

impl YouTubeExtractor {
    pub fn new() -> Result<Self> {
        let client = NetworkUtils::create_client()?;
        Ok(Self { client })
    }
    
    /// Extract video information from YouTube URL
    pub async fn extract_video_info(&self, url: &str) -> Result<VideoInfo> {
        debug!("Extracting video info from: {}", url);
        
        // 1. Validate and extract video ID
        let video_id = UrlValidator::extract_video_id(url)?;
        let normalized_url = UrlValidator::normalize_url(url)?;
        
        // 2. Fetch the YouTube page
        let html = self.fetch_page(&normalized_url).await?;
        
        // 3. Parse video metadata and formats
        let mut video_info = self.parse_video_page(&html, &video_id)?;
        
        // 4. Extract and filter formats (MP4 video and MP3 audio only)
        let formats = self.extract_formats(&html)?;
        let filtered_formats = self.filter_formats(formats);
        
        if filtered_formats.is_empty() {
            return Err(DownloaderError::NoFormatsFound);
        }
        
        video_info.available_formats = filtered_formats;
        
        debug!("Successfully extracted info for video: {}", video_info.title);
        Ok(video_info)
    }
    
    /// Fetch YouTube page HTML
    async fn fetch_page(&self, url: &str) -> Result<String> {
        debug!("Fetching YouTube page: {}", url);
        
        let response = NetworkUtils::retry_with_backoff(
            || async { 
                self.client.get(url).send().await.map_err(DownloaderError::Network)
            },
            3
        ).await?;
        
        if !response.status().is_success() {
            return Err(DownloaderError::VideoNotFound);
        }
        
        let html = response.text().await?;
        debug!("Successfully fetched page ({} bytes)", html.len());
        Ok(html)
    }
    
    /// Parse video page HTML to extract basic video information
    fn parse_video_page(&self, html: &str, video_id: &str) -> Result<VideoInfo> {
        // Extract title using regex from various possible locations
        let title = self.extract_title(html)?;
        
        // Extract duration (optional, might not always be available)
        let duration = self.extract_duration(html).unwrap_or_else(|| "Unknown".to_string());
        
        // Extract uploader (optional)
        let uploader = self.extract_uploader(html);
        
        // Extract thumbnail URL
        let thumbnail_url = self.extract_thumbnail_url(html, video_id);
        
        let mut video_info = VideoInfo::new(title, duration, video_id.to_string());
        video_info.uploader = uploader;
        video_info.thumbnail_url = thumbnail_url;
        
        Ok(video_info)
    }
    
    /// Extract video title from HTML
    fn extract_title(&self, html: &str) -> Result<String> {
        static TITLE_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
        let patterns = TITLE_PATTERNS.get_or_init(|| {
            vec![
                // YouTube's current meta property
                Regex::new(r#"<meta property="og:title" content="([^"]+)""#).unwrap(),
                // Alternative title patterns
                Regex::new(r#""title":"([^"]+)""#).unwrap(),
                Regex::new(r#"<title>([^<]+) - YouTube</title>"#).unwrap(),
                // JSON-LD structured data
                Regex::new(r#""name":"([^"]+)""#).unwrap(),
            ]
        });
        
        for pattern in patterns {
            if let Some(captures) = pattern.captures(html) {
                if let Some(title_match) = captures.get(1) {
                    let title = title_match.as_str();
                    // Decode HTML entities and clean up
                    let cleaned_title = self.decode_html_entities(title);
                    if !cleaned_title.is_empty() && cleaned_title != "YouTube" {
                        debug!("Extracted title: {}", cleaned_title);
                        return Ok(cleaned_title);
                    }
                }
            }
        }
        
        Err(DownloaderError::ExtractionFailed("Could not extract video title".to_string()))
    }
    
    /// Extract video duration from HTML
    fn extract_duration(&self, html: &str) -> Option<String> {
        static DURATION_PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
        let patterns = DURATION_PATTERNS.get_or_init(|| {
            vec![
                // ISO 8601 duration format
                Regex::new(r#""duration":"PT(\d+M)?(\d+S)?""#).unwrap(),
                // Seconds format
                Regex::new(r#""lengthSeconds":"(\d+)""#).unwrap(),
            ]
        });
        
        for pattern in patterns {
            if let Some(captures) = pattern.captures(html) {
                if let Some(duration_match) = captures.get(1) {
                    let duration_str = duration_match.as_str();
                    if let Ok(seconds) = duration_str.parse::<u64>() {
                        return Some(self.format_duration(seconds));
                    }
                }
            }
        }
        
        None
    }
    
    /// Extract uploader/channel name from HTML
    fn extract_uploader(&self, html: &str) -> Option<String> {
        static UPLOADER_PATTERN: OnceLock<Regex> = OnceLock::new();
        let pattern = UPLOADER_PATTERN.get_or_init(|| {
            Regex::new(r#""ownerChannelName":"([^"]+)""#).unwrap()
        });
        
        if let Some(captures) = pattern.captures(html) {
            if let Some(uploader_match) = captures.get(1) {
                let uploader = self.decode_html_entities(uploader_match.as_str());
                return Some(uploader);
            }
        }
        
        None
    }
    
    /// Extract thumbnail URL
    fn extract_thumbnail_url(&self, _html: &str, video_id: &str) -> String {
        // Use YouTube's predictable thumbnail URL format
        format!("https://img.youtube.com/vi/{}/maxresdefault.jpg", video_id)
    }
    
    /// Extract available formats from YouTube page
    fn extract_formats(&self, html: &str) -> Result<Vec<Format>> {
        debug!("Extracting formats from page");
        
        // Look for the ytInitialPlayerResponse or similar JSON data
        static PLAYER_RESPONSE_PATTERN: OnceLock<Regex> = OnceLock::new();
        let pattern = PLAYER_RESPONSE_PATTERN.get_or_init(|| {
            Regex::new(r#"ytInitialPlayerResponse["\s]*=["\s]*(\{.+?\});?"#).unwrap()
        });
        
        if let Some(captures) = pattern.captures(html) {
            if let Some(json_match) = captures.get(1) {
                let json_str = json_match.as_str();
                if let Ok(json_data) = serde_json::from_str::<Value>(json_str) {
                    return self.parse_formats_from_json(&json_data);
                }
            }
        }
        
        // Fallback: try alternative extraction methods
        warn!("Could not find ytInitialPlayerResponse, trying alternative methods");
        self.extract_formats_fallback(html)
    }
    
    /// Parse formats from YouTube's JSON player response
    fn parse_formats_from_json(&self, json_data: &Value) -> Result<Vec<Format>> {
        let mut formats = Vec::new();
        
        // Navigate to streaming data
        if let Some(streaming_data) = json_data.get("streamingData") {
            // Extract adaptive formats (separate audio/video)
            if let Some(adaptive_formats) = streaming_data.get("adaptiveFormats") {
                if let Some(adaptive_array) = adaptive_formats.as_array() {
                    for format_obj in adaptive_array {
                        if let Some(format) = self.parse_single_format(format_obj, true) {
                            formats.push(format);
                        }
                    }
                }
            }
            
            // Extract regular formats (combined audio/video)
            if let Some(regular_formats) = streaming_data.get("formats") {
                if let Some(formats_array) = regular_formats.as_array() {
                    for format_obj in formats_array {
                        if let Some(format) = self.parse_single_format(format_obj, false) {
                            formats.push(format);
                        }
                    }
                }
            }
        }
        
        if formats.is_empty() {
            return Err(DownloaderError::NoFormatsFound);
        }
        
        debug!("Extracted {} formats", formats.len());
        Ok(formats)
    }
    
    /// Parse a single format object from JSON
    fn parse_single_format(&self, format_obj: &Value, is_adaptive: bool) -> Option<Format> {
        let url = format_obj.get("url")?.as_str()?.to_string();
        let mime_type = format_obj.get("mimeType")?.as_str()?;
        
        // Determine format type and extension based on mime type
        let (format_type, file_extension) = if mime_type.contains("audio") {
            (FormatType::Audio, "mp3".to_string()) // We'll convert to MP3
        } else if mime_type.contains("video") {
            (FormatType::Video, "mp4".to_string()) // We'll focus on MP4
        } else {
            return None; // Skip unsupported formats
        };
        
        // Only process MP4 video and audio formats for simplicity
        if !mime_type.contains("mp4") && !mime_type.contains("audio") {
            return None;
        }
        
        // Extract quality information
        let quality = if format_type == FormatType::Video {
            // For video, try to get height or quality label
            if let Some(height) = format_obj.get("height").and_then(|h| h.as_u64()) {
                format!("{}p", height)
            } else if let Some(quality_label) = format_obj.get("qualityLabel").and_then(|q| q.as_str()) {
                quality_label.to_string()
            } else {
                "Unknown".to_string()
            }
        } else {
            // For audio, try to get bitrate
            if let Some(bitrate) = format_obj.get("averageBitrate").and_then(|b| b.as_u64()) {
                format!("{}kbps", bitrate / 1000)
            } else {
                "Audio".to_string()
            }
        };
        
        let mut format = Format::new(quality, format_type, file_extension, url);
        
        // Extract additional metadata
        if let Some(file_size) = format_obj.get("contentLength").and_then(|s| s.as_str()) {
            if let Ok(size) = file_size.parse::<u64>() {
                format.file_size = Some(size);
            }
        }
        
        if let Some(bitrate) = format_obj.get("averageBitrate").and_then(|b| b.as_u64()) {
            format.bitrate = Some(bitrate as u32);
        }
        
        // Extract codec information
        if let Some(codecs) = format_obj.get("codecs").and_then(|c| c.as_str()) {
            format.codec = Some(codecs.to_string());
        }
        
        Some(format)
    }
    
    /// Fallback format extraction when main method fails
    fn extract_formats_fallback(&self, _html: &str) -> Result<Vec<Format>> {
        // This is a simplified fallback - in a real implementation,
        // you might want to try other extraction methods
        warn!("Fallback format extraction not fully implemented");
        Err(DownloaderError::NoFormatsFound)
    }
    
    /// Filter formats to only include MP4 video and MP3 audio
    fn filter_formats(&self, formats: Vec<Format>) -> Vec<Format> {
        let mut filtered: Vec<Format> = formats
            .into_iter()
            .filter(|format| {
                match format.format_type {
                    FormatType::Video => format.file_extension == "mp4",
                    FormatType::Audio => true, // We'll handle audio format conversion
                }
            })
            .collect();
        
        // Sort by quality (best first)
        filtered.sort_by(|a, b| {
            match (&a.format_type, &b.format_type) {
                (FormatType::Video, FormatType::Video) => {
                    // Sort video by resolution (higher first)
                    self.compare_video_quality(&a.quality, &b.quality)
                }
                (FormatType::Audio, FormatType::Audio) => {
                    // Sort audio by bitrate (higher first)
                    b.bitrate.unwrap_or(0).cmp(&a.bitrate.unwrap_or(0))
                }
                (FormatType::Video, FormatType::Audio) => std::cmp::Ordering::Less,
                (FormatType::Audio, FormatType::Video) => std::cmp::Ordering::Greater,
            }
        });
        
        debug!("Filtered to {} supported formats", filtered.len());
        filtered
    }
    
    /// Compare video quality strings (e.g., "1080p" vs "720p")
    fn compare_video_quality(&self, a: &str, b: &str) -> std::cmp::Ordering {
        let extract_number = |s: &str| -> u32 {
            s.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse()
                .unwrap_or(0)
        };
        
        let a_num = extract_number(a);
        let b_num = extract_number(b);
        
        b_num.cmp(&a_num) // Descending order (higher quality first)
    }
    
    /// Format duration from seconds to MM:SS or HH:MM:SS
    fn format_duration(&self, total_seconds: u64) -> String {
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        
        if hours > 0 {
            format!("{}:{:02}:{:02}", hours, minutes, seconds)
        } else {
            format!("{}:{:02}", minutes, seconds)
        }
    }
    
    /// Decode common HTML entities
    fn decode_html_entities(&self, text: &str) -> String {
        text.replace("&amp;", "&")
            .replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&#39;", "'")
            .replace("&apos;", "'")
    }
}