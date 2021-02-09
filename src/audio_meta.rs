use id3;
use metaflac;
use std::path::Path;

#[derive(Debug)]
pub struct AudioMeta {
    pub artist: Option<String>,
    pub title: Option<String>,
    pub track_number: Option<u32>
}

impl AudioMeta {
    pub fn extract(path: &Path, extension: &str) -> AudioMeta {
        if extension == "flac" {
            if let Ok(tag) = metaflac::Tag::read_from_path(path) {
                return AudioMeta {
                    artist: tag.get_vorbis("artist").map(|iter| iter.collect()),
                    title: tag.get_vorbis("title").map(|iter| iter.collect()),
                    track_number: tag.get_vorbis("track") // TODO: Unconfirmed if that key is correct/available ("guessed it" for now :))
                        .map(|iter| iter.collect())
                        .filter(|str: &String| str.parse::<u32>().is_ok())
                        .map(|str: String| str.parse::<u32>().unwrap()) 
                };
            }
        } else if extension == "mp3" {
            if let Ok(tag) = id3::Tag::read_from_path(path) {
                return AudioMeta {
                    artist: tag.artist().map(|str| str.to_string()),
                    title: tag.title().map(|str| str.to_string()),
                    track_number: tag.track()
                };
            }
        }
        
        AudioMeta {
            artist: None,
            title: None,
            track_number: None
        }
    }
}