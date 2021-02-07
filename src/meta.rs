use id3;
use metaflac;
use std::path::Path;

pub struct Meta {
    pub artist: Option<String>,
    pub title: Option<String>
}

impl Meta {
    pub fn extract(extension: &str, path: &Path) -> Meta {
        if extension == "flac" {
            if let Ok(tag) = metaflac::Tag::read_from_path(path) {
                return Meta {
                    artist: tag.get_vorbis("artist").map(|iter| iter.collect()),
                    title: tag.get_vorbis("title").map(|iter| iter.collect())
                };
            }
        } else if extension == "mp3" {
            if let Ok(tag) = id3::Tag::read_from_path(path) {
                return Meta {
                    artist: tag.artist().map(|str| str.to_string()),
                    title: tag.title().map(|str| str.to_string())
                };
            }
        }
        
        Meta {
            artist: None,
            title: None
        }
    }
}