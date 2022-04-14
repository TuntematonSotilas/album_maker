use serde::{Serialize, Deserialize};

use super::{oid::Oid, group::Group};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
	#[serde(rename(deserialize = "_id"))]
    pub id: Oid,
	pub title: String,
	pub groups: Option<Vec<Group>>,
}

impl Album {
	pub fn new() -> Self {
		Self {
			id: Oid {
				value: String::new()
			},
			title: String::new(),
			groups: Some(Vec::new()),
		}
	}
}