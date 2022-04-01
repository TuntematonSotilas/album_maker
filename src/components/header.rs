use seed::{self, prelude::*, *};
use load_dotenv::load_dotenv;

use crate::models::{user::User, page::{LK_NEW_ALBUM, LK_MY_ALBUMS, Page, TITLE_MY_ALBUMS, TITLE_NEW_ALBUM}};

load_dotenv!();

const TITLE: &str = "Album maker";

// ------ ------
//     Model
// ------ -----
pub struct Model {
	pub user: Option<User>,
	is_menu_open: bool,
	page: Page,
}

impl Model {
	pub fn new(page: Page) -> Self {
		Self { 
			user: None, 
			is_menu_open: false,
			page: page,
		}
	}
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	OpenOrCloseMenu,
	SetPage(Page),
	UserLoged,
	LogInOrOut,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::OpenOrCloseMenu => {
			model.is_menu_open = !model.is_menu_open;
		},
		Msg::SetPage(page) => {
			model.page = page;
		},
		Msg::UserLoged => (),
		Msg::LogInOrOut => {
			model.user = Some(User {
				name: "GB".to_string(),
				picture : "GB".to_string(),
				sub : "GB".to_string()
			});
			orders.send_msg(Msg::UserLoged);
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
						IF!(model.user.is_some() => 
							figure![C!["image", "is-24x24", "htitle-avatar"],
								img![C!("is-rounded"), 
									attrs!{ 
										At::Src => model.user.to_owned().unwrap().picture,
										At::Alt => model.user.to_owned().unwrap().name,
										At::Title => model.user.to_owned().unwrap().name,
										At::ReferrerPolicy => "no-referrer",
									}
								]
							]
						),
						a![C!["button", "is-light"],
							b![
								match model.user.is_some() {
									true => "LOGOUT",
									false => "LOGIN",
								}
							],
							ev(Ev::Click, |_| Msg::LogInOrOut),
						]
					]
				]
			]
		]
	]
}