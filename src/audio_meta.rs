use id3;
use metaflac;
use std::path::Path;

#[derive(Debug)]
pub struct AudioMeta {
    pub album: Option<String>,
    pub artist: Option<String>,
    pub duration_seconds: Option<u32>,
    pub title: Option<String>,
    pub track_number: Option<u32>
}

impl AudioMeta {
    pub fn extract(path: &Path, extension: &str) -> AudioMeta {
        if extension == "flac" {
            if let Ok(tag) = metaflac::Tag::read_from_path(path) {
                return AudioMeta {
                    album: tag.get_vorbis("album").map(|iter| iter.collect()),
                    artist: tag.get_vorbis("artist").map(|iter| iter.collect()),
                    duration_seconds: tag.get_streaminfo().map(|streaminfo|
                        (streaminfo.total_samples / streaminfo.sample_rate as u64) as u32
                    ),
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
                    album: tag.album().map(|str| str.to_string()),
                    artist: tag.artist().map(|str| str.to_string()),
                    duration_seconds: tag.duration(),
                    title: tag.title().map(|str| str.to_string()),
                    track_number: tag.track()
                };
            }
        }
        
        AudioMeta {
            album: None,
            artist: None,
            duration_seconds: None,
            title: None,
            track_number: None
        }
    }
}