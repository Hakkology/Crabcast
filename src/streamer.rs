use crate::broadcaster::Broadcaster;
use crate::pacer::ByteRateLimiter;
use crate::scanner;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub struct Streamer {
    files: Vec<PathBuf>,
    broadcaster: Arc<Broadcaster>,
    music_dir: String,
    pacer: ByteRateLimiter,
}

impl Streamer {
    pub fn new(files: Vec<PathBuf>, broadcaster: Arc<Broadcaster>, music_dir: String) -> Self {
        Streamer {
            files,
            broadcaster,
            music_dir,
            pacer: ByteRateLimiter::new(128),
        }
    }

    pub fn stream_loop(&mut self) {
        let mut current_index = 0;
        let mut rng = thread_rng();
        self.files.shuffle(&mut rng);
        self.pacer.reset();

        loop {
            if self.files.is_empty() {
                println!("⚠️ No files to play. Rescanning...");
                self.files = scanner::scan_music(&self.music_dir);
                if self.files.is_empty() {
                    thread::sleep(Duration::from_secs(5));
                    continue;
                }
                self.files.shuffle(&mut rng);
            }

            if current_index >= self.files.len() {
                current_index = 0;
                self.files.shuffle(&mut rng);
            }

            let path = self.files[current_index].clone();
            let (artist, title) = Self::parse_metadata(&path);

            println!(
                "🎵 Playing [{} / {}]: {} - {}",
                current_index + 1,
                self.files.len(),
                artist,
                title
            );
            self.broadcaster.set_metadata(artist, title);

            if let Err(e) = self.stream_file(&path) {
                eprintln!("⚠️ Error streaming {:?}: {}", path, e);
                thread::sleep(Duration::from_secs(1));
            }

            current_index += 1;
        }
    }

    fn stream_file(&mut self, path: &PathBuf) -> io::Result<()> {
        let file = File::open(path)?;
        let mut reader = std::io::BufReader::new(file);
        let mut buffer = [0u8; 1024];

        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }

            let chunk = Arc::new(buffer[..bytes_read].to_vec());
            self.broadcaster.broadcast(chunk);
            self.pacer.pace(bytes_read);
        }
        Ok(())
    }

    fn parse_metadata(path: &PathBuf) -> (String, String) {
        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        let stem = std::path::Path::new(&*filename)
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy();

        if let Some((artist, title)) = stem.split_once(" - ") {
            (artist.trim().to_string(), title.trim().to_string())
        } else {
            ("Unknown Artist".to_string(), stem.trim().to_string())
        }
    }
}
