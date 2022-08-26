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
