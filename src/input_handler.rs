use std::fs::{File, OpenOptions};
use std::io::{self, Write, Read, BufReader, BufRead};
use std::path::Path;
use termios::{Termios, tcsetattr, ICANON, ECHO, TCSANOW, VMIN, VTIME};
use std::os::unix::io::AsRawFd;

const LOG_PATH: &str = "/tmp/archtry-log.txt";

/// Initializes terminal in raw mode
pub fn init_raw_mode() -> io::Result<Termios> {
    let stdin = io::stdin();
    let fd = stdin.as_raw_fd();
    let original = Termios::from_fd(fd)?;
    let mut new_termios = original;
    
    // Disable canonical mode and echo
    new_termios.c_lflag &= !(ICANON | ECHO);
    new_termios.c_cc[VMIN] = 1;  // Minimum chars to read
    new_termios.c_cc[VTIME] = 0; // Timeout in deciseconds
    tcsetattr(fd, TCSANOW, &new_termios)?;
    
    Ok(original)
}

/// Restores terminal to original settings
pub fn restore_terminal(original: &Termios) -> io::Result<()> {
    let stdin = io::stdin();
    tcsetattr(stdin.as_raw_fd(), TCSANOW, original)?;
    Ok(())
}

/// Reads command history from log file
fn read_history() -> io::Result<Vec<String>> {
    if Path::new(LOG_PATH).exists() {
        let file = File::open(LOG_PATH)?;
        let reader = BufReader::new(file);
        Ok(reader.lines().map_while(Result::ok).collect())
    } else {
        Ok(Vec::new())
    }
}

/// Logs command to history file
pub fn log_command(command: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(LOG_PATH)?;
    
    writeln!(file, "{}", command)
}

/// Computes the visible length of a string (ignoring ANSI escape sequences)
fn visible_length(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1B' {
            in_escape = true;
        }
        if !in_escape {
            len += 1;
        }
        if in_escape && c == 'm' {
            in_escape = false;
        }
    }
    len
}

/// Redraws current line with cursor at correct position
fn redraw_line(input: &str, cursor_pos: usize, prompt: &str) -> io::Result<()> {
    // Clear line and redraw prompt and input
    print!("\r\x1B[K{}{}", prompt, input);
    
    // Move cursor to correct position using visible length
    let visible_prompt_len = visible_length(prompt);
    let total_pos = visible_prompt_len + cursor_pos;
    print!("\r\x1B[{}C", total_pos);
    
    io::stdout().flush()
}

/// Reads user input with history support
pub fn read_input_with_history(prompt: &str) -> io::Result<String> {
    // Save and configure terminal settings
    let original_term = init_raw_mode()?;
    
    // Load command history
    let history = read_history()?;
    let mut history_index = history.len(); // Start at the end
    let mut input = String::new();
    let mut cursor_pos = 0;
    let mut current_input_before_history = String::new(); // To save input before history navigation

    print!("{}", prompt);
    io::stdout().flush()?;

    let mut stdin = io::stdin();
    let mut buf = [0; 1];

    loop {
        if stdin.read(&mut buf)? == 0 {
            break;
        }

        match buf[0] {
            // Enter key
            10 | 13 => {
                if !input.is_empty() {
                    log_command(&input)?;
                }
                println!();
                break;
            },
            // Backspace
            127 => {
                if cursor_pos > 0 {
                    cursor_pos -= 1;
                    input.remove(cursor_pos);
                    redraw_line(&input, cursor_pos, prompt)?;
                }
            },
            // Escape sequence (arrow keys)
            27 => {
                let mut seq = [0; 2];
                if stdin.read(&mut seq)? != 2 {
                    continue;
                }

                if seq[0] == 91 { // '[' after ESC
                    match seq[1] {
                        // Up arrow
                        65 => {
                            if !history.is_empty() {
                                if history_index == history.len() {
                                    // Save current input before history navigation
                                    current_input_before_history = input.clone();
                                    history_index = history_index.saturating_sub(1);
                                    input = history[history_index].clone();
                                } else if history_index > 0 {
                                    history_index -= 1;
                                    input = history[history_index].clone();
                                }
                                cursor_pos = input.len();
                                redraw_line(&input, cursor_pos, prompt)?;
                            }
                        },
                        // Down arrow
                        66 => {
                            if !history.is_empty() {
                                if history_index < history.len().saturating_sub(1) {
                                    history_index += 1;
                                    input = history[history_index].clone();
                                    cursor_pos = input.len();
                                    redraw_line(&input, cursor_pos, prompt)?;
                                } else if history_index == history.len().saturating_sub(1) {
                                    // Restore saved input when returning to bottom
                                    history_index = history.len();
                                    input = current_input_before_history.clone();
                                    cursor_pos = input.len();
                                    redraw_line(&input, cursor_pos, prompt)?;
                                }
                            }
                        },
                        // Right arrow
                        67 => {
                            if cursor_pos < input.len() {
                                cursor_pos += 1;
                                redraw_line(&input, cursor_pos, prompt)?;
                            }
                        },
                        // Left arrow
                        68 => {
                            if cursor_pos > 0 {
                                cursor_pos = cursor_pos.saturating_sub(1);
                                redraw_line(&input, cursor_pos, prompt)?;
                            }
                        },
                        _ => {}
                    }
                }
            },
            // Printable ASCII characters (32-126)
            c if (32..=126).contains(&c) => {
                input.insert(cursor_pos, c as char);
                cursor_pos += 1;
                redraw_line(&input, cursor_pos, prompt)?;
            },
            _ => {}
        }
    }

    // Restore terminal settings
    restore_terminal(&original_term)?;
    Ok(input)
}
