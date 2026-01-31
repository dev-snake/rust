use anyhow::Result;
use chrono::{DateTime, Local};
use colored::*;
use std::time::SystemTime;
use walkdir::WalkDir;

use crate::ui::{self, chars};
use crate::utils::{format_bytes, parse_duration, should_skip};

pub fn run(path: &str, within: &str, top: usize) -> Result<()> {
    let seconds = parse_duration(within)?;
    let cutoff = SystemTime::now() - std::time::Duration::from_secs(seconds);

    ui::print_start(&format!("Finding files modified within {}", within.green()), path);
    println!();

    let mut recent_files: Vec<(String, u64, DateTime<Local>)> = Vec::new();

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
            if let Ok(modified) = metadata.modified() {
                if modified > cutoff {
                    let size = metadata.len();
                    let datetime = DateTime::<Local>::from(modified);
                    recent_files.push((entry_path.display().to_string(), size, datetime));
                }
            }
        }
    }

    recent_files.sort_by(|a, b| b.2.cmp(&a.2));
    recent_files.truncate(top);

    if recent_files.is_empty() {
        ui::print_warning(&format!("No files modified within {}", within));
        return Ok(());
    }

    ui::print_info(&format!(
        "Found {} files",
        recent_files.len().to_string().green().bold()
    ));
    println!();

    // Table header
    println!(
        "  {:>19}  {:>12}  {}",
        "MODIFIED".cyan().bold(),
        "SIZE".cyan().bold(),
        "FILE".cyan().bold()
    );
    ui::print_line(80);

    let now = Local::now();

    for (file_path, size, modified) in &recent_files {
        let duration = now.signed_duration_since(*modified);
        
        let relative_time = if duration.num_seconds() < 60 {
            format!("{}s ago", duration.num_seconds())
        } else if duration.num_minutes() < 60 {
            format!("{}m ago", duration.num_minutes())
        } else if duration.num_hours() < 24 {
            format!("{}h ago", duration.num_hours())
        } else {
            format!("{}d ago", duration.num_days())
        };

        let time_str = format!(
            "{} {}",
            modified.format("%Y-%m-%d %H:%M").to_string().dimmed(),
            format!("({})", relative_time).yellow()
        );

        println!(
            "  {}  {:>12}  {}",
            time_str,
            format_bytes(*size).dimmed(),
            file_path
        );
    }

    println!();
    ui::print_line(80);
    println!(
        "{} {} recent files",
        chars::ARROW.dimmed(),
        recent_files.len().to_string().green().bold()
    );

    Ok(())
}
