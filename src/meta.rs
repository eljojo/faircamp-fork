use id3;
use metaflac;
use std::path::Path;

pub fn extract_title(extension: &str, path: &Path) -> Option<String> {
    match extension {
        "flac" => match metaflac::Tag::read_from_path(path) {
            Ok(tag) => tag.get_vorbis("title").map(|iter| iter.collect()),
            Err(_) => None
        }
        "mp3" => match id3::Tag::read_from_path(path) {
            Ok(tag) => tag.title().map(|str| str.to_string()),
            Err(_) => None
        },
        _ => None
    }
}