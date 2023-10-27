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
    } else if size >= BYTES_KB {
        format!("{}KB", size / BYTES_KB) // e.g. "3KB", "267KB", "510KB"
    } else {
        format!("{}B", size) // e.g. "367B"
    }
}

/// Takes `seconds` and adaptively formats them as `M:SS`, or `H:MM:SS` if
/// longer than one hour.
pub fn format_time(seconds: f32) -> String {
    let seconds_u32 = seconds as u32;

    if seconds_u32 > SECONDS_HOUR {
        let hour = seconds_u32 / SECONDS_HOUR;
        let minute = (seconds_u32 % SECONDS_HOUR) / 60;
        let second = seconds_u32 % 60;
        format!("{hour}:{minute:02}:{second:02}")
    } else {
        let minute = seconds_u32 / 60;
        let second = seconds_u32 % 60;
        format!("{minute}:{second:02}")
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

/// Given e.g. "\"foo\"", it will first turn the input into "&quot;foo&quot;", 
/// then into "&amp;quot;foo&amp;quot;". When this is rendered in the browser,
/// the second escaping pass is removed again, i.e. people will see this on
/// the site: "&quot;foo&quot;". Used to render embeddable code snippets.
pub fn html_double_escape_inside_attribute(string: &str) -> String {
    html_escape_inside_attribute(string)
        .replace('&', "&amp;")
}

/// Escape e.g. "me&you" so it can be rendered into an attribute,
/// e.g. as <img alt="me&quot;you" src="...">
pub fn html_escape_inside_attribute(string: &str) -> String {
    string.replace('&', "&amp;")
          .replace('<', "&lt;")
          .replace('>', "&gt;")
          .replace('"', "&quot;")
          .replace('\'', "&#39;")
}

/// Escape e.g. "love>hate" so it can be rendered into the page,
/// e.g. as <span>love&gt;hate</span>
pub fn html_escape_outside_attribute(string: &str) -> String {
    string.replace('&', "&amp;")
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