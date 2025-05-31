mod config;
mod models;
mod simulator;
mod utils;
mod input_handler;

use std::path::Path;
use colored::Colorize;
use crate::utils::{clear_screen, sleep}; // تم إزالة الواردات غير المستخدمة

const ASCII_LOGO: &str = r#"
     ____  ____  ____  _     _____  ____ ___  _
    /  _ \/  __\/   _\/ \ /|/__ __\/  __\\  \//
    | / \||  \/||  /  | |_||  / \  |  \/| \  / 
    | |-|||    /|  \_ | | ||  | |  |    / / /  
    \_/ \|\_/\_\\____/\_/ \|  \_/  \_/\_\/_/   
"#;

fn main() {
    clear_screen();
    println!("{}", ASCII_LOGO.bright_green());
    println!("{}", "Loading ArchTry...".bright_blue());
    sleep(3);
    
    // Create log file only if it doesn't exist
    if !Path::new("/tmp/archtry-log.txt").exists() {
        let _ = std::fs::File::create("/tmp/archtry-log.txt");
    }
    
    // Get user choices and run simulation
    let user_choices = config::get_user_choices();
    simulator::run_simulation(&user_choices);
    
    // Final message
    println!("\n{}", "Thank you for using ArchTry!".bright_green());
    println!("{}", "Learn more: https://wiki.archlinux.org/title/Installation_guide".bright_blue());
}
