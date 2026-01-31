use anyhow::Result;
use colored::*;
use regex::{Regex, RegexBuilder};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

use crate::ui::{self, chars};
use crate::utils::{matches_extensions, should_skip};

pub fn run(
    pattern: &str,
    path: &str,
    extensions: Option<String>,
    ignore_case: bool,
    files_only: bool,
    line_numbers: bool,
    context: usize,
) -> Result<()> {
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(ignore_case)
        .build()?;

    ui::print_start(&format!("Searching for '{}'", pattern.bright_yellow()), path);
    println!();

    let mut total_matches = 0usize;
    let mut files_with_matches = 0usize;

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let file_path = entry.path();

        if !file_path.is_file() || should_skip(file_path, false) {
            continue;
        }

        if !matches_extensions(file_path, &extensions) {
            continue;
        }

        if is_binary_file(file_path) {
            continue;
        }

        match search_file(file_path, &regex, files_only, line_numbers, context) {
            Ok(matches) if !matches.is_empty() => {
                files_with_matches += 1;
                total_matches += matches.len();

                if files_only {
                    println!("{}", file_path.display().to_string().green());
                } else {
                    println!("{}", file_path.display().to_string().bright_magenta().bold());
                    for m in matches {
                        println!("{}", m);
                    }
                    println!();
                }
            }
            Err(_) => continue,
            _ => continue,
        }
    }

    // Summary
    ui::print_count(total_matches, "match", "matches");
    println!(
        "{} found in {} files",
        chars::ARROW.bright_black(),
        files_with_matches.to_string().bright_green().bold()
    );

    Ok(())
}

fn search_file(
    path: &Path,
    regex: &Regex,
    files_only: bool,
    line_numbers: bool,
    context: usize,
) -> Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(|l| l.ok()).collect();

    let mut results = Vec::new();
    let mut matched_lines: Vec<usize> = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if regex.is_match(line) {
            matched_lines.push(i);
        }
    }

    if matched_lines.is_empty() {
        return Ok(results);
    }

    if files_only {
        results.push(String::new());
        return Ok(results);
    }

    let mut displayed: std::collections::HashSet<usize> = std::collections::HashSet::new();

    for &match_line in &matched_lines {
        let start = match_line.saturating_sub(context);
        let end = (match_line + context + 1).min(lines.len());

        for i in start..end {
            if displayed.contains(&i) {
                continue;
            }
            displayed.insert(i);

            let line_num = if line_numbers {
                format!("{:>4} {} ", i + 1, chars::V_LINE).dimmed().to_string()
            } else {
                String::new()
            };

            let content = &lines[i];
            let formatted = if i == match_line {
                let highlighted = regex.replace_all(content, |caps: &regex::Captures| {
                    caps[0].red().bold().to_string()
                });
                format!("{}{}", line_num, highlighted)
            } else {
                format!("{}{}", line_num, content.dimmed())
            };

            results.push(formatted);
        }

        if context > 0 && end < lines.len() {
            results.push(format!("  {}", chars::DOT.repeat(3).bright_black()));
        }
    }

    Ok(results)
}

fn is_binary_file(path: &Path) -> bool {
    if let Ok(file) = File::open(path) {
        let mut reader = BufReader::new(file);
        let mut buffer = [0u8; 512];
        if let Ok(bytes_read) = std::io::Read::read(&mut reader, &mut buffer) {
            for byte in &buffer[..bytes_read] {
                if *byte == 0 {
                    return true;
                }
            }
        }
    }
    false
}
