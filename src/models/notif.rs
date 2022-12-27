#[derive(Clone)]
pub struct Notif {
    pub message: String,
    pub notif_type: TypeNotifs,
}

impl Notif {
    pub const fn new() -> Self {
        Self {
            message: String::new(),
            notif_type: TypeNotifs::Success,
        }
    }
}

#[derive(Clone, Copy)]
pub enum TypeNotifs {
    Success,
    Error,
    Share,
}
