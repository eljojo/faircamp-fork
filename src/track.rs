use std::path::PathBuf;
use std::rc::Rc;

use crate::artist::Artist;

#[derive(Debug)]
pub struct Track {
    pub artist: Rc<Artist>,
    pub length: u8,
    pub source_file: PathBuf,
    pub title: String,
    pub uuid: String
}

impl Track {
    pub fn init(artist: Rc<Artist>, source_file: PathBuf, title: String, uuid: String) -> Track {
        Track {
            artist,
            length: 0,
            source_file,
            title,
            uuid
        }
    }
}