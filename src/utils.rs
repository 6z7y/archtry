use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::{thread, time::Duration};

/// Pauses execution for specified seconds
pub fn sleep(secs: u64) {
    thread::sleep(Duration::from_secs(secs));
}

/// Clears terminal screen
pub fn clear_screen() {
    print!("{}[2J", 27 as char);
}

/// Creates progress bar with custom style
pub fn create_progress_bar(length: u64) -> ProgressBar {
    let pb = ProgressBar::new(length);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({percent}%) {msg}")
            .unwrap()
            .progress_chars("=>-"),
    );
    pb
}

/// Displays colored header message
pub fn show_header(message: &str) {
    println!("\n{}", message.bright_blue());
    sleep(1);
}

/// Displays colored success message
pub fn show_success(message: &str) {
    println!("\n{}\n", message.bright_green());
    sleep(1);
}

/// Displays colored warning message
pub fn show_warning(message: &str) {
    println!("{}", message.bright_yellow());
}
