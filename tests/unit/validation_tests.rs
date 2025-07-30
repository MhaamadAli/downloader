//! URL validation unit tests

use youtube_downloader::utils::UrlValidator;

#[test]
fn test_valid_youtube_urls() {
    // TODO: Test various valid YouTube URL formats
    assert!(UrlValidator::is_valid_youtube_url("https://www.youtube.com/watch?v=dQw4w9WgXcQ"));
    assert!(UrlValidator::is_valid_youtube_url("https://youtu.be/dQw4w9WgXcQ"));
    // Add more test cases
}

#[test]
fn test_invalid_youtube_urls() {
    // TODO: Test invalid URL formats
    assert!(!UrlValidator::is_valid_youtube_url("https://example.com"));
    assert!(!UrlValidator::is_valid_youtube_url("not-a-url"));
    // Add more test cases
}

#[test]
fn test_video_id_extraction() {
    // TODO: Test video ID extraction from various URL formats
}

// TODO: Add more validation unit tests