use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use serde::Serialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::ui::{self, chars};
use crate::utils::{format_bytes, hash_file_sha256, matches_extensions, should_skip};

#[derive(Serialize)]
struct DuplicateGroup {
    hash: String,
    size: u64,
    files: Vec<String>,
}

#[derive(Serialize)]
struct DuplicateReport {
    total_groups: usize,
    total_duplicates: usize,
    wasted_space: u64,
    groups: Vec<DuplicateGroup>,
}

pub fn run(
    path: &str,
    min_size: u64,
    extensions: Option<String>,
    output: Option<String>,
    delete: bool,
) -> Result<()> {
    ui::print_start("Scanning for duplicates", path);

    // Step 1: Collect all files and group by size
    let mut size_groups: HashMap<u64, Vec<PathBuf>> = HashMap::new();
    let mut file_count = 0u64;

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_file() && !should_skip(path, false) {
            if let Ok(metadata) = path.metadata() {
                let size = metadata.len();

                if size >= min_size && matches_extensions(path, &extensions) {
                    size_groups
                        .entry(size)
                        .or_default()
                        .push(path.to_path_buf());
                    file_count += 1;
                }
            }
        }
    }

    println!(
        "  {} files indexed",
        file_count.to_string().green()
    );

    // Step 2: Filter groups with more than one file (potential duplicates)
    let potential_dupes: Vec<(u64, Vec<PathBuf>)> = size_groups
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .collect();

    if potential_dupes.is_empty() {
        ui::print_success("No duplicate files found");
        return Ok(());
    }

    let total_to_hash: usize = potential_dupes.iter().map(|(_, f)| f.len()).sum();
    println!(
        "  {} candidates with matching sizes",
        total_to_hash.to_string().yellow()
    );

    // Step 3: Calculate hashes for potential duplicates
    let pb = ProgressBar::new(total_to_hash as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len}")?
            .progress_chars("━━─"),
    );

    let mut hash_groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for (_, files) in potential_dupes {
        let hashes: Vec<(PathBuf, Option<String>)> = files
            .par_iter()
            .map(|f| {
                let hash = hash_file_sha256(f).ok();
                pb.inc(1);
                (f.clone(), hash)
            })
            .collect();

        for (file, hash) in hashes {
            if let Some(h) = hash {
                hash_groups.entry(h).or_default().push(file);
            }
        }
    }

    pb.finish_and_clear();

    // Step 4: Filter to actual duplicates
    let duplicates: Vec<(String, Vec<PathBuf>)> = hash_groups
        .into_iter()
        .filter(|(_, files)| files.len() > 1)
        .collect();

    if duplicates.is_empty() {
        ui::print_success("No duplicate files found");
        return Ok(());
    }

    // Calculate statistics
    let total_groups = duplicates.len();
    let total_duplicates: usize = duplicates.iter().map(|(_, f)| f.len() - 1).sum();
    let wasted_space: u64 = duplicates
        .iter()
        .filter_map(|(_, files)| {
            files.first().and_then(|f| f.metadata().ok()).map(|m| {
                m.len() * (files.len() as u64 - 1)
            })
        })
        .sum();

    // Print results
    ui::print_header("DUPLICATE FILES REPORT");
    println!();
    ui::print_kv("Duplicate groups", &total_groups.to_string());
    ui::print_kv("Total duplicates", &total_duplicates.to_string());
    ui::print_kv_colored("Wasted space", format_bytes(wasted_space).red().bold());
    println!();
    ui::print_line(60);

    // Print each group
    for (hash, files) in &duplicates {
        let size = files
            .first()
            .and_then(|f| f.metadata().ok())
            .map(|m| m.len())
            .unwrap_or(0);

        println!();
        println!(
            "  {} {} files, {} each",
            chars::BULLET.yellow(),
            files.len().to_string().yellow().bold(),
            format_bytes(size).dimmed()
        );
        println!("    {} {}", "hash:".dimmed(), &hash[..16].dimmed());

        for (i, file) in files.iter().enumerate() {
            let (prefix, label) = if i == 0 {
                (chars::T_RIGHT.green(), "keep".green())
            } else {
                (chars::T_RIGHT.red(), "dupe".red())
            };
            println!("    {} [{}] {}", prefix, label, file.display());
        }
    }

    println!();
    ui::print_line(60);

    // Export to JSON if requested
    if let Some(output_path) = output {
        let report = DuplicateReport {
            total_groups,
            total_duplicates,
            wasted_space,
            groups: duplicates
                .iter()
                .map(|(hash, files)| {
                    let size = files
                        .first()
                        .and_then(|f| f.metadata().ok())
                        .map(|m| m.len())
                        .unwrap_or(0);
                    DuplicateGroup {
                        hash: hash.clone(),
                        size,
                        files: files.iter().map(|f| f.display().to_string()).collect(),
                    }
                })
                .collect(),
        };

        let json = serde_json::to_string_pretty(&report)?;
        fs::write(&output_path, json)?;
        ui::print_success(&format!("Report saved to {}", output_path));
    }

    // Delete duplicates if requested
    if delete {
        println!();
        ui::print_warning("Deleting duplicates (keeping first occurrence)...");

        let mut deleted_count = 0;
        let mut freed_space = 0u64;

        for (_, files) in &duplicates {
            for file in files.iter().skip(1) {
                if let Ok(metadata) = file.metadata() {
                    freed_space += metadata.len();
                }
                if fs::remove_file(file).is_ok() {
                    deleted_count += 1;
                    println!(
                        "    {} {}",
                        chars::CROSS_MARK.red(),
                        file.display().to_string().dimmed()
                    );
                }
            }
        }

        println!();
        ui::print_success(&format!(
            "Deleted {} files, freed {}",
            deleted_count,
            format_bytes(freed_space)
        ));
    }

    Ok(())
}
