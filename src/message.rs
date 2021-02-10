// TODO: These would probably be more elegant as macros?

const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

pub fn discouraged(text: &str) {
    println!("{}[DISCOURAGED] {}{}", RED, text, RESET)
}

pub fn transcoding(text: &str) {
    println!("{}[TRANSCODING] {}{}", BLUE, text, RESET)
}