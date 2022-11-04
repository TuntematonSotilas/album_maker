use crate::{
    api::apifn,
    models::{
        album::Album,
        notif::{Notif, TypeNotifs},
        page::{LK_EDIT_ALBUM, TITLE_EDIT_ALBUM, TITLE_SLIDESHOW, LK_SLIDESHOW},
        vars::THUMB_URI,
    },
};
use seed::{self, prelude::*, *};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    auth_header: String,
    album: Album,
	is_loaded: bool,
}

impl Model {
    pub const fn new() -> Self {
        Self {
            auth_header: String::new(),
            album: Album::new(),
			is_loaded: false,
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp(String),
    ErrorGet,
    Received(Album),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp(id) => {
            orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
            orders.perform_cmd(async {
                let opt_album = apifn::get_album(id, auth).await;
                match opt_album {
                    Some(album) => Msg::Received(album),
                    None => Msg::ErrorGet,
                }
            });
        }
        Msg::ErrorGet => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error getting album".to_string(),
            });
        }
        Msg::Received(album) => {
			model.is_loaded = true;
            model.album = album;
        }
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
	div![
		C!["column", "is-two-thirds"],
		if model.is_loaded {
			div![
				div![
					C!["column"],
					div![
						C!["title", "is-5", "has-text-link"],
						&model.album.title
					],
					div![C!("mb-2"),
						a![
							C!["button", "is-link", "is-light", "is-small", "ml-2"],
							attrs! { At::Href => format!("/{}/{}", LK_EDIT_ALBUM, model.album.id) },
							span![C!("icon"), i![C!("ion-edit")]],
							span![TITLE_EDIT_ALBUM],
						],
						a![
							C!["button", "is-primary", "is-light", "is-small", "ml-2"],
							attrs! { At::Href => format!("/{}/{}", LK_SLIDESHOW, model.album.id) },
							span![C!("icon"), i![C!("ion-play")]],
							span![TITLE_SLIDESHOW],
						]
					]
				],
				match &model.album.groups {
					Some(groups) => div![groups.iter().map(|group| {
						div![
							C!("box"),
							p![C!["title", "is-6", "has-text-link"], &group.title],
							div![match &group.pictures {
								Some(pictures) => div![
										C!["is-flex", "is-flex-wrap-wrap", "is-justify-content-center"],
										pictures.iter().map(|picture| {
											div![
												C!["mr-1", "view-picture"],
												figure![
													C!["image", "is-128x128", "m-1"],
													img![attrs!{ At::Src => format!("{}{}.{}", THUMB_URI, picture.public_id, picture.format) }]
												],
												span![picture.caption.clone().unwrap_or_default()],
											]
										}
									)],
								None => empty![],
							}]
						]
					})],
					None => empty![],
				}
			]
		} else {
			div![
				div![
					C!["column", "is-two-fifths", "mb-4"],
					progress![
						C!["progress", "is-small", "table-progress"],
						attrs! { At::Max => 100 }
					]
				],
				div![
					C!("box"),
					div![
						C!["column", "is-one-third"],
						progress![
							C!["progress", "is-small", "table-progress"],
							attrs! { At::Max => 100 }
						]
					],
					figure![
						C!["image", "is-128x128", "m-4"],
						progress![
							C!["progress", "picture-progress"],
							attrs! { At::Max => 100 }
						],
					],
				],
			]
		}
	]
}
