use anyhow::{anyhow, Result};
use colored::*;
use rayon::prelude::*;
use serde::Serialize;
use std::path::Path;

use crate::ui::{self, chars};
use crate::utils::{hash_file_sha256, hash_file_sha512, hash_file_md5};

#[derive(Serialize)]
struct HashResult {
    file: String,
    algorithm: String,
    hash: String,
}

pub fn run(
    files: Vec<String>,
    algorithm: &str,
    verify: Option<String>,
    format: &str,
) -> Result<()> {
    if files.is_empty() {
        return Err(anyhow!("No files specified"));
    }

    let algorithm = algorithm.to_lowercase();
    
    if !["sha256", "sha512", "md5"].contains(&algorithm.as_str()) {
        return Err(anyhow!(
            "Unsupported algorithm: {}. Use sha256, sha512, or md5",
            algorithm
        ));
    }

    let results: Vec<(String, Result<String>)> = files
        .par_iter()
        .map(|file| {
            let path = Path::new(file);
            if !path.exists() {
                return (file.clone(), Err(anyhow!("File not found")));
            }
            if !path.is_file() {
                return (file.clone(), Err(anyhow!("Not a file")));
            }

            let hash_result = match algorithm.as_str() {
                "sha256" => hash_file_sha256(path),
                "sha512" => hash_file_sha512(path),
                "md5" => hash_file_md5(path),
                _ => Err(anyhow!("Unsupported algorithm")),
            };

            (file.clone(), hash_result)
        })
        .collect();

    // Verify mode
    if let Some(expected_hash) = verify {
        if files.len() != 1 {
            return Err(anyhow!("--verify can only be used with a single file"));
        }

        let (file, result) = &results[0];
        match result {
            Ok(hash) => {
                let expected = expected_hash.to_lowercase();
                let actual = hash.to_lowercase();

                if actual == expected || actual.starts_with(&expected) || expected.starts_with(&actual) {
                    println!(
                        "{} {} {}",
                        format!("[{}]", chars::CHECK).green().bold(),
                        file.green().bold(),
                        "MATCH".green().bold()
                    );
                    return Ok(());
                } else {
                    println!(
                        "{} {} {}",
                        format!("[{}]", chars::CROSS_MARK).red().bold(),
                        file.red().bold(),
                        "MISMATCH".red().bold()
                    );
                    ui::print_kv("Expected", &expected);
                    ui::print_kv_colored("Actual", actual.red());
                    return Err(anyhow!("Hash verification failed"));
                }
            }
            Err(e) => {
                return Err(anyhow!("Failed to hash {}: {}", file, e));
            }
        }
    }

    // Output results
    match format {
        "json" => {
            let json_results: Vec<HashResult> = results
                .iter()
                .filter_map(|(file, result)| {
                    result.as_ref().ok().map(|hash| HashResult {
                        file: file.clone(),
                        algorithm: algorithm.clone(),
                        hash: hash.clone(),
                    })
                })
                .collect();

            println!("{}", serde_json::to_string_pretty(&json_results)?);
        }
        _ => {
            println!(
                "{} File Hashes ({})",
                chars::BULLET.cyan(),
                algorithm.to_uppercase().yellow()
            );
            ui::print_line(80);

            for (file, result) in &results {
                match result {
                    Ok(hash) => {
                        println!("{}", hash.green());
                        println!("  {} {}", chars::BL_CORNER.dimmed(), file.dimmed());
                    }
                    Err(e) => {
                        ui::print_error(&format!("{} ({})", file, e));
                    }
                }
            }
        }
    }

    Ok(())
}
