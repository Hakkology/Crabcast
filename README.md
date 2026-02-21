# RadioBroadcaster 📻

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Powered by Rust 🦀**

A high-performance, lightweight audio broadcasting server built in Rust. Designed for real-time audio distribution with minimal latency and predictable resource usage.


## 🚀 Features

- **Fan-out Architecture**: Efficiently distribute audio to thousands of concurrent listeners.
- **Microsecond Precision**: Pacing logic ensures streams never drift or jitter.
- **Gapless Playback**: Seamlessly transitions between tracks without audio pops or silence.
- **JSON Metadata**: Integrated API for real-time track and station info.
- **Docker Ready**: Deploy anywhere with a single command.

## ⚡ Quick Start

### 1. Development Environment
Ensure you have the latest stable [Rust](https://rustup.rs/) installed.

### 2. Installation
```bash
git clone https://github.com/yourusername/RadioBroadcaster.git
cd RadioBroadcaster
```

### 3. Configuration
```bash
cp .env.example .env
# Open .env and set your MUSIC_DIR path
```

### 4. Run
```bash
cargo run --release
```

## 🛠 Configuration

The application is configured entirely via environment variables.

| Variable | Description | Default |
|----------|-------------|---------|
| `MUSIC_DIR` | Path to your audio files directory | `/music` |
| `STATION_NAME` | Name of your radio station | `My Radio Station` |
| `STATION_DESCRIPTION` | Short description of the station | `A high-performance audio stream` |
| `STATION_GENRE` | Music genre (comma separated) | `Various` |
| `STATION_URL` | Base URL of the station | `http://localhost:3000` |
| `STATION_LOGO_URL` | Public URL for the station logo | `STATION_URL/logo.png` |
| `LOGO_PATH` | Local file path for the logo image | (empty) |
| `ICON_PATH` | Local file path for the icon image | (empty) |
| `RADIO_PORT` | Port for the HTTP server | `3000` |


## 📡 API Endpoints

- **`GET /stream`**: The main high-quality audio stream.
- **`GET /metadata`**: Current track title, station name, and listener count.
- **`GET /logo.png`**: Serves the station branding.

## 🏗 Architecture

RadioBroadcaster uses a "Fan-out" producer-consumer model:
1. **Producer**: Reads audio from disk, decodes/paces it to match real-time playback speed.
2. **Buffer**: A shared lock-free ring buffer holds the most recent audio chunks.
3. **Consumers**: HTTP clients subscribe to the buffer and receive audio chunks simultaneously.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
