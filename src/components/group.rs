use seed::{self, prelude::*, *};

use crate::models::group::Group;

// ------ ------
//     Model
// ------ -----
pub struct Model {
	group: Group,
}

impl Model {
	pub fn new() -> Self {
		Self {
			group: Group::new(),
		}
	}
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	SetTitle(String),
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::SetTitle(title) => model.group.title = title,
	}
}

pub fn view<Ms>(group: &Group) -> Node<Ms> {
	div![C!("panel-block"),
		div![C!("field"),
			div![C!("control"),
				input![C!("input"),
					attrs!{
						At::Type => "text", 
						At::Name => "title",
						At::Placeholder => "Group name",
						At::Value => group.title,
					},
				]
			]
		]
	]
}