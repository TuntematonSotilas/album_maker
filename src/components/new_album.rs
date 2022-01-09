use seed::{self, prelude::*, *};

use crate::models::page::TXT_NEW_ALBUM;

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
	div![C!["columns", "is-centered"],
		div![C!("box"),
			h1![C!("title"), TXT_NEW_ALBUM],
			div![C!["field", "has-addons"],
				div![C!("control"),
					input![C!("input"),
						attrs!{
							At::Type => "text", 
							At::Name => "title",
							At::Placeholder => "Title",
						}
					]
				],
				div![C!("control"),
					a![C!["button", "is-primary"], 
						"Save"
					]
				]
			],
			div![C!("field"),
				div![C!("control"),
					a![C!["button", "is-link", "is-light", "is-small"],
						span![C!("icon"),
							i![C!("ion-plus")]
						],
						span!["Add Group"],
					],
				]
			],
		]
	]
}