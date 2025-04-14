use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: u32, // in seconds
    pub file_format: String,
    pub source: String,
    pub after: Vec<Narration>,
    pub before: Vec<Narration>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Narration {
    pub title: String,
    pub duration: u32, // in seconds
    pub file_format: String,
    pub source: String,
}

impl Track {
    pub fn new(title: String, artist: String, album: String, duration: u32, file_format: String, source: String, after: Vec<Narration>, before:Vec<Narration>) -> Track {
        Track {
            title,
            artist,
            album,
            duration,
            file_format,
            source,
            after,
            before
        }
    }
}