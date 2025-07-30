# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a high-performance YouTube downloader CLI application built in Rust. The application features multi-threaded downloading with resume capability, interactive format selection, and clean progress tracking.

## Common Commands

### Build and Run
- `cargo build` - Compile the project
- `cargo run -- <youtube-url>` - Run with a YouTube URL
- `cargo run -- --help` - Show help information

### Testing
- `cargo test` - Run all tests
- `cargo test --lib` - Run library unit tests only
- `cargo test --bin` - Run binary integration tests only
- `cargo test <test_name>` - Run specific test

### Development
- `cargo check` - Fast compilation check without producing executable
- `cargo clippy` - Run linter for code quality
- `cargo fmt` - Format code according to Rust standards

## Architecture

The application follows a modular architecture with clear separation of concerns:

### Core Modules

**CLI Module** (`src/cli/`)
- `args.rs` - Command-line argument parsing using Clap
- `interface.rs` - Interactive user interface components

**Extractor Module** (`src/extractor/`)
- `youtube.rs` - YouTube-specific video information extraction
- `format.rs` - Format parsing and processing utilities

**Downloader Module** (`src/downloader/`)
- `manager.rs` - Main download coordination and multi-threading
- `chunk.rs` - Individual chunk downloading with HTTP range requests
- `progress.rs` - Download progress tracking and ETA calculation

**Models Module** (`src/models/`)
- `video.rs` - VideoInfo struct with metadata
- `format.rs` - Format and FormatType definitions
- `download.rs` - DownloadTask and DownloadProgress structs

**UI Module** (`src/ui/`)
- `selection.rs` - Format and quality selection interface
- `progress_bar.rs` - Progress bar implementation using indicatif

**File System Module** (`src/file_system/`)
- `organizer.rs` - File organization and directory management
- `resume.rs` - Download resume capability management

**Error Module** (`src/error/`)
- `types.rs` - Custom error types using thiserror, including DownloaderError enum

**Utils Module** (`src/utils/`)
- `validation.rs` - URL validation and video ID extraction
- `networking.rs` - HTTP client utilities and connectivity tests

**Config Module** (`src/config/`)
- `settings.rs` - Application settings with serde serialization

### Key Dependencies
- **tokio** - Async runtime for multi-threaded operations
- **reqwest** - HTTP client for downloads and API calls
- **clap** - Command-line argument parsing
- **indicatif** - Progress bars and terminal UI
- **serde** - JSON serialization/deserialization
- **thiserror** - Structured error handling

### Error Handling Pattern
The codebase uses a centralized error handling approach with `DownloaderError` enum and `Result<T>` type alias. Errors include network failures, file system issues, and user cancellation scenarios.

### Testing Strategy
- **Unit tests** in `tests/unit/` for individual components like validation and format handling
- **Integration tests** in `tests/integration/` for CLI workflows, download processes, and extractor functionality
- Tests use `tempfile` for temporary directories and `assert_cmd` for CLI testing

### Multi-threading Architecture
The downloader supports chunked downloading where large files are split into ranges and downloaded concurrently. The `DownloadManager` coordinates multiple `ChunkDownloader` instances and combines results.

### Resume Capability
Partial downloads are detected and resumed using HTTP range requests. The `ResumeManager` validates partial file integrity before continuing downloads.

## Development Notes

### Current Implementation Status
Most modules contain TODO items and placeholder implementations. The project structure is established but core functionality needs implementation.

### Entry Point
Main application flow starts in `src/main.rs` with the `run_application` function that orchestrates URL validation, video extraction, format selection, and download execution.

### Configuration
Settings are managed through the `Settings` struct with defaults for concurrent downloads (4), chunk size (1MB), and large file threshold (100MB).