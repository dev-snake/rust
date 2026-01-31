use anyhow::Result;
use chrono::{DateTime, Local};
use colored::*;
use glob::Pattern;
use std::cmp::Ordering;
use walkdir::WalkDir;

use crate::ui::{self, chars};
use crate::utils::{format_bytes, get_extension};

struct FileInfo {
    path: String,
    name: String,
    size: u64,
    modified: DateTime<Local>,
    extension: String,
    is_dir: bool,
}

pub fn run(
    path: &str,
    sort: &str,
    reverse: bool,
    recursive: bool,
    pattern: Option<String>,
    long: bool,
) -> Result<()> {
    let glob_pattern = pattern.as_ref().map(|p| Pattern::new(p).ok()).flatten();

    let walker = if recursive {
        WalkDir::new(path).follow_links(false)
    } else {
        WalkDir::new(path).max_depth(1).follow_links(false)
    };

    let mut files: Vec<FileInfo> = Vec::new();

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path();

        if entry_path.to_string_lossy() == path {
            continue;
        }

        let name = entry_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();

        if let Some(ref pat) = glob_pattern {
            if !pat.matches(&name) {
                continue;
            }
        }

        let metadata = entry_path.metadata().ok();
        let is_dir = entry_path.is_dir();

        let size = metadata.as_ref().map(|m| m.len()).unwrap_or(0);
        let modified = metadata
            .as_ref()
            .and_then(|m| m.modified().ok())
            .map(|t| DateTime::<Local>::from(t))
            .unwrap_or_else(Local::now);

        files.push(FileInfo {
            path: entry_path.display().to_string(),
            name,
            size,
            modified,
            extension: if is_dir {
                String::new()
            } else {
                get_extension(entry_path)
            },
            is_dir,
        });
    }

    // Sort
    files.sort_by(|a, b| {
        let ord = match sort {
            "size" => b.size.cmp(&a.size),
            "date" => b.modified.cmp(&a.modified),
            "ext" => a.extension.cmp(&b.extension),
            _ => {
                match (a.is_dir, b.is_dir) {
                    (true, false) => Ordering::Less,
                    (false, true) => Ordering::Greater,
                    _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                }
            }
        };

        if reverse {
            ord.reverse()
        } else {
            ord
        }
    });

    // Print
    if long {
        println!(
            "  {:>12}  {:>19}  {}",
            "SIZE".cyan().bold(),
            "MODIFIED".cyan().bold(),
            "NAME".cyan().bold()
        );
        ui::print_line(60);

        for file in &files {
            let size_str = if file.is_dir {
                format!("{:>12}", "<DIR>".blue())
            } else {
                format!("{:>12}", format_bytes(file.size))
            };

            let name_str = if file.is_dir {
                format!("{}/", file.name).blue().bold().to_string()
            } else {
                file.name.clone()
            };

            println!(
                "  {}  {}  {}",
                size_str,
                file.modified.format("%Y-%m-%d %H:%M:%S").to_string().dimmed(),
                name_str
            );
        }
    } else {
        let term_width = 80;
        let max_name_len = files.iter().map(|f| f.name.len()).max().unwrap_or(20);
        let col_width = (max_name_len + 4).min(30);
        let cols = (term_width / col_width).max(1);

        for chunk in files.chunks(cols) {
            print!("  ");
            for file in chunk {
                let name = if file.is_dir {
                    format!("{}/", file.name).blue().bold().to_string()
                } else {
                    file.name.clone()
                };
                print!("{:width$}", name, width = col_width);
            }
            println!();
        }
    }

    println!();
    println!(
        "{} {} items",
        chars::ARROW.dimmed(),
        files.len().to_string().green().bold()
    );

    Ok(())
}
