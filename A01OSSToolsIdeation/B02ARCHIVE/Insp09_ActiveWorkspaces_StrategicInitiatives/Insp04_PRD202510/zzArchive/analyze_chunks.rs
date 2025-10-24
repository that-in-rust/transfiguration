use std::fs;
use std::collections::HashMap;
use md5::{Digest, Md5};

fn md5_short(text: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())[..8].to_string()
}

fn main() {
    let dir = "W04-chunksAB/";
    let output_file = "/Users/amuldotexe/Projects/transfiguration/Insp09_ActiveWorkspaces_StrategicInitiatives/Insp04_PRD202510/chunk_themes.txt";
    
    let mut results = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        if let Ok(e) = entry {
            let path = e.path();
            if path.extension().unwrap_or_default() == "txt" {
                let content = fs::read_to_string(&path).unwrap();
                let filename = path.file_name().unwrap().to_string_lossy().to_string();
                let hash = md5_short(&content);
                // Simple 4-word theme (first 4 words)
                let words: Vec<&str> = content.split_whitespace().collect();
                let theme = words.iter().take(4).cloned().collect::<Vec<_>>().join(" ");
                results.push(format!("{}|{}|{}", path.to_string_lossy(), filename, theme));
            }
        }
    }
    
    fs::write(output_file, results.join("\n")).unwrap();
    println!("Saved {} entries to {}", results.len(), output_file);
}
