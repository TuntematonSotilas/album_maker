use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Picture {
    pub asset_id: String,
    pub url: String,
}
