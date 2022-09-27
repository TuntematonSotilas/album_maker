use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::picture::Picture;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: Uuid,
    pub title: String,
    pub pictures: Option<Vec<Picture>>,
    #[serde(skip_serializing, skip_deserializing)]
    pub count_fake_pictures: u32,
}

impl Group {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
            pictures: Some(Vec::new()),
            count_fake_pictures: 0,
        }
    }
}
