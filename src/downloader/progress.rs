//! Download progress tracking and reporting

use crate::models::DownloadProgress;
use std::time::{Duration, Instant};
use std::collections::VecDeque;

pub struct ProgressTracker {
    start_time: Instant,
    last_update: Instant,
    last_downloaded: u64,
    speed_samples: VecDeque<(Instant, u64)>, // (timestamp, bytes_downloaded)
    max_samples: usize,
}

impl ProgressTracker {
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            start_time: now,
            last_update: now,
            last_downloaded: 0,
            speed_samples: VecDeque::new(),
            max_samples: 10, // Keep last 10 samples for smooth speed calculation
        }
    }
    
    /// Update progress and calculate speed/ETA
    pub fn update(&mut self, downloaded: u64, total: u64) -> DownloadProgress {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_update);
        
        // Add new sample for speed calculation
        self.speed_samples.push_back((now, downloaded));
        
        // Remove old samples to keep only recent ones
        while self.speed_samples.len() > self.max_samples {
            self.speed_samples.pop_front();
        }
        
        // Calculate current download speed
        let download_speed = self.calculate_speed();
        
        // Calculate ETA
        let eta_seconds = if download_speed > 0.0 && total > downloaded {
            ((total - downloaded) as f64 / download_speed) as u64
        } else {
            0
        };
        
        let is_complete = downloaded >= total && total > 0;
        
        // Update tracking state
        self.last_update = now;
        self.last_downloaded = downloaded;
        
        DownloadProgress {
            total_size: total,
            downloaded_size: downloaded,
            download_speed,
            eta_seconds,
            is_complete,
        }
    }
    
    /// Calculate download speed based on recent samples
    fn calculate_speed(&self) -> f64 {
        if self.speed_samples.len() < 2 {
            return 0.0;
        }
        
        // Use samples from the last few seconds for more accurate speed
        let now = Instant::now();
        let cutoff_time = now - Duration::from_secs(3);
        
        let recent_samples: Vec<_> = self.speed_samples
            .iter()
            .filter(|(timestamp, _)| *timestamp >= cutoff_time)
            .cloned()
            .collect();
        
        if recent_samples.len() < 2 {
            // Fall back to all samples if we don't have enough recent ones
            return self.calculate_speed_from_samples(&self.speed_samples.iter().cloned().collect());
        }
        
        self.calculate_speed_from_samples(&recent_samples)
    }
    
    /// Calculate speed from a set of samples
    fn calculate_speed_from_samples(&self, samples: &[(Instant, u64)]) -> f64 {
        if samples.len() < 2 {
            return 0.0;
        }
        
        let first = &samples[0];
        let last = &samples[samples.len() - 1];
        
        let time_diff = last.0.duration_since(first.0);
        let bytes_diff = last.1.saturating_sub(first.1);
        
        if time_diff.as_secs_f64() > 0.0 {
            bytes_diff as f64 / time_diff.as_secs_f64()
        } else {
            0.0
        }
    }
    
    /// Get overall average speed since start
    pub fn average_speed(&self) -> f64 {
        let elapsed = self.last_update.duration_since(self.start_time);
        if elapsed.as_secs_f64() > 0.0 && self.last_downloaded > 0 {
            self.last_downloaded as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        }
    }
    
    /// Get elapsed time since start
    pub fn elapsed_time(&self) -> Duration {
        self.last_update.duration_since(self.start_time)
    }
    
    /// Reset the tracker (useful for resume scenarios)
    pub fn reset(&mut self, initial_downloaded: u64) {
        let now = Instant::now();
        self.start_time = now;
        self.last_update = now;
        self.last_downloaded = initial_downloaded;
        self.speed_samples.clear();
        
        // Add initial sample
        self.speed_samples.push_back((now, initial_downloaded));
    }
}