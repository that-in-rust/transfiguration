use thiserror::Error;
use std::path::{Path, PathBuf};

#[derive(Error, Debug)]
pub enum ExtractionError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Path traversal detected: {path}")]
    PathTraversal { path: String },
    #[error("Archive format error: {0}")]
    Format(String),
    #[error("Path validation error: {0}")]
    Path(#[from] PathError),
    #[error("Recursion depth limit exceeded: {depth} (max: {limit})")]
    RecursionLimitExceeded { depth: usize, limit: usize },
}

#[derive(Error, Debug)]
pub enum PathError {
    #[error("Path contains '..'")]
    Traversal,
    #[error("Absolute path not allowed")]
    Absolute,
}

pub type Result<T> = std::result::Result<T, ExtractionError>;
pub type PathResult<T> = std::result::Result<T, PathError>;

pub fn extract_deb(input_path: &Path, output_dir: &Path) -> Result<()> {
    extract_deb_with_depth(input_path, output_dir, 10) // Default depth limit of 10
}

pub fn extract_deb_with_depth(input_path: &Path, output_dir: &Path, max_depth: usize) -> Result<()> {
    use std::fs::{self, File};
    use std::io::{BufReader, Read};
    
    // Check depth limit at the start
    if max_depth == 0 {
        return Err(ExtractionError::RecursionLimitExceeded { 
            depth: 0, 
            limit: max_depth 
        });
    }
    
    fs::create_dir_all(output_dir)?;
    let file = File::open(input_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    
    let mut archive = ar::Archive::new(&buffer[..]);
    
    while let Some(entry_result) = archive.next_entry() {
        let mut entry = entry_result.map_err(|e| ExtractionError::Format(format!("AR parse error: {}", e)))?;
        
        let filename = {
            let header = entry.header();
            std::str::from_utf8(header.identifier())
                .map_err(|e| ExtractionError::Format(format!("Invalid filename encoding: {}", e)))?
                .to_string()
        };
        
        let safe_path = validate_path(&filename, output_dir)?;
        let mut entry_data = Vec::new();
        entry.read_to_end(&mut entry_data)?;
        
        match filename.as_str() {
            "debian-binary" => {
                fs::write(&safe_path, &entry_data)?;
                println!("Extracted debian-binary to {:?}", safe_path);
            }
            name if name.starts_with("control.tar") => {
                println!("DEBUG: About to extract control archive: {}", name);
                extract_tar_archive_with_name_and_depth(&entry_data, output_dir, "control", name, max_depth, 0)?;
                println!("Extracted control archive: {}", name);
            }
            name if name.starts_with("data.tar") => {
                extract_tar_archive_with_name_and_depth(&entry_data, output_dir, "data", name, max_depth, 0)?;
                println!("Extracted data archive: {}", name);
            }
            _ => {
                fs::write(&safe_path, &entry_data)?;
                println!("Extracted file: {} to {:?}", filename, safe_path);
            }
        }
    }
    Ok(())
}

fn extract_tar_archive_with_name_and_depth(data: &[u8], base_output_dir: &Path, subdir: &str, filename: &str, max_depth: usize, current_depth: usize) -> Result<()> {
    if current_depth >= max_depth {
        return Err(ExtractionError::RecursionLimitExceeded { 
            depth: current_depth, 
            limit: max_depth 
        });
    }
    
    let output_dir = base_output_dir.join(subdir);
    std::fs::create_dir_all(&output_dir)?;
    
    let extraction_result = if filename.ends_with(".xz") {
        println!("Detected XZ compression for {}", filename);
        try_extract_xz_tar_with_depth(data, &output_dir, max_depth, current_depth + 1)
    } else if filename.ends_with(".gz") {
        println!("Detected Gzip compression for {}", filename);
        try_extract_gzipped_tar_with_depth(data, &output_dir, max_depth, current_depth + 1)
    } else {
        println!("No compression detected for {}, trying raw TAR", filename);
        try_extract_raw_tar_with_depth(data, &output_dir, max_depth, current_depth + 1)
    };
    
    match extraction_result {
        Ok(()) => {
            println!("Successfully extracted {} archive", subdir);
            Ok(())
        }
        Err(e) => {
            println!("Warning: Failed to extract {} archive: {}", subdir, e);
            // Still create the directory even if extraction fails
            std::fs::create_dir_all(&output_dir)?;
            Ok(())
        }
    }
}

fn try_extract_gzipped_tar_with_depth(data: &[u8], output_dir: &Path, max_depth: usize, current_depth: usize) -> Result<()> {
    use std::io::Cursor;
    use flate2::read::GzDecoder;
    
    println!("Attempting Gzip extraction, data size: {} bytes", data.len());
    
    let cursor = Cursor::new(data);
    let gz_decoder = GzDecoder::new(cursor);
    let mut archive = tar::Archive::new(gz_decoder);
    let entries = archive.entries().map_err(|e| {
        println!("Gzip TAR entries error: {}", e);
        ExtractionError::Format(format!("Gzipped TAR parse error: {}", e))
    })?;
    extract_tar_entries_with_depth(entries, output_dir, max_depth, current_depth)
}

fn try_extract_raw_tar_with_depth(data: &[u8], output_dir: &Path, max_depth: usize, current_depth: usize) -> Result<()> {
    use std::io::Cursor;
    
    let cursor = Cursor::new(data);
    let mut archive = tar::Archive::new(cursor);
    let entries = archive.entries().map_err(|e| ExtractionError::Format(format!("Raw TAR parse error: {}", e)))?;
    extract_tar_entries_with_depth(entries, output_dir, max_depth, current_depth)
}

fn try_extract_xz_tar_with_depth(data: &[u8], output_dir: &Path, max_depth: usize, current_depth: usize) -> Result<()> {
    use std::io::Cursor;
    use xz2::read::XzDecoder;
    
    println!("Attempting XZ extraction, data size: {} bytes", data.len());
    
    let cursor = Cursor::new(data);
    let xz_decoder = XzDecoder::new(cursor);
    let mut archive = tar::Archive::new(xz_decoder);
    let entries = archive.entries().map_err(|e| {
        println!("XZ TAR entries error: {}", e);
        ExtractionError::Format(format!("XZ TAR parse error: {}", e))
    })?;
    extract_tar_entries_with_depth(entries, output_dir, max_depth, current_depth)
}

fn extract_tar_entries_with_depth<R: std::io::Read>(entries: tar::Entries<R>, output_dir: &Path, max_depth: usize, current_depth: usize) -> Result<()> {
    if current_depth >= max_depth {
        println!("Warning: Recursion depth limit reached at depth {}, skipping further extraction", current_depth);
        return Ok(());
    }
    
    for entry_result in entries {
        let mut entry = entry_result.map_err(|e| ExtractionError::Format(format!("TAR entry error: {}", e)))?;
        
        let path_str = {
            let entry_path = entry.path().map_err(|e| ExtractionError::Format(format!("Invalid tar path: {}", e)))?;
            entry_path.to_string_lossy().to_string()
        };
        
        let safe_path = validate_path(&path_str, output_dir)?;
        
        if let Some(parent) = safe_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        if entry.header().entry_type().is_file() {
            entry.unpack(&safe_path)?;
            println!("Extracted tar file: {} to {:?}", path_str, safe_path);
        } else if entry.header().entry_type().is_dir() {
            std::fs::create_dir_all(&safe_path)?;
            println!("Created directory: {:?}", safe_path);
        }
    }
    Ok(())
}

pub
 fn validate_path(path: &str, base_dir: &Path) -> PathResult<PathBuf> {
    if path.is_empty() {
        return Ok(base_dir.to_path_buf());
    }
    
    if path.starts_with('/') {
        return Err(PathError::Absolute);
    }
    
    if path.len() >= 2
        && path.chars().nth(1) == Some(':') && path.chars().next().is_some_and(|c| c.is_ascii_alphabetic()) {
            return Err(PathError::Absolute);
        }
    
    if path.starts_with("\\\\") {
        return Err(PathError::Absolute);
    }
    
    if path.contains("..") {
        return Err(PathError::Traversal);
    }
    
    let path_buf = Path::new(path);
    let mut components = Vec::new();
    for component in path_buf.components() {
        match component {
            std::path::Component::Normal(name) => {
                components.push(name);
            }
            std::path::Component::CurDir => {
                continue;
            }
            std::path::Component::ParentDir => {
                return Err(PathError::Traversal);
            }
            std::path::Component::Prefix(_) | std::path::Component::RootDir => {
                return Err(PathError::Absolute);
            }
        }
    }
    
    let mut result = base_dir.to_path_buf();
    for component in components {
        result.push(component);
    }
    
    Ok(result)
}

#[cfg(test)]
mod recursion_tests;

pub mod cli {
    use clap::Parser;
    use std::path::PathBuf;

    #[derive(Parser, Debug, PartialEq)]
    #[command(about = "Extract .deb files safely", version)]
    pub struct Args {
        pub input: PathBuf,
        #[arg(short, long, default_value = "./extracted")]
        pub output: PathBuf,
        #[arg(short, long)]
        pub verbose: bool,
        #[arg(long, default_value = "10")]
        pub max_depth: usize,
    }

    impl Args {
        pub fn parse_from_args(args: Vec<&str>) -> Result<Self, clap::Error> {
            Self::try_parse_from(args)
        }
    }
}