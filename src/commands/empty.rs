use anyhow::Result;
use colored::*;
use std::fs;
use walkdir::WalkDir;

use crate::ui::{self, chars};

pub fn run(path: &str, dirs_only: bool, files_only: bool, delete: bool) -> Result<()> {
    ui::print_start("Finding empty items", path);
    println!();

    let find_dirs = dirs_only || (!dirs_only && !files_only);
    let find_files = files_only || (!dirs_only && !files_only);

    let mut empty_dirs = Vec::new();
    let mut empty_files = Vec::new();

    // Find empty files first
    if find_files {
        for entry in WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry_path = entry.path();
            if entry_path.is_file() {
                if let Ok(metadata) = entry_path.metadata() {
                    if metadata.len() == 0 {
                        empty_files.push(entry_path.to_path_buf());
                    }
                }
            }
        }
    }

    // Find empty directories
    if find_dirs {
        let mut all_dirs: Vec<_> = WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_dir())
            .map(|e| e.path().to_path_buf())
            .collect();

        all_dirs.sort_by(|a, b| {
            let depth_a = a.components().count();
            let depth_b = b.components().count();
            depth_b.cmp(&depth_a)
        });

        for dir in all_dirs {
            if is_dir_empty(&dir) {
                empty_dirs.push(dir);
            }
        }
    }

    // Print results
    if empty_files.is_empty() && empty_dirs.is_empty() {
        ui::print_success("No empty items found");
        return Ok(());
    }

    if !empty_files.is_empty() {
        ui::print_section(&format!("Empty Files ({})", empty_files.len()));
        for file in &empty_files {
            println!("  {} {}", chars::DOT.yellow(), file.display());
        }
        println!();
    }

    if !empty_dirs.is_empty() {
        ui::print_section(&format!("Empty Directories ({})", empty_dirs.len()));
        for dir in &empty_dirs {
            println!("  {} {}", chars::DOT.yellow(), dir.display());
        }
        println!();
    }

    // Delete if requested
    if delete {
        ui::print_warning("Deleting empty items...");
        println!();

        let mut deleted_files = 0;
        let mut deleted_dirs = 0;
        let mut errors = 0;

        // Delete files first
        for file in &empty_files {
            match fs::remove_file(file) {
                Ok(_) => {
                    deleted_files += 1;
                    println!(
                        "  {} {}",
                        chars::CROSS_MARK.red(),
                        file.display().to_string().dimmed()
                    );
                }
                Err(_) => errors += 1,
            }
        }

        // Delete directories (already sorted deepest first)
        for dir in &empty_dirs {
            match fs::remove_dir(dir) {
                Ok(_) => {
                    deleted_dirs += 1;
                    println!(
                        "  {} {}",
                        chars::CROSS_MARK.red(),
                        dir.display().to_string().dimmed()
                    );
                }
                Err(_) => errors += 1,
            }
        }

        println!();
        ui::print_line(50);
        println!(
            "{} Deleted: {} files, {} directories{}",
            chars::ARROW.dimmed(),
            deleted_files.to_string().green().bold(),
            deleted_dirs.to_string().green().bold(),
            if errors > 0 {
                format!(" ({} errors)", errors).red().to_string()
            } else {
                String::new()
            }
        );
    }

    Ok(())
}

fn is_dir_empty(path: &std::path::Path) -> bool {
    match fs::read_dir(path) {
        Ok(mut entries) => entries.next().is_none(),
        Err(_) => false,
    }
}
