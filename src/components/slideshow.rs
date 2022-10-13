use std::process::id;

use crate::{
    api::apifn,
    models::{
        album::Album,
        notif::{Notif, TypeNotifs},
    },
};
use seed::{self, prelude::*, *};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    auth_header: String,
    album: Album,
}

impl Model {
    pub const fn new() -> Self {
        Self {
            auth_header: String::new(),
            album: Album::new(),
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
	Fullscreen,
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
            model.album = album;
        }
		Msg::Fullscreen => {
			let ele = seed::document().get_element_by_id("slideshow");
			if let Some(ele) = ele {
				_ = ele.request_fullscreen();
			}
		}
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
    div![C!("slideshow"), id!("slideshow"),
		button![
			C!["button"],
				span![C!("icon"), i![C!("ion-plus")]],
				span!["Fullscreen"],
			ev(Ev::Click, |_| Msg::Fullscreen),
		],
    ]
}
