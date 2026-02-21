use std::path::PathBuf;
use walkdir::WalkDir;

pub fn scan_music(dir: &str) -> Vec<PathBuf> {
    println!("🔍 Scanning directory: {}", dir);

    let files: Vec<PathBuf> = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            let path = e.path();
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.to_lowercase());

            matches!(ext.as_deref(), Some("mp3") | Some("ogg"))
        })
        .map(|e| e.path().to_path_buf())
        .collect();

    println!("✅ Found {} music files.", files.len());
    files
}
