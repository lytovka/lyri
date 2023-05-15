use serde::{Serialize, Deserialize};

use super::song::Song;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hit {
    pub index: String,
    pub r#type: String,
    pub result: Song,
}