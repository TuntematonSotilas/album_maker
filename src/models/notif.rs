#[derive(Clone, Copy)]
pub enum NotifType {
    Success,
    Error,
}

#[derive(Clone)]
pub struct Notif {
	pub message: String,
	pub notif_type: NotifType
}

impl Notif {
    pub fn new() -> Self {
        Self {
			message: String::new(),
			notif_type: NotifType::Success
		}
	}
}
