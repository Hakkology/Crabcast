use crate::config::Config;
use std::collections::VecDeque;
use std::sync::mpsc::{self, SyncSender};
use std::sync::{Arc, Mutex, RwLock};

pub type AudioChunk = Arc<Vec<u8>>;

const CLIENT_BUFFER_SIZE: usize = 512;
const PRE_ROLL_SIZE: usize = 160;

#[derive(Clone, Default)]
pub struct Metadata {
    pub station_name: String,
    pub station_description: String,
    pub station_genre: String,
    pub station_logo: String,
    pub artist: String,
    pub title: String,
}

pub struct Broadcaster {
    clients: Mutex<Vec<SyncSender<AudioChunk>>>,
    metadata: RwLock<Metadata>,
    history: Mutex<VecDeque<AudioChunk>>,
    config: Config,
}

impl Broadcaster {
    pub fn new(config: Config) -> Self {
        Broadcaster {
            clients: Mutex::new(Vec::new()),
            metadata: RwLock::new(Metadata::default()),
            history: Mutex::new(VecDeque::with_capacity(PRE_ROLL_SIZE)),
            config,
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn set_metadata(&self, artist: String, title: String) {
        let mut meta = self.metadata.write().unwrap_or_else(|e| e.into_inner());
        meta.artist = artist;
        meta.title = title;
    }

    pub fn update_station_info(&self, name: String, desc: String, genre: String, logo: String) {
        let mut meta = self.metadata.write().unwrap_or_else(|e| e.into_inner());
        meta.station_name = name;
        meta.station_description = desc;
        meta.station_genre = genre;
        meta.station_logo = logo;
    }

    pub fn get_metadata(&self) -> Metadata {
        self.metadata
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .clone()
    }

    pub fn subscribe(&self) -> mpsc::Receiver<AudioChunk> {
        let (tx, rx) = mpsc::sync_channel(CLIENT_BUFFER_SIZE);

        {
            let history = self.history.lock().unwrap_or_else(|e| e.into_inner());
            for chunk in history.iter() {
                let _ = tx.try_send(Arc::clone(chunk));
            }
        }

        let mut clients = self.clients.lock().unwrap_or_else(|e| e.into_inner());
        clients.push(tx);
        rx
    }

    pub fn broadcast(&self, chunk: AudioChunk) {
        {
            let mut history = self.history.lock().unwrap_or_else(|e| e.into_inner());
            history.push_back(Arc::clone(&chunk));
            if history.len() > PRE_ROLL_SIZE {
                history.pop_front();
            }
        }

        let mut clients = self.clients.lock().unwrap_or_else(|e| e.into_inner());
        clients.retain(|client| client.try_send(Arc::clone(&chunk)).is_ok());
    }

    pub fn client_count(&self) -> usize {
        self.clients.lock().unwrap_or_else(|e| e.into_inner()).len()
    }
}
