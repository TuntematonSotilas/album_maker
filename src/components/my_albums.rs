use seed::{self, prelude::*, *};

use crate::models::page::TITLE_MY_ALBUMS;

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
}

// ------ ------
//     View
// ------ ------
pub fn view<Ms>(_model: &Model) -> Node<Ms> {
	div![C!["columns", "is-centered"],
		h1![C!("title"), TITLE_MY_ALBUMS],
	]
}