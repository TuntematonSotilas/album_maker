use seed::{self, prelude::*, *};
use crate::models::page::{LK_NEW_ALBUM, LK_MY_ALBUMS, Page, TITLE_MY_ALBUMS, TITLE_NEW_ALBUM, LK_LOGIN};

const TITLE: &str = "Album maker";

// ------ ------
//     Model
// ------ -----
pub struct Model {
	is_menu_open: bool,
	page: Page,
	is_logged: bool,
}

impl Model {
	pub fn new(page: Page) -> Self {
		Self { 
			is_menu_open: false,
			page: page,
			is_logged: false,
		}
	}
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	OpenOrCloseMenu,
	SetPage(Page),
}

pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::OpenOrCloseMenu => {
			model.is_menu_open = !model.is_menu_open;
		},
		Msg::SetPage(page) => {
			model.page = page;
		},
	}
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
	let menu_is_active = match &model.is_menu_open {
		true => "is-active",
		false => ""
	};
	nav![C!("navbar"),
		attrs!{ At::AriaLabel => "main navigation" },
		div![C!("navbar-brand"),
			a![C!("navbar-item"),
				attrs!{ At::Href => "/" },
				div![C!("htitle"), 
					div![TITLE]
				],
			],
			a![C!["navbar-burger", menu_is_active],
				attrs!{ 
					At::AriaLabel => "menu", 
					At::AriaExpanded => &model.is_menu_open
				},
      			span![attrs!{ At::AriaHidden => "true" }],
				span![attrs!{ At::AriaHidden => "true" }],
				span![attrs!{ At::AriaHidden => "true" }],
				ev(Ev::Click, |_| Msg::OpenOrCloseMenu),
			],
		],
		div![C!["navbar-menu", menu_is_active],
			a![C!["navbar-item", "is-tab", IF!(matches!(model.page, Page::MyAlbums) => "is-active")],
				attrs! { At::Href => format!("/{}", LK_MY_ALBUMS) },
		        TITLE_MY_ALBUMS
			],
			div![C!("navbar-item"),
				div![C!("buttons"),
					a![C!["button", "is-primary"],
						attrs!{ At::Href => format!("/{}", LK_NEW_ALBUM) },
						span![C!("icon"),
							i![C!("ion-plus")]
						],
						span![TITLE_NEW_ALBUM],
					],
				],
			],
			div![C!("navbar-end"),
				div![C!("navbar-item"),
					div![C!("buttons"),
						a![C!["button", "is-light"],
							attrs!{ At::Href => format!("/{}", LK_LOGIN) },
							match model.is_logged {
								true => "Sign out",
								false => "Sign in",
							}
						]
					]
				]
			]
		]
	]
}