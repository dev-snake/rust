use colored::*;

/// Professional CLI UI module - No emojis, clean design
/// Inspired by: ripgrep, fd, exa, bat, tokei

pub const VERSION: &str = "1.0.0";

// Box drawing characters
pub mod chars {
    pub const H_LINE: &str = "─";
    pub const V_LINE: &str = "│";
    pub const TL_CORNER: &str = "┌";
    pub const TR_CORNER: &str = "┐";
    pub const BL_CORNER: &str = "└";
    pub const BR_CORNER: &str = "┘";
    pub const T_DOWN: &str = "┬";
    pub const T_UP: &str = "┴";
    pub const T_RIGHT: &str = "├";
    pub const T_LEFT: &str = "┤";
    pub const CROSS: &str = "┼";
    pub const BULLET: &str = "•";
    pub const ARROW: &str = "→";
    pub const CHECK: &str = "✓";
    pub const CROSS_MARK: &str = "✗";
    pub const DOT: &str = "·";
}

pub struct Theme;

impl Theme {
    // Header style - cyan bold for titles
    pub fn header(text: &str) -> ColoredString {
        text.cyan().bold()
    }

    // Section title - yellow bold
    pub fn section(text: &str) -> ColoredString {
        text.yellow().bold()
    }

    // Success/positive - green
    pub fn success(text: &str) -> ColoredString {
        text.green()
    }

    // Warning - yellow
    pub fn warning(text: &str) -> ColoredString {
        text.yellow()
    }

    // Error/danger - red
    pub fn error(text: &str) -> ColoredString {
        text.red()
    }

    // Muted/secondary - dimmed
    pub fn muted(text: &str) -> ColoredString {
        text.dimmed()
    }

    // Accent - magenta
    pub fn accent(text: &str) -> ColoredString {
        text.magenta()
    }

    // Path - blue
    pub fn path(text: &str) -> ColoredString {
        text.blue()
    }

    // Number/value - white bold
    pub fn value(text: &str) -> ColoredString {
        text.white().bold()
    }
}

/// Print a styled header box
pub fn print_header(title: &str) {
    let width = 60;
    let padding = (width - title.len() - 2) / 2;
    
    println!();
    println!(
        "{}{}{}",
        chars::TL_CORNER.cyan(),
        chars::H_LINE.repeat(width - 2).cyan(),
        chars::TR_CORNER.cyan()
    );
    println!(
        "{}{:^width$}{}",
        chars::V_LINE.cyan(),
        title.cyan().bold(),
        chars::V_LINE.cyan(),
        width = width - 2
    );
    println!(
        "{}{}{}",
        chars::BL_CORNER.cyan(),
        chars::H_LINE.repeat(width - 2).cyan(),
        chars::BR_CORNER.cyan()
    );
}

/// Print a section divider with optional title
pub fn print_section(title: &str) {
    println!();
    println!(
        "{} {} {}",
        chars::H_LINE.repeat(2).dimmed(),
        title.yellow().bold(),
        chars::H_LINE.repeat(50 - title.len()).dimmed()
    );
}

/// Print a simple horizontal line
pub fn print_line(width: usize) {
    println!("{}", chars::H_LINE.repeat(width).dimmed());
}

/// Print a table header row
pub fn print_table_header(columns: &[(&str, usize)]) {
    let mut header = String::new();
    let mut separator = String::new();

    for (name, width) in columns {
        header.push_str(&format!("{:width$}  ", name.cyan().bold(), width = width));
        separator.push_str(&chars::H_LINE.repeat(*width));
        separator.push_str("  ");
    }

    println!("{}", header);
    println!("{}", separator.dimmed());
}

/// Print a status message
pub fn print_status(label: &str, value: &str) {
    println!(
        "  {} {}",
        format!("{}:", label).dimmed(),
        value
    );
}

/// Print operation start message
pub fn print_start(operation: &str, target: &str) {
    println!(
        "{} {} {}",
        "[*]".cyan().bold(),
        operation,
        target.yellow()
    );
}

/// Print success message
pub fn print_success(message: &str) {
    println!(
        "{} {}",
        format!("[{}]", chars::CHECK).green().bold(),
        message.green()
    );
}

/// Print error message
pub fn print_error(message: &str) {
    println!(
        "{} {}",
        format!("[{}]", chars::CROSS_MARK).red().bold(),
        message.red()
    );
}

/// Print warning message
pub fn print_warning(message: &str) {
    println!(
        "{} {}",
        "[!]".yellow().bold(),
        message.yellow()
    );
}

/// Print info message
pub fn print_info(message: &str) {
    println!(
        "{} {}",
        "[i]".blue().bold(),
        message
    );
}

/// Print a key-value pair
pub fn print_kv(key: &str, value: &str) {
    println!(
        "  {:.<24} {}",
        format!("{} ", key).dimmed(),
        value.white()
    );
}

/// Print a key-value pair with colored value
pub fn print_kv_colored(key: &str, value: ColoredString) {
    println!(
        "  {:.<24} {}",
        format!("{} ", key).dimmed(),
        value
    );
}

/// Create a progress bar string
pub fn progress_bar(percentage: f64, width: usize) -> String {
    let filled = ((percentage / 100.0) * width as f64) as usize;
    let empty = width.saturating_sub(filled);
    
    format!(
        "{}{}{}{}",
        chars::V_LINE.dimmed(),
        "█".repeat(filled).cyan(),
        "░".repeat(empty).dimmed(),
        chars::V_LINE.dimmed()
    )
}

/// Format a file listing item
pub fn format_file_entry(is_dir: bool, name: &str, size: Option<&str>) -> String {
    let prefix = if is_dir {
        chars::T_RIGHT.blue().to_string()
    } else {
        " ".to_string()
    };

    let name_fmt = if is_dir {
        format!("{}/", name).blue().bold().to_string()
    } else {
        name.to_string()
    };

    match size {
        Some(s) => format!("{} {:50} {}", prefix, name_fmt, s.dimmed()),
        None => format!("{} {}", prefix, name_fmt),
    }
}

/// Print summary stats in a compact format
pub fn print_summary(items: &[(&str, String)]) {
    println!();
    print_line(60);
    for (label, value) in items {
        print!(
            "  {}: {}  ",
            label.dimmed(),
            value.green().bold()
        );
    }
    println!();
}

/// Print a result count
pub fn print_count(count: usize, singular: &str, plural: &str) {
    let word = if count == 1 { singular } else { plural };
    println!(
        "\n{} {} {}",
        chars::ARROW.dimmed(),
        count.to_string().green().bold(),
        word.dimmed()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_bar() {
        let bar = progress_bar(50.0, 10);
        assert!(bar.contains("█"));
        assert!(bar.contains("░"));
    }
}
