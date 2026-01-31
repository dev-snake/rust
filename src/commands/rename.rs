use anyhow::Result;
use colored::*;
use regex::Regex;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::ui::{self, chars};
use crate::utils::matches_extensions;

pub fn run(
    path: &str,
    find: &str,
    replace: &str,
    extensions: Option<String>,
    dry_run: bool,
    recursive: bool,
) -> Result<()> {
    let regex = Regex::new(find)?;

    ui::print_start("Bulk rename", path);
    println!(
        "  {} '{}' {} '{}'",
        "Pattern:".dimmed(),
        find.yellow(),
        chars::ARROW.dimmed(),
        replace.green()
    );
    println!(
        "  {} {}",
        "Mode:".dimmed(),
        if dry_run {
            "DRY RUN (preview only)".yellow()
        } else {
            "LIVE (will rename files)".red().bold()
        }
    );
    println!();

    let walker = if recursive {
        WalkDir::new(path).follow_links(false)
    } else {
        WalkDir::new(path).max_depth(1).follow_links(false)
    };

    let mut changes: Vec<(PathBuf, PathBuf)> = Vec::new();

    for entry in walker.into_iter().filter_map(|e| e.ok()) {
        let file_path = entry.path();

        if !file_path.is_file() {
            continue;
        }

        if !matches_extensions(file_path, &extensions) {
            continue;
        }

        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
            if regex.is_match(file_name) {
                let new_name = regex.replace_all(file_name, replace);
                if new_name != file_name {
                    let new_path = file_path.with_file_name(new_name.as_ref());
                    changes.push((file_path.to_path_buf(), new_path));
                }
            }
        }
    }

    if changes.is_empty() {
        ui::print_warning("No files match the pattern");
        return Ok(());
    }

    // Check for conflicts
    let mut conflicts = Vec::new();
    let new_names: Vec<_> = changes.iter().map(|(_, new)| new.clone()).collect();
    for (i, (_, new_path)) in changes.iter().enumerate() {
        if new_path.exists() && !changes.iter().any(|(old, _)| old == new_path) {
            conflicts.push((new_path.clone(), "already exists"));
        }
        for (j, other) in new_names.iter().enumerate() {
            if i != j && new_path == other {
                conflicts.push((new_path.clone(), "duplicate target"));
            }
        }
    }

    if !conflicts.is_empty() {
        ui::print_section("Conflicts Detected");
        for (path, reason) in &conflicts {
            println!(
                "  {} {} ({})",
                chars::CROSS_MARK.red(),
                path.display(),
                reason.red()
            );
        }
        println!();
        if !dry_run {
            ui::print_error("Aborting due to conflicts");
            return Ok(());
        }
    }

    // Display changes
    ui::print_section(&format!("Changes ({})", changes.len()));
    println!();

    for (old, new) in &changes {
        let old_name = old.file_name().unwrap_or_default().to_string_lossy();
        let new_name = new.file_name().unwrap_or_default().to_string_lossy();
        println!(
            "  {} {}  {}  {}",
            chars::BULLET.dimmed(),
            old_name.red(),
            chars::ARROW.dimmed(),
            new_name.green()
        );
    }

    // Execute if not dry run
    if !dry_run {
        println!();
        ui::print_section("Executing");

        let mut success_count = 0;
        let mut error_count = 0;

        for (old, new) in &changes {
            match fs::rename(old, new) {
                Ok(_) => {
                    success_count += 1;
                    println!(
                        "  {} {}",
                        chars::CHECK.green(),
                        new.file_name().unwrap_or_default().to_string_lossy()
                    );
                }
                Err(e) => {
                    error_count += 1;
                    println!(
                        "  {} {} ({})",
                        chars::CROSS_MARK.red(),
                        old.file_name().unwrap_or_default().to_string_lossy(),
                        e.to_string().red()
                    );
                }
            }
        }

        println!();
        ui::print_line(50);
        println!(
            "{} {} renamed, {} failed",
            chars::ARROW.dimmed(),
            success_count.to_string().green().bold(),
            error_count.to_string().red()
        );
    } else {
        println!();
        ui::print_info("Run without --dry-run to apply changes");
    }

    Ok(())
}
