use crate::broadcaster::{AudioChunk, Broadcaster};
use std::io::Read;
use std::sync::Arc;
use std::thread;
use tiny_http::{Header, Response, Server};

const MAX_LISTENERS: usize = 120;

struct ReceiverReader {
    rx: std::sync::mpsc::Receiver<AudioChunk>,
    current_chunk: Option<AudioChunk>,
    pos: usize,
}

impl Read for ReceiverReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.current_chunk.is_none() {
            match self.rx.recv() {
                Ok(chunk) => {
                    self.current_chunk = Some(chunk);
                    self.pos = 0;
                }
                Err(_) => return Ok(0),
            }
        }

        if let Some(chunk) = &self.current_chunk {
            let available = chunk.len() - self.pos;
            let n = std::cmp::min(available, buf.len());
            buf[..n].copy_from_slice(&chunk[self.pos..self.pos + n]);
            self.pos += n;

            if self.pos >= chunk.len() {
                self.current_chunk = None;
            }
            Ok(n)
        } else {
            Ok(0)
        }
    }
}

pub fn start(port: u16, broadcaster: Arc<Broadcaster>) {
    let addr = format!("0.0.0.0:{}", port);
    let server = Server::http(&addr).expect("Failed to start HTTP server");
    println!("📡 Station listening on {}", addr);

    thread::spawn(move || {
        for request in server.incoming_requests() {
            let broadcaster = Arc::clone(&broadcaster);

            match request.url() {
                "/stream" => {
                    if broadcaster.client_count() >= MAX_LISTENERS {
                        let _ = request
                            .respond(Response::from_string("Server full").with_status_code(503));
                        continue;
                    }

                    let rx = broadcaster.subscribe();
                    let reader = ReceiverReader {
                        rx,
                        current_chunk: None,
                        pos: 0,
                    };

                    let response = Response::new(
                        200.into(),
                        vec![
                            Header::from_bytes(&b"Content-Type"[..], &b"audio/mpeg"[..]).unwrap(),
                            Header::from_bytes(&b"Cache-Control"[..], &b"no-cache, private"[..])
                                .unwrap(),
                            Header::from_bytes(&b"Connection"[..], &b"keep-alive"[..]).unwrap(),
                            Header::from_bytes(&b"X-Content-Type-Options"[..], &b"nosniff"[..])
                                .unwrap(),
                        ],
                        reader,
                        None,
                        None,
                    );

                    thread::spawn(move || {
                        let _ = request.respond(response);
                    });
                }
                "/metadata" => {
                    let meta = broadcaster.get_metadata();
                    let json = format!(
                        "{{\"station\": \"{}\", \"description\": \"{}\", \"genre\": \"{}\", \"logo\": \"{}\", \"artist\": \"{}\", \"title\": \"{}\"}}",
                        meta.station_name.replace('"', "\\\""),
                        meta.station_description.replace('"', "\\\""),
                        meta.station_genre.replace('"', "\\\""),
                        meta.station_logo.replace('"', "\\\""),
                        meta.artist.replace('"', "\\\""),
                        meta.title.replace('"', "\\\"")
                    );
                    let response = Response::from_string(json)
                        .with_header(
                            Header::from_bytes(&b"Content-Type"[..], &b"application/json"[..])
                                .unwrap(),
                        )
                        .with_header(
                            Header::from_bytes(&b"Access-Control-Allow-Origin"[..], &b"*"[..])
                                .unwrap(),
                        );
                    let _ = request.respond(response);
                }
                "/logo.png" => {
                    let path = &broadcaster.get_config().station.logo_path;
                    if !path.is_empty() {
                        serve_file(request, path, "image/png");
                    } else {
                        let _ = request
                            .respond(Response::from_string("Not configured").with_status_code(404));
                    }
                }
                "/icon.png" => {
                    let path = &broadcaster.get_config().station.icon_path;
                    if !path.is_empty() {
                        serve_file(request, path, "image/png");
                    } else {
                        let _ = request
                            .respond(Response::from_string("Not configured").with_status_code(404));
                    }
                }
                _ => {
                    let _ = request
                        .respond(Response::from_string("Radio Station").with_status_code(404));
                }
            }
        }
    });
}

fn serve_file(request: tiny_http::Request, path: &str, content_type: &str) {
    if let Ok(file) = std::fs::File::open(path) {
        let response = Response::from_file(file).with_header(
            Header::from_bytes(&b"Content-Type"[..], content_type.as_bytes()).unwrap(),
        );
        let _ = request.respond(response);
    } else {
        let _ = request.respond(Response::from_string("Not found").with_status_code(404));
    }
}
