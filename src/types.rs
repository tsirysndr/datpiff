use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mixtape {
    pub id: String,
    pub artist: String,
    pub title: String,
    pub listens: String,
    pub link: String,
    pub cover: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MixtapeDetails {
    pub tracks: Vec<String>,
    pub uploader: String,
    pub added_at: String,
    pub title: String,
    pub artist: String,
    pub listens: String,
    pub dj: String,
    pub cover: String,
    pub cover_back: String,
}
