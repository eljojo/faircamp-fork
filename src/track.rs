use std::path::PathBuf;
use std::rc::Rc;

use crate::artist::Artist;

#[derive(Debug)]
pub struct Track {
    pub artists: Vec<Rc<Artist>>,
    pub length: u8,
    pub number: Option<u32>,
    pub source_file: PathBuf,
    pub title: String,
    pub uuid: String
}

impl Track {
    pub fn init(artists: Vec<Rc<Artist>>, number: Option<u32>, source_file: PathBuf, title: String, uuid: String) -> Track {
        Track {
            artists,
            length: 0,
            number,
            source_file,
            title,
            uuid
        }
    }
}