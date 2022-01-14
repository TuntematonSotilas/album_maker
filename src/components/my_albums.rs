use seed::{self, prelude::*, *};

use crate::models::page::TITLE_MY_ALBUMS;

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	Fetch,
}

pub fn update(msg: Msg, _model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Fetch => {
            orders.skip(); // No need to rerender
			log!("fetch");
		}
	}
}

// ------ ------
//     View
// ------ ------
pub fn view<Ms>(_model: &Model) -> Node<Ms> {
	div![C!["columns", "is-centered"],
		h1![C!("title"), TITLE_MY_ALBUMS],
	]
}