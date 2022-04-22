use seed::{self, prelude::*, *};
use uuid::Uuid;

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
	TitleChanged(String, Group),
	UpdateGroup(Group),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::TitleChanged(input, group) => {
			model.group.id = group.id;
			model.group.title = input;
			orders.send_msg(Msg::UpdateGroup(model.group.to_owned()));
		},
		Msg::UpdateGroup(Group) => (),
	}
}

pub fn view(group: Group) -> Node<Msg> {
	
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
					input_ev(Ev::Input, move |input| Msg::TitleChanged(input, group)),
				]
			]
		]
	]
}