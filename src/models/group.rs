use serde::{Serialize, Deserialize};

use super::oid::Oid;

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Group {
	#[serde(rename(deserialize = "_id"))]
    pub id: Oid,
	pub title: String,
}

impl Group {
	pub fn new() -> Self {
		Self {
			id: Oid {
				value: String::new()
			},
			title: String::new()
		}
	}
}