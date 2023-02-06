use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Default)]
pub enum TranspMode {
    #[default]
    Train,
    Plane,
    Car,
}

pub const TRANSP_MODE: [TranspMode; 3] = [TranspMode::Train, TranspMode::Plane, TranspMode::Car];

impl std::fmt::Display for TranspMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Trip {
    pub transp_mode: TranspMode,
    pub origin: String,
    pub destination: String,
}
