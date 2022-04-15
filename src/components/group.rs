use seed::{self, prelude::*, *};

use crate::models::group::Group;

pub fn view<Ms>(group: &Group) -> Node<Ms> {
	div![C!("panel-block"),
		div![C!("field"),
			div![C!("control"),
				input![C!("input"),
					attrs!{
						At::Type => "text", 
						At::Name => "title",
						At::Placeholder => "Title",
						At::Value => group.title,
					},
				]
			]
		]
	]
}