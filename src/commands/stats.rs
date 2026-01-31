use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use walkdir::WalkDir;

use crate::ui;
use crate::utils::{format_bytes, get_extension, should_skip};

pub fn run(path: &str, hidden: bool) -> Result<()> {
    ui::print_start("Analyzing", path);
    println!();

    let mut total_files = 0u64;
    let mut total_dirs = 0u64;
    let mut total_size = 0u64;
    let mut max_size = 0u64;
    let mut max_file = String::new();
    let mut extension_count: HashMap<String, usize> = HashMap::new();
    let mut extension_size: HashMap<String, u64> = HashMap::new();

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();

        if !hidden && should_skip(entry_path, false) {
            continue;
        }

        if entry_path.is_dir() {
            total_dirs += 1;
        } else if entry_path.is_file() {
            total_files += 1;

            if let Ok(metadata) = entry_path.metadata() {
                let size = metadata.len();
                total_size += size;

                if size > max_size {
                    max_size = size;
                    max_file = entry_path.display().to_string();
                }

                let ext = get_extension(entry_path);
                *extension_count.entry(ext.clone()).or_insert(0) += 1;
                *extension_size.entry(ext).or_insert(0) += size;
            }
        }
    }

    let avg_size = if total_files > 0 {
        total_size / total_files
    } else {
        0
    };

    let mut ext_by_count: Vec<_> = extension_count.iter().collect();
    ext_by_count.sort_by(|a, b| b.1.cmp(a.1));

    let mut ext_by_size: Vec<_> = extension_size.iter().collect();
    ext_by_size.sort_by(|a, b| b.1.cmp(a.1));

    // Print statistics
    ui::print_header("DIRECTORY STATISTICS");
    println!();

    ui::print_section("Overview");
    ui::print_kv("Total files", &total_files.to_string());
    ui::print_kv("Total directories", &total_dirs.to_string());
    ui::print_kv_colored("Total size", format_bytes(total_size).green().bold());
    ui::print_kv("Average file size", &format_bytes(avg_size));

    if !max_file.is_empty() {
        println!();
        ui::print_section("Largest File");
        ui::print_kv_colored("Size", format_bytes(max_size).red().bold());
        ui::print_kv("Path", &max_file);
    }

    println!();
    ui::print_section("Top Extensions by Count");
    println!();
    
    for (ext, count) in ext_by_count.iter().take(10) {
        let percentage = (**count as f64 / total_files as f64) * 100.0;
        let bar = ui::progress_bar(percentage, 15);

        let ext_display = if *ext == "(no ext)" {
            ext.dimmed().to_string()
        } else {
            format!(".{}", ext).cyan().to_string()
        };

        println!(
            "  {:>8} {:>6} {:>5.1}% {}",
            ext_display,
            count,
            percentage,
            bar
        );
    }

    println!();
    ui::print_section("Top Extensions by Size");
    println!();
    
    for (ext, size) in ext_by_size.iter().take(10) {
        let percentage = (**size as f64 / total_size as f64) * 100.0;
        let bar = ui::progress_bar(percentage, 15);

        let ext_display = if *ext == "(no ext)" {
            ext.dimmed().to_string()
        } else {
            format!(".{}", ext).cyan().to_string()
        };

        println!(
            "  {:>8} {:>10} {:>5.1}% {}",
            ext_display,
            format_bytes(**size),
            percentage,
            bar
        );
    }

    println!();
    ui::print_line(50);

    Ok(())
}
