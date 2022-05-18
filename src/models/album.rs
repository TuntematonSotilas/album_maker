use serde::{Deserialize, Serialize};

use super::{group::Group, oid::Oid};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    #[serde(rename(deserialize = "_id"))]
    pub id: Oid,
    pub title: String,
    pub groups: Option<Vec<Group>>,
}

impl Album {
    pub const fn new() -> Self {
        Self {
            id: Oid {
                value: String::new(),
            },
            title: String::new(),
            groups: Some(Vec::new()),
        }
    }
}
