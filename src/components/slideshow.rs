use crate::{
    api::apifn,
    models::{
        album::Album,
        notif::{Notif, TypeNotifs}, vars::IMG_URI, picture::Picture,
    },
};
use seed::{self, prelude::*, *};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    auth_header: String,
    album: Album,
	is_title: bool,
	group_title: Option<String>,
	picture: Option<Picture>,
	current_group: usize,
	current_pic: usize,
}

impl Model {
    pub const fn new() -> Self {
        Self {
            auth_header: String::new(),
            album: Album::new(),
			is_title: true,
			group_title: None,
			picture: None,
			current_group: 0,
			current_pic: 0,
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
	Next,
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
			model.is_title = true;
			model.current_group = 0;
			model.current_pic = 0;
        }
        Msg::ErrorGet => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error getting album".to_string(),
            });
        }
        Msg::Received(album) => {
            model.album = album;
        },
		Msg::Next => {
			if let Some(groups) = &model.album.groups {
				let grp = groups.get(model.current_group);
				if let Some(grp) = grp {
					if model.group_title.is_none() {
						model.group_title = Some(grp.title.clone());
					} else {
						model.is_title = false;
						model.group_title = None;
						if let Some(pictures) = &grp.pictures
						{
							if let Some(picture) = pictures.get(model.current_pic) 
							{
								log!("pic found");
								model.picture = Some(picture.clone());
								model.current_pic += 1;
							} else {
								model.current_group += 1;
							}
						}
					}
					
				}
			}
		}
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
    div![
		C!("slideshow"),
		if model.is_title || model.group_title.is_some() {
			div![
				C!("container"),
				div![
					C!["hero", "is-large"],
					div![
						C!("hero-body"),
						div![
							C!["is-flex", "is-justify-content-center", "has-text-centered"],
							h1![C!["title", "has-text-link" ], 
								if let Some(group_title) = &model.group_title {
									&group_title
								} else {
									&model.album.title
								}
							],
						],
					],
				]
			]
		} else if let Some(picture) = &model.picture {
			figure![
				C!["image", "slideshow-image"],
				img![
					C!("slideshow-img"),
					attrs! { At::Src => format!("{}{}.{}", IMG_URI, picture.public_id, picture.format) }
				]
			]
		} else {
			empty!()
		},
		ev(Ev::Click, |_| Msg::Next),
    ]
}
