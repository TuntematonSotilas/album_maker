#[derive(serde::Deserialize, Debug, Clone)]
pub struct Album {
	#[serde(rename(deserialize = "_id"))]
    pub id: Oid,
	pub title: String,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct Oid {
    #[serde(rename(deserialize = "$oid"))]
    pub value: String,
}
