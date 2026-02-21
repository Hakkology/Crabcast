use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub station: StationConfig,
    pub music_dir: String,
    pub server_port: u16,
}

#[derive(Debug, Clone)]
pub struct StationConfig {
    pub name: String,
    pub description: String,
    pub genre: String,
    pub url: String,
    pub logo_url: String,
    pub logo_path: String,
    pub icon_path: String,
}

impl Config {
    pub fn load() -> Self {
        dotenv::dotenv().ok();

        let station_url =
            env::var("STATION_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

        let station = StationConfig {
            name: env::var("STATION_NAME").unwrap_or_else(|_| "My Radio Station".to_string()),
            description: env::var("STATION_DESCRIPTION")
                .unwrap_or_else(|_| "A high-performance audio stream".to_string()),
            genre: env::var("STATION_GENRE").unwrap_or_else(|_| "Miscellaneous".to_string()),
            url: station_url.clone(),
            logo_url: env::var("STATION_LOGO_URL")
                .unwrap_or_else(|_| format!("{}/logo.png", station_url)),
            logo_path: env::var("LOGO_PATH").unwrap_or_default(),
            icon_path: env::var("ICON_PATH").unwrap_or_default(),
        };

        Config {
            station,
            music_dir: env::var("MUSIC_DIR").unwrap_or_default(),
            server_port: env::var("RADIO_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .trim()
                .parse::<u16>()
                .unwrap_or(3000),
        }
    }
}
