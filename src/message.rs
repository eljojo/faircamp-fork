// TODO: These would probably be more elegant as macros?

const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";

const RESET: &str = "\x1b[0m";

pub fn cache(text: &str) {
    println!("{}[CACHE] {}{}", MAGENTA, text, RESET)
}

pub fn discouraged(text: &str) {
    println!("{}[DISCOURAGED] {}{}", YELLOW, text, RESET)
}

pub fn error(text: &str) {
    println!("{}[ERROR] {}{}", RED, text, RESET)
}

pub fn stats(text: &str) {
    println!("{}[STATS] {}{}", CYAN, text, RESET)
}

pub fn transcoding(text: &str) {
    println!("{}[TRANSCODING] {}{}", BLUE, text, RESET)
}