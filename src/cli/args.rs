//! Command-line argument definitions and parsing

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "downloader")]
#[command(about = "High-performance YouTube video downloader")]
#[command(version)]
pub struct Args {
    /// YouTube video URL to download
    #[arg(value_name = "URL")]
    pub url: String,
    
    /// Output directory (optional)
    #[arg(short, long)]
    pub output: Option<String>,
    
    /// Skip interactive selection and use best quality
    #[arg(short, long)]
    pub auto: bool,
    
    /// Force audio-only download
    #[arg(short = 'a', long)]
    pub audio_only: bool,
    
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

impl Args {
    // TODO: Add validation methods
    pub fn validate(&self) -> crate::Result<()> {
        todo!("Implement argument validation")
    }
}