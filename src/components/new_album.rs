use seed::{self, prelude::*, *};

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
}

pub fn update(msg: Msg, _model: &mut Model, _orders: &mut impl Orders<Msg>) {
	match msg {
	}
}

// ------ ------
//     View
// ------ ------
pub fn view<Ms>(_model: &Model) -> Node<Ms> {
	div![
		div![C!("field"),
			label![
				C!("label"),
				"Title",
				attrs!{ At::For => "title" },
			],
			div![C!("control"),
				input![C!("input"),
					attrs!{
						At::Type => "text", 
						At::Id => "title",
						At::Name => "title",
						At::Placeholder => "Title",
					}
				]
			]
		],
		div![C!("field"),
			div![C!("control"),
				button![C!("button is-primary"),
					"Submit"
				]
			]
		],
	]
}