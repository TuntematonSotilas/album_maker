use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sharing {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub album_id: String,
    pub album_name: String,
    pub nb_like: u32,
    pub nb_view: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct AddViewLike {
    pub share_id: String,
    pub like: bool,
    pub view: bool,
}