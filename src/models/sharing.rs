use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sharing {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub album_id: String,
    pub album_name: String,
    pub token: String,
    pub nb_views: String,
    pub locations: Vec<String>,
}