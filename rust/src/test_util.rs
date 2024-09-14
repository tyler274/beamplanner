use std::process::exit;

pub const BOLD: &str = "\u{001b}[1m";
pub const GRAY: &str = "\u{001b}[38;5;248m";
pub const CYAN: &str = "\u{001b}[36m";
pub const RED: &str = "\u{001b}[31m";
pub const GREEN: &str = "\u{001b}[32m";
pub const YELLOW: &str = "\u{001b}[33m";
pub const RESET: &str = "\u{001b}[0m";

pub fn fail(message: &str) {
    println!("{}{}{}{}{}", RED, BOLD, "FAIL: ", RESET, message);
    exit(1);
}

pub fn check(condition: bool, message: &str) {
    if !condition {
        fail(message);
    }
}
