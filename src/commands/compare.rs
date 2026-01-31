use anyhow::Result;
use colored::*;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use walkdir::WalkDir;

use crate::ui::{self, chars};
use crate::utils::{format_bytes, hash_file_sha256};

pub fn run(dir1: &str, dir2: &str, content: bool, diff_only: bool) -> Result<()> {
    ui::print_start("Comparing directories", "");
    println!("  {} {}", "A:".yellow(), dir1.blue());
    println!("  {} {}", "B:".yellow(), dir2.blue());
    println!();

    let files1 = collect_files(dir1)?;
    let files2 = collect_files(dir2)?;

    let names1: HashSet<_> = files1.keys().collect();
    let names2: HashSet<_> = files2.keys().collect();

    let only_in_1: Vec<_> = names1.difference(&names2).collect();
    let only_in_2: Vec<_> = names2.difference(&names1).collect();
    let in_both: Vec<_> = names1.intersection(&names2).collect();

    let mut modified = Vec::new();
    let mut identical = Vec::new();

    for name in &in_both {
        let path1 = &files1[**name];
        let path2 = &files2[**name];

        let meta1 = path1.metadata().ok();
        let meta2 = path2.metadata().ok();

        let size_match = match (&meta1, &meta2) {
            (Some(m1), Some(m2)) => m1.len() == m2.len(),
            _ => false,
        };

        if content {
            if size_match {
                let hash1 = hash_file_sha256(path1).ok();
                let hash2 = hash_file_sha256(path2).ok();

                if hash1 == hash2 {
                    identical.push(**name);
                } else {
                    modified.push(**name);
                }
            } else {
                modified.push(**name);
            }
        } else {
            if size_match {
                identical.push(**name);
            } else {
                modified.push(**name);
            }
        }
    }

    let total_changes = only_in_1.len() + only_in_2.len() + modified.len();

    if total_changes == 0 {
        ui::print_success("Directories are identical");
        return Ok(());
    }

    // Summary header
    ui::print_header("COMPARISON RESULT");
    println!();
    ui::print_kv_colored("Only in A", only_in_1.len().to_string().yellow().bold());
    ui::print_kv_colored("Only in B", only_in_2.len().to_string().yellow().bold());
    ui::print_kv_colored("Modified", modified.len().to_string().red().bold());
    if !diff_only {
        ui::print_kv_colored("Identical", identical.len().to_string().green().bold());
    }
    println!();
    ui::print_line(60);

    // Only in A
    if !only_in_1.is_empty() {
        println!();
        ui::print_section("Only in A");
        for name in &only_in_1 {
            let path = &files1[**name];
            let size = path.metadata().map(|m| m.len()).unwrap_or(0);
            println!(
                "  {} {} {}",
                chars::CROSS_MARK.red(),
                name.red(),
                format!("({})", format_bytes(size)).dimmed()
            );
        }
    }

    // Only in B
    if !only_in_2.is_empty() {
        println!();
        ui::print_section("Only in B");
        for name in &only_in_2 {
            let path = &files2[**name];
            let size = path.metadata().map(|m| m.len()).unwrap_or(0);
            println!(
                "  {} {} {}",
                chars::CHECK.green(),
                name.green(),
                format!("({})", format_bytes(size)).dimmed()
            );
        }
    }

    // Modified
    if !modified.is_empty() {
        println!();
        ui::print_section("Modified");
        for name in &modified {
            let path1 = &files1[*name];
            let path2 = &files2[*name];
            let size1 = path1.metadata().map(|m| m.len()).unwrap_or(0);
            let size2 = path2.metadata().map(|m| m.len()).unwrap_or(0);

            let size_diff = if size2 > size1 {
                format!("+{}", format_bytes(size2 - size1)).green()
            } else if size1 > size2 {
                format!("-{}", format_bytes(size1 - size2)).red()
            } else {
                "content differs".yellow()
            };

            println!(
                "  {} {} [{}]",
                chars::BULLET.yellow(),
                name.yellow(),
                size_diff
            );
        }
    }

    println!();
    ui::print_line(60);

    Ok(())
}

fn collect_files(base: &str) -> Result<HashMap<String, PathBuf>> {
    let mut files = HashMap::new();
    let base_path = PathBuf::from(base);

    for entry in WalkDir::new(base)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Ok(relative) = path.strip_prefix(&base_path) {
                files.insert(relative.display().to_string(), path.to_path_buf());
            }
        }
    }

    Ok(files)
}
