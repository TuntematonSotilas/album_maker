use seed::{self, prelude::*, *};

use crate::models::page::TITLE_NEW_ALBUM;

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
		div![C!("panel is-link"),
			p![C!("panel-heading"), TITLE_NEW_ALBUM],

			div![C!("m-2"),
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
			],

			
		]
	]
}