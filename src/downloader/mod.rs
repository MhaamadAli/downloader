//! Multi-threaded download management module

pub mod manager;
pub mod chunk;
pub mod progress;

pub use manager::DownloadManager;
pub use chunk::ChunkDownloader;
pub use progress::ProgressTracker;