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
}

impl Default for NotifType {
    fn default() -> Self { NotifType::Success }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	Show(NotifType, String),
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Show(notif_type, message) => {
			log!("Show");
			model.is_visible = true;
			model.notif_type = notif_type;
			model.message = message;
		},
	}
}

pub fn view(model: &Model) -> Node<Msg> {
	let c_visible = match &model.is_visible {
		false => "is-hidden",
		_ => "",
	};
	let c_type = match &model.notif_type {
		Sucess => "is-success",
		_ => "is-info",
	};
	div![C!["notification", c_type, c_visible],
		button![C!("delete")],
		&model.message,
    ]
}