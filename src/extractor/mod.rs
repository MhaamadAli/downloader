//! Video information extraction module
//! Handles YouTube metadata and format extraction

pub mod youtube;
pub mod format;

pub use youtube::YouTubeExtractor;
pub use format::FormatExtractor;