use crate::{
    api::{albumapi, sharingapi},
    models::{
        album::Album,
        notif::{Notif, TypeNotifs},
        page::{LK_EDIT_ALBUM, LK_SLIDESHOW, TITLE_EDIT_ALBUM, TITLE_SLIDESHOW, LK_SHARESLIDE},
        vars::THUMB_URI, sharing::{Sharing, AddViewLike},
    },
};
use seed::{self, prelude::*, *};

use super::error;

// ------ ------
//     Model
// ------ -----
pub struct Model {
    auth_header: String,
    album: Album,
    is_loaded: bool,
	share_id: Option<String>,
    error: bool,
    is_liked: bool,
}

impl Model {
    pub const fn new() -> Self {
        Self {
            auth_header: String::new(),
            album: Album::new(),
            is_loaded: false,
			share_id: None,
            error: false,
            is_liked: false,
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp(Option<String>, Option<String>),
    ErrorGet,
    Received(Album),
    Share,
    ShareSuccess(String),
    ShareError,
    AddViewLike(bool, bool),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp(id, share_id) => {
            orders.skip(); // No need to rerender
            model.error = false;
            model.is_liked = false;
            let auth = model.auth_header.clone();
            model.share_id = share_id.clone();
            let share_id = share_id.clone();
            orders.perform_cmd(async {
                let opt_album = albumapi::get_album(id, share_id, auth).await;
                opt_album.map_or(Msg::ErrorGet, Msg::Received)
            });
            orders.send_msg(Msg::AddViewLike(true, false));
        }
        Msg::ErrorGet => {
            model.error = true;
        }
        Msg::Received(album) => {
            model.is_loaded = true;
            model.album = album;
        }
        Msg::Share => {
            orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
            let album = model.album.clone();
            orders.perform_cmd(async move {
                let sharing = Sharing {
                    id: String::new(),
                    album_id: album.id,
                    album_name: String::new(),
                    nb_like: 0,
                    nb_view: 0
                };
                let opt_id = sharingapi::add_sharing(auth, sharing).await;
                opt_id.map_or(Msg::ShareError, Msg::ShareSuccess)
            });
        }
        Msg::ShareError => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error sharing".to_string(),
            });
        }
        Msg::ShareSuccess(id) => {
            log!("ShareSuccess");
            let base_url = web_sys::window().unwrap().location().origin().unwrap();
            orders.notify(Notif {
                notif_type: TypeNotifs::Share,
                message: format!("Share your album with this URL : {base_url}/share/{id}"),
            });
        }
        Msg::AddViewLike(is_view, is_like ) => {
            if is_like {
                model.is_liked = true;
            }
            if let Some(share_id) = model.share_id.clone() {
                let auth = model.auth_header.clone();
			    orders.perform_cmd(async move {
                    let add_view_like = AddViewLike {
                        view: is_view,
                        like: is_like,
                        share_id: share_id
                    };
                    sharingapi::add_view_like(auth, add_view_like).await;
                });
            }            
        }
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {

	let mut lk_slideshow = format!("/{LK_SLIDESHOW}/{}", model.album.id);
	if let Some(share_id) = &model.share_id {
		lk_slideshow = format!("/{LK_SHARESLIDE}/{share_id}");
	}

    match model.error {
        false => div![
            C!["column", "is-two-thirds"],
            if model.is_loaded {
                div![
                    div![
                        C!["column"],
                        div![C!["title", "is-5", "has-text-link"], &model.album.title],
                        div![
                            C!["is-flex", "mb-2"],
                            IF!(!model.share_id.is_some() =>
                                div![
                                    a![
                                        C!["button", "is-link", "is-light", "is-small", "mr-2"],
                                        attrs! { At::Href => format!("/{LK_EDIT_ALBUM}/{}", model.album.id) },
                                        span![C!("icon"), i![C!("ion-edit")]],
                                        span![TITLE_EDIT_ALBUM],
                                    ],
                                    button![
                                        C!["button", "is-link", "is-light", "is-small", "mr-2"],
                                        span![C!("icon"), i![C!("ion-android-share-alt")]],
                                        span!["Share"],
                                        ev(Ev::Click, |_| Msg::Share),
                                    ]
                                ]
                            ),
                            a![
                                C!["button", "is-primary", "is-light", "is-small"],
                                attrs! { At::Href => lk_slideshow },
                                span![C!("icon"), i![C!("ion-play")]],
                                span![TITLE_SLIDESHOW],
                            ],
                            IF!(model.share_id.is_some() =>
                                button![
                                    C!["button", "is-danger", "is-light", "is-small", "ml-2"],
                                    span![C!("icon"), i![C!("ion-heart")]],
                                    span!["Like"],
                                    attrs! { At::Disabled => model.is_liked.as_at_value() },
                                    ev(Ev::Click, |_| Msg::AddViewLike(false, true)),
                                ]
                            )
                        ]
                    ],
                    model
                        .album
                        .groups
                        .as_ref()
                        .map_or(empty!(), |groups| div![groups.iter().map(|group| {
                            div![
                                C!("box"),
                                p![C!["title", "is-6", "has-text-link"], &group.title],
                                div![group.pictures.as_ref().map_or(empty!(), |pictures| div![
                                        C!["is-flex", "is-flex-wrap-wrap", "is-justify-content-center"],
                                        pictures.iter().map(|picture| {
                                            div![
                                                C!["mr-1", "album-view-picture"],
                                                figure![
                                                    C!["image", "is-128x128", "m-1"],
                                                    img![attrs!{ At::Src => format!("{THUMB_URI}{}.{}", picture.public_id, picture.format) }]
                                                ],
                                                span![picture.caption.clone()],
                                            ]
                                        }
                                    )])]
                            ]
                        })])
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
        ],
        true => error::view(
            "Forbidden".to_string(),
            "ion-android-remove-circle".to_string())
    }
}
