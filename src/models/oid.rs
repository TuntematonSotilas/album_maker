use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Oid {
    #[serde(rename(deserialize = "$oid"))]
    pub value: String,
}
