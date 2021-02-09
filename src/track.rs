use std::path::PathBuf;
use std::rc::Rc;

use crate::artist::Artist;

const HOUR_SECONDS: u32 = 60 * 60;

#[derive(Debug)]
pub struct Track {
    pub artists: Vec<Rc<Artist>>,
    pub duration_seconds: Option<u32>,
    pub number: Option<u32>,
    pub source_file: PathBuf,
    pub title: String,
    pub uuid: String
}

impl Track {
    pub fn duration_formatted(&self) -> String {
        match self.duration_seconds {
            Some(seconds) => {
                if seconds > HOUR_SECONDS {
                    format!("{:02}:{:02}:{:02}", seconds / HOUR_SECONDS, (seconds % HOUR_SECONDS) / 60, seconds % 60)
                } else {
                    format!("{:02}:{:02}", (seconds % HOUR_SECONDS) / 60, seconds % 60)
                }
            },
            None => String::new()
        }
    }
    
    pub fn init(
        artists: Vec<Rc<Artist>>,
        duration_seconds: Option<u32>,
        number: Option<u32>,
        source_file: PathBuf,
        title: String,
        uuid: String
    ) -> Track {
        Track {
            artists,
            duration_seconds,
            number,
            source_file,
            title,
            uuid
        }
    }
}