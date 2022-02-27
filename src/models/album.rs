use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
	#[serde(rename(deserialize = "_id"))]
    pub id: Oid,
	pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Oid {
    #[serde(rename(deserialize = "$oid"))]
    pub value: String,
}

impl Album {
	pub fn new() -> Self {
		Self {
			id: Oid {
				value: String::new()
			},
			title: String::new()
		}
	}
}