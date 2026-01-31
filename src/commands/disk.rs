use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use walkdir::WalkDir;

use crate::ui;
use crate::utils::{format_bytes, get_extension, parse_size, should_skip};

struct DirSize {
    path: String,
    size: u64,
    file_count: usize,
}

struct ExtSize {
    extension: String,
    size: u64,
    file_count: usize,
}

pub fn run(
    path: &str,
    top: usize,
    by_type: bool,
    hidden: bool,
    min: Option<String>,
    csv_output: Option<String>,
) -> Result<()> {
    let min_size = match &min {
        Some(s) => parse_size(s)?,
        None => 0,
    };

    ui::print_start("Analyzing disk usage", path);
    println!();

    if by_type {
        analyze_by_type(path, top, hidden, min_size, csv_output)
    } else {
        analyze_by_directory(path, top, hidden, min_size, csv_output)
    }
}

fn analyze_by_directory(
    path: &str,
    top: usize,
    hidden: bool,
    min_size: u64,
    csv_output: Option<String>,
) -> Result<()> {
    let mut dir_sizes: HashMap<String, (u64, usize)> = HashMap::new();
    let mut total_size = 0u64;
    let mut total_files = 0usize;

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();

        if !hidden && should_skip(entry_path, false) {
            continue;
        }

        if entry_path.is_file() {
            if let Ok(metadata) = entry_path.metadata() {
                let size = metadata.len();
                total_size += size;
                total_files += 1;

                if let Some(parent) = entry_path.parent() {
                    let parent_str = parent.display().to_string();
                    let entry = dir_sizes.entry(parent_str).or_insert((0, 0));
                    entry.0 += size;
                    entry.1 += 1;
                }
            }
        }
    }

    let mut dirs: Vec<DirSize> = dir_sizes
        .into_iter()
        .filter(|(_, (size, _))| *size >= min_size)
        .map(|(path, (size, count))| DirSize {
            path,
            size,
            file_count: count,
        })
        .collect();

    dirs.sort_by(|a, b| b.size.cmp(&a.size));
    dirs.truncate(top);

    if dirs.is_empty() {
        ui::print_warning("No directories found matching criteria");
        return Ok(());
    }

    let max_size = dirs.first().map(|d| d.size).unwrap_or(1);

    // Print header
    ui::print_header("DISK USAGE BY DIRECTORY");
    println!();
    ui::print_info(&format!(
        "Total: {} in {} files",
        format_bytes(total_size).bright_green().bold(),
        total_files.to_string().bright_green()
    ));
    println!();

    // Table
    println!(
        "  {:>12}  {:>6}  {:22}  {}",
        "SIZE".cyan().bold(),
        "FILES".cyan().bold(),
        "".to_string(),
        "DIRECTORY".cyan().bold()
    );
    ui::print_line(80);

    for dir in &dirs {
        let percentage = (dir.size as f64 / total_size as f64) * 100.0;
        let bar_width = 20;
        let filled = ((dir.size as f64 / max_size as f64) * bar_width as f64) as usize;
        let bar = format!(
            "{}{}",
            "━".repeat(filled).cyan(),
            "─".repeat(bar_width - filled).dimmed()
        );

        println!(
            "  {:>12}  {:>6}  {} {:>5.1}%  {}",
            format_bytes(dir.size).bright_yellow().bold(),
            dir.file_count.to_string().bright_white(),
            bar,
            percentage,
            dir.path.bright_black()
        );
    }

    ui::print_line(80);

    // CSV export
    if let Some(csv_path) = csv_output {
        let mut file = File::create(&csv_path)?;
        writeln!(file, "directory,size_bytes,file_count")?;
        for dir in &dirs {
            writeln!(file, "\"{}\",{},{}", dir.path, dir.size, dir.file_count)?;
        }
        ui::print_success(&format!("Exported to {}", csv_path));
    }

    Ok(())
}

fn analyze_by_type(
    path: &str,
    top: usize,
    hidden: bool,
    min_size: u64,
    csv_output: Option<String>,
) -> Result<()> {
    let mut ext_sizes: HashMap<String, (u64, usize)> = HashMap::new();
    let mut total_size = 0u64;

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();

        if !hidden && should_skip(entry_path, false) {
            continue;
        }

        if entry_path.is_file() {
            if let Ok(metadata) = entry_path.metadata() {
                let size = metadata.len();
                total_size += size;

                let ext = get_extension(entry_path);
                let entry = ext_sizes.entry(ext).or_insert((0, 0));
                entry.0 += size;
                entry.1 += 1;
            }
        }
    }

    let mut exts: Vec<ExtSize> = ext_sizes
        .into_iter()
        .filter(|(_, (size, _))| *size >= min_size)
        .map(|(ext, (size, count))| ExtSize {
            extension: ext,
            size,
            file_count: count,
        })
        .collect();

    exts.sort_by(|a, b| b.size.cmp(&a.size));
    exts.truncate(top);

    if exts.is_empty() {
        ui::print_warning("No file types found matching criteria");
        return Ok(());
    }

    let max_size = exts.first().map(|e| e.size).unwrap_or(1);

    // Print
    ui::print_header("DISK USAGE BY FILE TYPE");
    println!();
    ui::print_info(&format!(
        "Total: {}",
        format_bytes(total_size).bright_green().bold()
    ));
    println!();

    println!(
        "  {:>8}  {:>12}  {:>6}  {:22}  {}",
        "EXT".cyan().bold(),
        "SIZE".cyan().bold(),
        "FILES".cyan().bold(),
        "".to_string(),
        "%".cyan().bold()
    );
    ui::print_line(70);

    for ext in &exts {
        let percentage = (ext.size as f64 / total_size as f64) * 100.0;
        let bar_width = 20;
        let filled = ((ext.size as f64 / max_size as f64) * bar_width as f64) as usize;
        let bar = format!(
            "{}{}",
            "━".repeat(filled).green(),
            "─".repeat(bar_width - filled).dimmed()
        );

        let ext_display = if ext.extension == "(no ext)" {
            ext.extension.bright_black().to_string()
        } else {
            format!(".{}", ext.extension).bright_cyan().to_string()
        };

        println!(
            "  {:>8}  {:>12}  {:>6}  {}  {:>5.1}%",
            ext_display,
            format_bytes(ext.size).bright_yellow().bold(),
            ext.file_count.to_string().bright_white(),
            bar,
            percentage
        );
    }

    ui::print_line(70);

    // CSV export
    if let Some(csv_path) = csv_output {
        let mut file = File::create(&csv_path)?;
        writeln!(file, "extension,size_bytes,file_count")?;
        for ext in &exts {
            writeln!(file, "\"{}\",{},{}", ext.extension, ext.size, ext.file_count)?;
        }
        ui::print_success(&format!("Exported to {}", csv_path));
    }

    Ok(())
}
