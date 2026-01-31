use anyhow::Result;
use humansize::{format_size, BINARY};
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Format bytes to human readable size
pub fn format_bytes(bytes: u64) -> String {
    format_size(bytes, BINARY)
}

/// Parse human readable size to bytes
pub fn parse_size(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim().to_uppercase();
    
    let (num_str, multiplier) = if size_str.ends_with("GB") {
        (&size_str[..size_str.len() - 2], 1024 * 1024 * 1024)
    } else if size_str.ends_with("MB") {
        (&size_str[..size_str.len() - 2], 1024 * 1024)
    } else if size_str.ends_with("KB") {
        (&size_str[..size_str.len() - 2], 1024)
    } else if size_str.ends_with("B") {
        (&size_str[..size_str.len() - 1], 1)
    } else {
        (size_str.as_str(), 1)
    };

    let num: u64 = num_str.trim().parse()?;
    Ok(num * multiplier)
}

/// Parse time duration string to seconds
pub fn parse_duration(duration_str: &str) -> Result<u64> {
    let duration_str = duration_str.trim().to_lowercase();
    
    let (num_str, multiplier) = if duration_str.ends_with('d') {
        (&duration_str[..duration_str.len() - 1], 24 * 60 * 60)
    } else if duration_str.ends_with('h') {
        (&duration_str[..duration_str.len() - 1], 60 * 60)
    } else if duration_str.ends_with('m') {
        (&duration_str[..duration_str.len() - 1], 60)
    } else if duration_str.ends_with('s') {
        (&duration_str[..duration_str.len() - 1], 1)
    } else {
        (duration_str.as_str(), 1)
    };

    let num: u64 = num_str.trim().parse()?;
    Ok(num * multiplier)
}

/// Calculate SHA256 hash of a file
pub fn hash_file_sha256(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(1024 * 1024, file); // 1MB buffer
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hex::encode(hasher.finalize()))
}

/// Calculate SHA512 hash of a file
pub fn hash_file_sha512(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(1024 * 1024, file);
    let mut hasher = Sha512::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hex::encode(hasher.finalize()))
}

/// Calculate MD5 hash of a file (for compatibility, not security)
pub fn hash_file_md5(path: &Path) -> Result<String> {
    use md5::Context;
    
    let file = File::open(path)?;
    let mut reader = BufReader::with_capacity(1024 * 1024, file);
    let mut context = Context::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        context.consume(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", context.compute()))
}

/// Check if a file matches the given extensions filter
pub fn matches_extensions(path: &Path, extensions: &Option<String>) -> bool {
    match extensions {
        None => true,
        Some(exts) => {
            let ext_list: Vec<&str> = exts.split(',').map(|s| s.trim()).collect();
            if let Some(file_ext) = path.extension() {
                let file_ext_str = file_ext.to_string_lossy().to_lowercase();
                ext_list.iter().any(|e| e.to_lowercase() == file_ext_str)
            } else {
                false
            }
        }
    }
}

/// Check if path should be skipped (hidden files, common ignore patterns)
pub fn should_skip(path: &Path, include_hidden: bool) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    
    // Skip hidden files if not requested
    if !include_hidden && name.starts_with('.') {
        return true;
    }
    
    // Skip common non-useful directories
    let skip_dirs = [
        "node_modules",
        ".git",
        ".svn",
        ".hg",
        "__pycache__",
        ".cache",
        "target",
        ".idea",
        ".vscode",
        "vendor",
        "dist",
        "build",
    ];
    
    if path.is_dir() && skip_dirs.contains(&name) {
        return true;
    }
    
    false
}

/// Get file extension as lowercase string
pub fn get_extension(path: &Path) -> String {
    path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_else(|| "(no ext)".to_string())
}

