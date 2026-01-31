use anyhow::Result;
use colored::*;
use walkdir::WalkDir;

use crate::ui::{self, chars};
use crate::utils::{format_bytes, parse_size, should_skip};

pub fn run(path: &str, size_str: &str, top: usize) -> Result<()> {
    let min_size = parse_size(size_str)?;

    ui::print_start(
        &format!("Finding large files (>= {})", format_bytes(min_size).green()),
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
        large_files.len().to_string().green().bold(),
        format_bytes(total_size).green().bold()
    ));
    println!();

    // Table header
    println!(
        "  {:>4}  {:>12}  {:20}  {}",
        "#".dimmed(),
        "SIZE".cyan().bold(),
        "".to_string(),
        "FILE".cyan().bold()
    );
    ui::print_line(80);

    for (i, (file_path, size)) in large_files.iter().enumerate() {
        let rank = format!("{:>4}", i + 1).dimmed();
        let size_str = format!("{:>12}", format_bytes(*size)).yellow().bold();

        let bar_width = 20;
        let filled = ((*size as f64 / max_size as f64) * bar_width as f64) as usize;
        let bar = format!(
            "{}{}",
            "━".repeat(filled).green(),
            "─".repeat(bar_width - filled).dimmed()
        );

        println!("  {}  {}  {}  {}", rank, size_str, bar, file_path);
    }

    println!();
    ui::print_line(80);
    println!(
        "{} {} files totaling {}",
        chars::ARROW.dimmed(),
        large_files.len().to_string().green().bold(),
        format_bytes(total_size).green().bold()
    );

    Ok(())
}
