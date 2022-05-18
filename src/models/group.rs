use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: Uuid,
    pub title: String,
}

impl Group {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: String::new(),
        }
    }
}
