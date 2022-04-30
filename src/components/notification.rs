use seed::{prelude::*, *};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
	pub is_visible: bool,
	pub notif_type: NotifType,
	pub message: String,
}

#[derive(Clone, Copy)]
pub enum NotifType {
	Success,
	Error,
}

impl Default for NotifType {
    fn default() -> Self { NotifType::Success }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	Show(NotifType, String),
	Hide,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show(notif_type, message) => {
			model.is_visible = true;
			model.notif_type = notif_type;
			model.message = message;
			orders.perform_cmd(cmds::timeout(3000, || Msg::Hide));
		},
		Msg::Hide => model.is_visible = false,
	}
}

pub fn view(model: &Model) -> Node<Msg> {
	let c_visible = match &model.is_visible {
		true => "notif-show",
		_ => "",
	};
	let c_type = match &model.notif_type {
		NotifType::Success => "is-success",
		NotifType::Error => "is-danger",
	};
	div![C!["notification", "notif", c_type, c_visible],
		&model.message,
		"aaaa"
	]
}