mod commands;
mod ui;
mod utils;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(name = "ftools")]
#[command(author = "Your Name")]
#[command(version = "1.0.0")]
#[command(about = "A powerful CLI toolkit for file operations", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Find duplicate files by content hash
    #[command(name = "dupes")]
    FindDuplicates {
        /// Directory to scan
        #[arg(default_value = ".")]
        path: String,

        /// Minimum file size in bytes (skip smaller files)
        #[arg(short, long, default_value = "1")]
        min_size: u64,

        /// File extension filter (e.g., "jpg,png,gif")
        #[arg(short, long)]
        extensions: Option<String>,

        /// Output results to JSON file
        #[arg(short, long)]
        output: Option<String>,

        /// Delete duplicates (keep first occurrence)
        #[arg(long, default_value = "false")]
        delete: bool,
    },

    /// Search for text pattern in files (grep-like)
    #[command(name = "search")]
    Search {
        /// Pattern to search (supports regex)
        pattern: String,

        /// Directory to search in
        #[arg(default_value = ".")]
        path: String,

        /// File extension filter
        #[arg(short, long)]
        extensions: Option<String>,

        /// Case insensitive search
        #[arg(short, long, default_value = "false")]
        ignore_case: bool,

        /// Show only filenames
        #[arg(short = 'l', long, default_value = "false")]
        files_only: bool,

        /// Show line numbers
        #[arg(short = 'n', long, default_value = "true")]
        line_numbers: bool,

        /// Context lines before/after match
        #[arg(short = 'C', long, default_value = "0")]
        context: usize,
    },

    /// Bulk rename files with regex pattern
    #[command(name = "rename")]
    BulkRename {
        /// Directory containing files
        #[arg(default_value = ".")]
        path: String,

        /// Search pattern (regex)
        #[arg(short, long)]
        find: String,

        /// Replacement string (supports $1, $2 for groups)
        #[arg(short, long)]
        replace: String,

        /// File extension filter
        #[arg(short, long)]
        extensions: Option<String>,

        /// Dry run - show changes without applying
        #[arg(long, default_value = "true")]
        dry_run: bool,

        /// Recursive rename in subdirectories
        #[arg(short = 'R', long, default_value = "false")]
        recursive: bool,
    },

    /// Analyze disk usage by directory or file type
    #[command(name = "size")]
    DiskUsage {
        /// Directory to analyze
        #[arg(default_value = ".")]
        path: String,

        /// Number of top items to show
        #[arg(short, long, default_value = "20")]
        top: usize,

        /// Group by file extension
        #[arg(short, long, default_value = "false")]
        by_type: bool,

        /// Show hidden files
        #[arg(long, default_value = "false")]
        hidden: bool,

        /// Minimum size to display (e.g., "1MB", "500KB")
        #[arg(long)]
        min: Option<String>,

        /// Export to CSV
        #[arg(long)]
        csv: Option<String>,
    },

    /// Calculate file hash (SHA256, SHA512, MD5)
    #[command(name = "hash")]
    Hash {
        /// Files to hash
        files: Vec<String>,

        /// Hash algorithm (sha256, sha512, md5)
        #[arg(short, long, default_value = "sha256")]
        algorithm: String,

        /// Verify against expected hash
        #[arg(short, long)]
        verify: Option<String>,

        /// Output format (text, json)
        #[arg(short, long, default_value = "text")]
        format: String,
    },

    /// Compare two directories for differences
    #[command(name = "diff")]
    Compare {
        /// First directory
        dir1: String,

        /// Second directory
        dir2: String,

        /// Compare content (not just names)
        #[arg(short, long, default_value = "false")]
        content: bool,

        /// Show only differences
        #[arg(short, long, default_value = "false")]
        diff_only: bool,
    },

    /// Find empty files and directories
    #[command(name = "empty")]
    FindEmpty {
        /// Directory to scan
        #[arg(default_value = ".")]
        path: String,

        /// Find empty directories only
        #[arg(short, long, default_value = "false")]
        dirs: bool,

        /// Find empty files only
        #[arg(short, long, default_value = "false")]
        files: bool,

        /// Delete empty items
        #[arg(long, default_value = "false")]
        delete: bool,
    },

    /// List files with sorting and filtering
    #[command(name = "list")]
    List {
        /// Directory to list
        #[arg(default_value = ".")]
        path: String,

        /// Sort by (name, size, date, ext)
        #[arg(short, long, default_value = "name")]
        sort: String,

        /// Reverse sort order
        #[arg(short, long, default_value = "false")]
        reverse: bool,

        /// Recursive listing
        #[arg(short = 'R', long, default_value = "false")]
        recursive: bool,

        /// Show only files matching pattern
        #[arg(short, long)]
        pattern: Option<String>,

        /// Long format with details
        #[arg(short, long, default_value = "false")]
        long: bool,
    },

    /// Find files exceeding a size threshold
    #[command(name = "large")]
    FindLarge {
        /// Directory to scan
        #[arg(default_value = ".")]
        path: String,

        /// Minimum size (e.g., "100MB", "1GB")
        #[arg(short, long, default_value = "100MB")]
        size: String,

        /// Number of results
        #[arg(short, long, default_value = "50")]
        top: usize,
    },

    /// Find recently modified files
    #[command(name = "recent")]
    Recent {
        /// Directory to scan
        #[arg(default_value = ".")]
        path: String,

        /// Time range (e.g., "1h", "24h", "7d", "30d")
        #[arg(short, long, default_value = "24h")]
        within: String,

        /// Number of results
        #[arg(short, long, default_value = "50")]
        top: usize,
    },

    /// Display file statistics for a directory
    #[command(name = "stats")]
    Stats {
        /// Directory to analyze
        #[arg(default_value = ".")]
        path: String,

        /// Show hidden files
        #[arg(long, default_value = "false")]
        hidden: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::FindDuplicates {
            path,
            min_size,
            extensions,
            output,
            delete,
        } => commands::duplicates::run(&path, min_size, extensions, output, delete),

        Commands::Search {
            pattern,
            path,
            extensions,
            ignore_case,
            files_only,
            line_numbers,
            context,
        } => commands::search::run(
            &pattern,
            &path,
            extensions,
            ignore_case,
            files_only,
            line_numbers,
            context,
        ),

        Commands::BulkRename {
            path,
            find,
            replace,
            extensions,
            dry_run,
            recursive,
        } => commands::rename::run(&path, &find, &replace, extensions, dry_run, recursive),

        Commands::DiskUsage {
            path,
            top,
            by_type,
            hidden,
            min,
            csv,
        } => commands::disk::run(&path, top, by_type, hidden, min, csv),

        Commands::Hash {
            files,
            algorithm,
            verify,
            format,
        } => commands::hash::run(files, &algorithm, verify, &format),

        Commands::Compare {
            dir1,
            dir2,
            content,
            diff_only,
        } => commands::compare::run(&dir1, &dir2, content, diff_only),

        Commands::FindEmpty {
            path,
            dirs,
            files,
            delete,
        } => commands::empty::run(&path, dirs, files, delete),

        Commands::List {
            path,
            sort,
            reverse,
            recursive,
            pattern,
            long,
        } => commands::list::run(&path, &sort, reverse, recursive, pattern, long),

        Commands::FindLarge { path, size, top } => commands::large::run(&path, &size, top),

        Commands::Recent { path, within, top } => commands::recent::run(&path, &within, top),

        Commands::Stats { path, hidden } => commands::stats::run(&path, hidden),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}


