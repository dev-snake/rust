use colored::*;

/// Professional CLI UI module - No emojis, clean design
/// Inspired by: ripgrep, fd, exa, bat, tokei

// Box drawing characters and icons
pub mod chars {
    pub const H_LINE: &str = "─";
    pub const V_LINE: &str = "│";
    pub const TL_CORNER: &str = "┌";
    pub const TR_CORNER: &str = "┐";
    pub const BL_CORNER: &str = "└";
    pub const BR_CORNER: &str = "┘";
    pub const T_RIGHT: &str = "├";
    
    // Icons (Professional symbols - NO EMOJIS)
    pub const BULLET: &str = "•";
    pub const ARROW: &str = "➜";
    pub const CHECK: &str = "v";
    pub const CROSS_MARK: &str = "x";
    pub const DOT: &str = "·";
    pub const INFO: &str = "i";
    pub const WARNING: &str = "!";
}


/// Print a styled header box with a "vibrant" feel
pub fn print_header(title: &str) {
    let width = 60;
    let title_len = title.len();
    let padding_left = (width - title_len - 4) / 2;
    let padding_right = width - title_len - 4 - padding_left;
    
    println!();
    println!(
        "{}{}{}",
        chars::TL_CORNER.bright_cyan(),
        chars::H_LINE.repeat(width - 2).bright_cyan(),
        chars::TR_CORNER.bright_cyan()
    );
    
    print!("{}", chars::V_LINE.bright_cyan());
    print!("{}", " ".repeat(padding_left));
    print!("{}", title.bright_white().bold().on_bright_blue());
    print!("{}", " ".repeat(padding_right));
    println!("{}", chars::V_LINE.bright_cyan());

    println!(
        "{}{}{}",
        chars::BL_CORNER.bright_cyan(),
        chars::H_LINE.repeat(width - 2).bright_cyan(),
        chars::BR_CORNER.bright_cyan()
    );
}

/// Print a section divider with optional title and icon
pub fn print_section(title: &str) {
    println!();
    println!(
        "{} {} {}",
        chars::H_LINE.repeat(3).bright_black(),
        title.bright_yellow().bold(),
        chars::H_LINE.repeat(45 - title.len()).bright_black()
    );
}

/// Print a simple horizontal line
pub fn print_line(width: usize) {
    println!("{}", chars::H_LINE.repeat(width).dimmed());
}


/// Print operation start message
pub fn print_start(operation: &str, target: &str) {
    println!(
        "{} {} {}",
        chars::ARROW.bright_cyan(),
        operation.bright_white(),
        target.bright_yellow()
    );
}

/// Print success message
pub fn print_success(message: &str) {
    println!(
        "{} {}",
        chars::CHECK.bright_green().bold(),
        message.bright_green()
    );
}

/// Print error message
pub fn print_error(message: &str) {
    println!(
        "{} {}",
        chars::CROSS_MARK.bright_red().bold(),
        message.bright_red()
    );
}

/// Print warning message
pub fn print_warning(message: &str) {
    println!(
        "{} {}",
        chars::WARNING.bright_yellow().bold(),
        message.bright_yellow()
    );
}

/// Print info message
pub fn print_info(message: &str) {
    println!(
        "{} {}",
        chars::INFO.bright_blue().bold(),
        message.bright_white()
    );
}

/// Print a key-value pair
pub fn print_kv(key: &str, value: &str) {
    println!(
        "  {:.<24} {}",
        format!("{} ", key).bright_black(),
        value.bright_white()
    );
}

/// Print a key-value pair with colored value
pub fn print_kv_colored(key: &str, value: ColoredString) {
    println!(
        "  {:.<24} {}",
        format!("{} ", key).bright_black(),
        value
    );
}

/// Create a progress bar string
pub fn progress_bar(percentage: f64, width: usize) -> String {
    let filled = ((percentage / 100.0) * width as f64) as usize;
    let empty = width.saturating_sub(filled);
    
    let filled_part = if filled > 0 {
        "█".repeat(filled).bright_cyan()
    } else {
        "".normal()
    };
    
    let empty_part = if empty > 0 {
        "░".repeat(empty).bright_black()
    } else {
        "".normal()
    };

    format!(
        "{}{}{}{}",
        chars::V_LINE.bright_black(),
        filled_part,
        empty_part,
        chars::V_LINE.bright_black()
    )
}



/// Print a result count
pub fn print_count(count: usize, singular: &str, plural: &str) {
    let word = if count == 1 { singular } else { plural };
    println!(
        "\n{} {} {}",
        chars::ARROW.bright_black(),
        count.to_string().bright_green().bold(),
        word.bright_black()
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
