//! Network utilities and helpers

use crate::Result;
use crate::error::DownloaderError;
use reqwest::{Client, header::{HeaderMap, HeaderValue, RANGE, USER_AGENT, ACCEPT, ACCEPT_LANGUAGE}};
use std::time::Duration;
use log::{debug, warn};

pub struct NetworkUtils;

impl NetworkUtils {
    /// Create HTTP client with appropriate headers to avoid bot detection
    pub fn create_client() -> Result<Client> {
        let mut headers = HeaderMap::new();
        
        // Use a realistic User-Agent to avoid bot detection
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        );
        
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8")
        );
        
        headers.insert(
            ACCEPT_LANGUAGE,
            HeaderValue::from_static("en-US,en;q=0.5")
        );

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .redirect(reqwest::redirect::Policy::limited(3))
            .build()?;
            
        Ok(client)
    }
    
    /// Get file size from HTTP headers using HEAD request
    pub async fn get_file_size(client: &Client, url: &str) -> Result<Option<u64>> {
        debug!("Getting file size for: {}", url);
        
        match client.head(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    if let Some(content_length) = response.headers().get("content-length") {
                        if let Ok(length_str) = content_length.to_str() {
                            if let Ok(length) = length_str.parse::<u64>() {
                                debug!("File size: {} bytes", length);
                                return Ok(Some(length));
                            }
                        }
                    }
                }
                debug!("No content-length header found");
                Ok(None)
            }
            Err(e) => {
                warn!("Failed to get file size: {}", e);
                // Don't fail the entire operation if we can't get file size
                Ok(None)
            }
        }
    }
    
    /// Test network connectivity by making a simple request to YouTube
    pub async fn test_connectivity() -> Result<bool> {
        let client = Self::create_client()?;
        
        match client.head("https://www.youtube.com").send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }
    
    /// Check if server supports range requests (HTTP partial content)
    pub async fn supports_range_requests(client: &Client, url: &str) -> Result<bool> {
        debug!("Checking range request support for: {}", url);
        
        match client.head(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    // Check for Accept-Ranges header
                    if let Some(accept_ranges) = response.headers().get("accept-ranges") {
                        if let Ok(ranges_str) = accept_ranges.to_str() {
                            let supports_ranges = ranges_str.contains("bytes");
                            debug!("Range request support: {}", supports_ranges);
                            return Ok(supports_ranges);
                        }
                    }
                }
                debug!("No accept-ranges header found");
                Ok(false)
            }
            Err(e) => {
                warn!("Failed to check range support: {}", e);
                Ok(false)
            }
        }
    }
    
    /// Create headers for range requests (HTTP partial content)
    pub fn create_range_headers(start: u64, end: Option<u64>) -> HeaderMap {
        let mut headers = HeaderMap::new();
        
        let range_value = match end {
            Some(end_byte) => format!("bytes={}-{}", start, end_byte),
            None => format!("bytes={}-", start),
        };
        
        if let Ok(header_value) = HeaderValue::from_str(&range_value) {
            headers.insert(RANGE, header_value);
        }
        
        headers
    }
    
    /// Create a client with custom headers for specific requests
    pub fn create_client_with_headers(additional_headers: HeaderMap) -> Result<Client> {
        let mut headers = HeaderMap::new();
        
        // Default headers
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        );
        
        // Add additional headers
        headers.extend(additional_headers);

        let client = Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .build()?;
            
        Ok(client)
    }
    
    /// Retry a network operation with exponential backoff
    pub async fn retry_with_backoff<F, Fut, T>(
        mut operation: F,
        max_retries: u32,
    ) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut attempts = 0;
        let mut delay = Duration::from_millis(100);
        
        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempts += 1;
                    
                    if attempts >= max_retries || !e.is_recoverable() {
                        return Err(e);
                    }
                    
                    debug!("Retry attempt {} after error: {}", attempts, e);
                    tokio::time::sleep(delay).await;
                    
                    // Exponential backoff with jitter
                    delay = std::cmp::min(delay * 2, Duration::from_secs(30));
                }
            }
        }
    }
}