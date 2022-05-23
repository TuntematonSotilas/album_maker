use serde::{Deserialize, Serialize};

use super::group::Group;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub title: String,
    pub groups: Option<Vec<Group>>,
}

impl Album {
    pub const fn new() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            groups: Some(Vec::new()),
        }
    }
}
