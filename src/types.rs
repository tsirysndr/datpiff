use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Mixtape {
    pub artist: String,
    pub title: String,
    pub listens: String,
    pub link: String,
}
