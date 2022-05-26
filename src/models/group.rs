use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::picture::Picture;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: Uuid,
    pub title: String,
	pictures: Option<Vec<Picture>>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
			pictures: Some(Vec::new()),
        }
    }
}
