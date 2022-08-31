use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Serialize, Deserialize, Debug, Tabled)]
pub struct Mixtape {
    pub id: String,
    pub artist: String,
    #[tabled(skip)]
    pub title: String,
    pub listens: String,
    #[tabled(skip)]
    pub link: String,
    #[tabled(skip)]
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
