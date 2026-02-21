use std::thread;
use std::time::{Duration, Instant};

pub struct ByteRateLimiter {
    bitrate_kbps: u32,
    total_bytes_sent: u64,
    start_time: Option<Instant>,
}

impl ByteRateLimiter {
    pub fn new(bitrate_kbps: u32) -> Self {
        ByteRateLimiter {
            bitrate_kbps,
            total_bytes_sent: 0,
            start_time: None,
        }
    }

    pub fn reset(&mut self) {
        self.total_bytes_sent = 0;
        self.start_time = Some(Instant::now());
    }

    pub fn pace(&mut self, bytes_sent: usize) {
        if self.start_time.is_none() {
            self.reset();
        }

        self.total_bytes_sent += bytes_sent as u64;

        let start = self.start_time.expect("Start time should be set");

        // Calculate how much time SHOULD have elapsed for this many bytes
        // (total_bytes * 8 bits/byte) / (bitrate * 1000 bits/sec) = target_seconds
        let target_seconds =
            (self.total_bytes_sent as f64 * 8.0) / (self.bitrate_kbps as f64 * 1000.0);
        let target_duration = Duration::from_secs_f64(target_seconds);

        let elapsed = start.elapsed();
        if target_duration > elapsed {
            thread::sleep(target_duration - elapsed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pacer_pacing() {
        let mut pacer = ByteRateLimiter::new(128); // 128 kbps = 16 KB/s
        let start = Instant::now();
        pacer.reset();

        // Send 32 KB (should take exactly 2 seconds)
        pacer.pace(16384);
        pacer.pace(16384);

        let elapsed = start.elapsed();
        // Allow a small margin for system scheduler fluctuations
        assert!(elapsed >= Duration::from_millis(1950));
    }
}
