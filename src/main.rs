mod broadcaster;
mod config;
mod pacer;
mod scanner;
mod server;
mod streamer;

use crate::broadcaster::Broadcaster;
use crate::config::Config;
use crate::streamer::Streamer;
use std::sync::Arc;

fn main() {
    let config = Config::load();

    // Server Mode Only - Commands removed as per user request
    println!("🚀 Starting {} (Standalone Mode)...", config.station.name);

    let files = scanner::scan_music(&config.music_dir);
    if files.is_empty() {
        eprintln!("❌ No music files found in {}", config.music_dir);
        std::process::exit(1);
    }

    // Initialize Broadcaster for real-time fan-out
    let broadcaster = Arc::new(Broadcaster::new(config.clone()));
    broadcaster.update_station_info(
        config.station.name.clone(),
        config.station.description.clone(),
        config.station.genre.clone(),
        config.station.logo_url.clone(),
    );

    // Start HTTP Streaming Server (Standalone Radio)
    let server_broadcaster = Arc::clone(&broadcaster);
    server::start(config.server_port, server_broadcaster);

    // Initialize and start Streamer
    let mut streamer = Streamer::new(files, broadcaster, config.music_dir.clone());

    println!("📻 Station is live at: {}", config.station.url);
    println!(
        "📡 Standalone server running on port {}",
        config.server_port
    );

    streamer.stream_loop();
}
