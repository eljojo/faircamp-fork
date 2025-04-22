// SPDX-FileCopyrightText: 2021-2025 Simon Repp
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::ffi::OsString;
use std::fmt::{Display, Formatter};
use std::fs;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::Path;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use nanoid::nanoid;

const BYTES_KB: u64 = 1024; 
const BYTES_MB: u64 = 1024 * BYTES_KB; 
const BYTES_GB: u64 = 1024 * BYTES_MB; 
const SECONDS_HOUR: u32 = 60 * 60;

/// We need some float values (e.g. in our themes) to be automatically
/// hashable (which f32 isn't), so we use the newtype pattern to ensure this
/// property.
#[derive(Clone, Debug)]
pub struct HashableF32(pub f32);

impl Display for HashableF32 {
    fn fmt(&self, formatter: &mut Formatter) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl Hash for HashableF32 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
    }
}

/// Takes an existing filename (that has elsewhere been identified to collide
/// with another file with the same filename) and returns the filename,
/// systematically altered as to resolve the existing collision and ideally
/// avoid further ones.
///
/// Some examples:
/// - "foo"            -> "foo(1)"
/// - "foo.bar"        -> "foo(1).bar"
/// - "foo.bar.baz"    -> "foo(1).bar.baz"
/// - "foo(1)"         -> "foo(2)"
/// - "foo(1).bar"     -> "foo(2).bar"
/// - "foo(1).bar.baz" -> "foo(2).bar.baz"
pub fn deduplicate_filename(filename: &str) -> String {
    fn deduplicate_filestem(filestem: &str) -> String {
        if filestem.ends_with(')') {
            if let Some(index) = filestem.rfind('(') {
                if let Ok(number) = &filestem[(index + 1)..(filestem.len() - 1)].parse::<usize>() {
                    return format!("{}({})", &filestem[..index], number + 1);
                }
            }
        }

        return format!("{filestem}(1)");
    }

    match filename.split_once('.') {
        Some((filestem, extension)) => deduplicate_filestem(filestem) + "." + extension,
        None => deduplicate_filestem(filename)
    }
}

pub fn ensure_dir_all(dir: &Path) {
    fs::create_dir_all(dir).unwrap();
}

pub fn ensure_dir_all_and_write_index(dir: &Path, html: &str) {
    ensure_dir_all(dir);
    fs::write(dir.join("index.html"), html).unwrap();
}

pub fn ensure_empty_dir(dir: &Path) {
    let _ = fs::remove_dir_all(dir);
    let _ = fs::create_dir_all(dir);
}

/// Takes a number of bytes and adaptively formats them as [n]B, [n]KB, [n]MB or [n]GB
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

pub fn generic_hash(hashable: &impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    hashable.hash(&mut hasher);
    hasher.finish()
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

/// Given e.g. "me&you", it will first turn the input into "me&amp;you", then
/// into "me&amp;amp;you". When this is rendered for rss feed readers, the
/// first layer of escaping enables nesting of html markup inside xml, and
/// the second layer of escaping differentiates html-reserved characters to
/// not be interpreted as html characters (but rather just displayed
/// verbatim). Used to render various rss feed content.
pub fn html_double_escape_outside_attribute(string: &str) -> String {
    html_escape_outside_attribute(string)
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

/// Efficient, reusable implementation of the annoying OsString to String conversion
pub fn string_from_os(os_string: OsString) -> String {
    match os_string.into_string() {
        Ok(string) => string,
        Err(os_string) => os_string.to_string_lossy().to_string()
    }
}

pub fn uid() -> String {
    nanoid!(8)
}

/// Convert a numerical hash to a more compact base64 string representation
/// that is safe to use in file names and urls.
pub fn url_safe_base64(hash: u64) -> String {
    URL_SAFE_NO_PAD.encode(hash.to_le_bytes())
}

pub fn url_safe_hash_base64(hashable: &impl Hash) -> String {
    let mut hasher = DefaultHasher::new();
    hashable.hash(&mut hasher);
    url_safe_base64(hasher.finish())
}
