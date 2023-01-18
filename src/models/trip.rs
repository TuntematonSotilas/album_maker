#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone, Default)]
pub enum TranspMode {
    #[default]
    Train,
    Plane,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Trip {
    pub transp_mode: TranspMode,
    pub origin: String,
    pub destination: i32,
}
