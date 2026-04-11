use colored::*;

pub const BANNER: &str = r"
  FSESC - Fast SEcret SCanner
  System Architect Edition v1.0.0
";

pub fn print_banner() {
    println!("{}", BANNER.cyan().bold());
    println!("{}", "=".repeat(50).dimmed());
}

pub fn format_count(count: usize) -> ColoredString {
    if count == 0 {
        "0".green()
    } else {
        count.to_string().red().bold()
    }
}
