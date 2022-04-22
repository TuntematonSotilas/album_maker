use seed::{self, prelude::*, *};

use crate::{
	models::{page::TITLE_NEW_ALBUM, album::Album, vars::BASE_URI, group::Group},
	components::group,
};

use super::notification::NotifType;

// ------ ------
//     Model
// ------ -----
pub struct Model {
	auth_header: String,
	album: Album,
	group: group::Model,
}

impl Model {
	pub fn new() -> Self {
		Self {
			auth_header: String::new(),
			album: Album::new(),
			group: group::Model::new(),
		}
	}
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	SetAuth(String),
	Submit,
	TitleChanged(String),
	ShowNotif(NotifType, String),
	AddGroup,
	Group(group::Msg),
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::SetAuth(auth_header) => {
			model.auth_header = auth_header;
		},
		Msg::Submit => {
            orders.skip(); // No need to rerender
			let uri = BASE_URI.to_owned() + "editalbum";
			let auth = model.auth_header.to_owned();
            let request = Request::new(uri)
                .method(Method::Put)
				.header(Header::authorization(auth))
                .json(&model.album)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");

                if response.status().is_ok() {
                    Msg::ShowNotif(NotifType::Success, "Album saved".to_owned())
                } else {
                    Msg::ShowNotif(NotifType::Error, "Error when saving".to_owned())
                }
            });
        },
		Msg::TitleChanged(title) => model.album.title = title,
		Msg::ShowNotif(_, _) => (),
		Msg::AddGroup => {
			if let Some(groups) = &mut model.album.groups {
				groups.push(Group::new());
			}
		},
		Msg::Group(msg) => {
			match msg {
				group::Msg::UpdateGroup(ref group) => {
					log!(group.to_owned())
				},
				_ => (),
			}
			group::update(msg, &mut model.group, &mut orders.proxy(Msg::Group));
			
		}
	}
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
	
	div![C!["column", "is-centered", "is-half"],
		div! [C!("panel"),
			div! [C!("box"),
				p![C!["title" ,"is-5", "has-text-link"], TITLE_NEW_ALBUM],
				div![C!["field", "has-addons"],
					div![C!("control"),
						input![C!("input"),
							attrs!{
								At::Type => "text", 
								At::Name => "title",
								At::Placeholder => "Album name",
								At::Value => model.album.title,
							},
							input_ev(Ev::Input, Msg::TitleChanged),
						]
					],
					div![C!("control"),
						a![C!["button", "is-primary"], 
							"Save",
							ev(Ev::Click, |_| Msg::Submit),
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
							ev(Ev::Click, |_| Msg::AddGroup),
						],
					]
				],
				match &model.album.groups {
					Some(groups) => div![
						groups.iter().map(|group| {
							group::view(group).map_msg(Msg::Group)
						})
					],
					None => empty![]
				}
			],
		],
	]
	
}