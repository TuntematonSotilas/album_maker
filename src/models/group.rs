use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{picture::Picture, state::State};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: Uuid,
    pub title: String,
    pub pictures: Option<Vec<Picture>>,
    pub cover: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub count_fake_pictures: u32,
    #[serde(skip_serializing, skip_deserializing)]
    pub state: Option<State>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            pictures: Some(Vec::new()),
            cover: String::new(),
            count_fake_pictures: 0,
            state: None,
        }
    }
}
