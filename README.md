# Rust URL Shortener

A high-performance URL shortener built with Rust, featuring both CLI and web server modes with persistent storage.

![Rust](https://img.shields.io/badge/Rust-1.70+-black?logo=rust)
![Actix-Web](https://img.shields.io/badge/Actix_Web-4.0-blue)

## Features

- **Hybrid Operation**: Run as CLI tool or web server
- **Persistent Storage**: Automatically saves URLs to `data.json`
- **Real Redirects**: 301 permanent redirects for SEO
- **Thread-Safe**: Uses `Arc<Mutex>` for concurrent access
- **Tiny URLs**: 6-character NanoID codes

## Installation

1. Install Rust via [rustup](https://rustup.rs/)
2. Clone this repository:
   ```bash
   git clone https://github.com/Lindelani21/URL_Shortener.git
   cd URL_Shortener
3. Build the project
   ```bash
   cargo build --release
## Usage
- Web Server Mode
  ```bash
  cargo run -- --server
- CLI Mode
  ```bash
  # Shorten a URL
  cargo run -- shorten https://example.com
  # Expand a short code
  cargo run -- expand abc123
  # List all URLs
  cargo run -- list
## Technical Stack
- Rust 2021 Edition

- Actix-Web (HTTP server)

- Serde (JSON serialization)

- NanoID (URL code generation)

- Parking Lot (High-performance mutex)

## Project Structure
src/ <br>
├── main.rs      
├── lib.rs       
data.json  <br>
Cargo.toml
