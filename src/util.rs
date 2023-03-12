use nanoid::nanoid;
use pulldown_cmark::{html, Parser};
use std::{fs, io, path::Path};

const BYTES_KB: u64 = 1024; 
const BYTES_MB: u64 = 1024 * BYTES_KB; 
const BYTES_GB: u64 = 1024 * BYTES_MB; 
const SECONDS_HOUR: u32 = 60 * 60;

pub fn ensure_dir(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
}

pub fn ensure_dir_and_write_index(dir: &Path, html: &str) {
    ensure_dir(dir);
    fs::write(dir.join("index.html"), html).unwrap();
}

pub fn ensure_empty_dir(dir: &Path) {
    remove_dir(dir);
    fs::create_dir_all(dir).unwrap();
}

/// Takes a number of bytes and adaptively formats them as [n]KB, [n]MB or [n]GB
pub fn format_bytes(size: u64) -> String {
    if size >= 512 * BYTES_MB {
        format!("{:.1}GB", size as f64 / BYTES_GB as f64) // e.g. "0.5GB", "1.3GB", "13.8GB"
    } else if size >= 100 * BYTES_MB {
        format!("{}MB", size / BYTES_MB) // e.g. "64MB", "267MB", "510MB"
    } else if size >= 512 * BYTES_KB {
        format!("{:.1}MB", size as f64 / BYTES_MB as f64) // e.g. "0.5MB", "1.3MB", "62.4MB"
    } else {
        format!("{}KB", size / BYTES_KB) // e.g. "3KB", "267KB", "510KB"
    }
}

/// Takes `seconds` and adaptively formats them as `MM:SS`, or `HH:MM:SS` if longer than one hour
pub fn format_time(seconds: u32) -> String {
    if seconds > SECONDS_HOUR {
        format!("{}:{}:{:02}", seconds / SECONDS_HOUR, (seconds % SECONDS_HOUR) / 60, seconds % 60)
    } else {
        format!("{}:{:02}", (seconds % SECONDS_HOUR) / 60, seconds % 60)
    }
}

/// Media that require heavy precomputation (image, audio) are stored in the
/// cache directory, and then copied to the build directory during
/// generation. In order to prevent double space usage, inside the build
/// directory we try to create hard links instead of copies. If hard links
/// can not be created (e.g. because cache and build directory are on
/// different file systems) we just silently fall back to regular copying.
pub fn hard_link_or_copy<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) {
    fs::hard_link(&from, &to)
        .unwrap_or_else(|_| {
            fs::copy(&from, &to).unwrap();
        });
}

pub fn html_escape_inside_attribute(string: &str) -> String {
    string.replace('&', "&amp")
          .replace('<', "&lt;")
          .replace('>', "&gt;")
          .replace('"', "&quot;")
          .replace('\'', "&#39;")
}

pub fn html_escape_outside_attribute(string: &str) -> String {
    string.replace('&', "&amp")
          .replace('<', "&lt;")
          .replace('>', "&gt;")
}

pub fn markdown_to_html(markdown: &str) -> String {
    let parser = Parser::new(markdown);
    
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    
    html_output
}

pub fn remove_dir(dir: &Path) {
    match fs::remove_dir_all(dir) {
        Ok(()) => (),
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => (), // just what we want anyway \o/
        Err(err) => panic!("{}", err)
    };
}

pub fn remove_file(path: &Path) {
    match fs::remove_file(path) {
        Ok(()) => (),
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => (), // just what we want anyway \o/
        Err(err) => panic!("{}", err)
    }
}

pub fn uid() -> String {
    nanoid!(8)
}