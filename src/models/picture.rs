use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Picture {
    pub asset_id: String,
    pub public_id: String,
    pub format: String,
    pub caption: Option<String>,
}
