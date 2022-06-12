use seed::{self, prelude::*, *};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
	auth_header: String,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	SetAuth(String),
    InitComp(Option<String>),
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::SetAuth(auth_header) => model.auth_header = auth_header,
		Msg::InitComp(opt_id) => {
			if let Some(id) = opt_id {
				log!(id);
			}
		}
	}
}

// ------ ------
//     View
// ------ ------
pub fn view(_model: &Model) -> Node<Msg> {
    div![
		"detail"
	]
}
