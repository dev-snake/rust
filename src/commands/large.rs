use anyhow::Result;
use colored::*;
use walkdir::WalkDir;

use crate::ui;
use crate::utils::{format_bytes, parse_size, should_skip};

pub fn run(path: &str, size_str: &str, top: usize) -> Result<()> {
    let min_size = parse_size(size_str)?;

    ui::print_start(
        &format!("Finding large files (>= {})", format_bytes(min_size).bright_green()),
        path,
    );
    println!();

    let mut large_files: Vec<(String, u64)> = Vec::new();

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();

        if !entry_path.is_file() || should_skip(entry_path, false) {
            continue;
        }

        if let Ok(metadata) = entry_path.metadata() {
            let size = metadata.len();
            if size >= min_size {
                large_files.push((entry_path.display().to_string(), size));
            }
        }
    }

    large_files.sort_by(|a, b| b.1.cmp(&a.1));
    large_files.truncate(top);

    if large_files.is_empty() {
        ui::print_warning(&format!("No files found >= {}", format_bytes(min_size)));
        return Ok(());
    }

    let total_size: u64 = large_files.iter().map(|(_, s)| s).sum();
    let max_size = large_files.first().map(|(_, s)| *s).unwrap_or(1);

    ui::print_info(&format!(
        "Found {} files, total {}",
        large_files.len().to_string().bright_green().bold(),
        format_bytes(total_size).bright_green().bold()
    ));
    println!();

    // Table header
    println!(
        "  {:>4}  {:>12}  {:20}  {}",
        "#".bright_black(),
        "SIZE".bright_cyan().bold(),
        "".to_string(),
        "FILE".bright_cyan().bold()
    );
    ui::print_line(80);

    for (i, (file_path, size)) in large_files.iter().enumerate() {
        let rank = format!("{:>4}", i + 1).bright_black();
        let size_str = format!("{:>12}", format_bytes(*size)).bright_yellow().bold();

        let bar_width = 20;
        let filled = ((*size as f64 / max_size as f64) * bar_width as f64) as usize;
        let bar = format!(
            "{}{}",
            "━".repeat(filled).bright_green(),
            "─".repeat(bar_width - filled).bright_black()
        );

        println!("  {}  {}  {}  {}", rank, size_str, bar, file_path);
    }

    ui::print_count(large_files.len(), "large file", "large files");

    Ok(())
}
