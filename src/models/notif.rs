#[derive(Clone)]
pub struct Notif {
    pub message: String,
    pub notif_type: NotifType,
}

impl Notif {
    pub const fn new() -> Self {
        Self {
            message: String::new(),
            notif_type: NotifType::Success,
        }
    }
}

#[derive(Clone, Copy)]
pub enum NotifType {
    Success,
    Error,
}
