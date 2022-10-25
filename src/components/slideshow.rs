use crate::{
    api::apifn,
    models::{
        album::Album,
        notif::{Notif, TypeNotifs}, vars::{IMG_URI, LOW_URI}, picture::Picture,
    },
};
use seed::{self, prelude::*, *};

#[derive(Debug, Clone)]
struct Slide {
	is_title: bool,
	group_title: Option<String>,
	picture: Option<Picture>,
}

// ------ ------
//     Model
// ------ -----
pub struct Model {
    auth_header: String,
    album: Album,
	slides: Vec<Slide>,
	slide: Slide,
	slide_id: usize,
	pic_loaded: bool,
}

impl Model {
    pub const fn new() -> Self {
        Self {
            auth_header: String::new(),
            album: Album::new(),
			slides: Vec::new(),
			slide: Slide {
				is_title: false,
				group_title: None,
				picture: None,
			},
			slide_id: 0,
			pic_loaded: false,
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp(String),
	InitSlides,
    ErrorGet,
    Received(Album),
	Next,
	PicLoadEnd,
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
			model.slide_id = 0;
        }
		Msg::ErrorGet => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error getting album".to_string(),
            });
        }
        Msg::Received(album) => {
            model.album = album;
			orders.send_msg(Msg::InitSlides);
			orders.send_msg(Msg::Next);
        }
		Msg::InitSlides => {
			model.slides.push(
				Slide { 
					is_title: true, 
					group_title: None, 
					picture: None,
				}
			);
			if let Some(groups) = model.album.groups.clone() {
				for group in groups.iter() {
					model.slides.push(
						Slide { 
							is_title: false, 
							group_title: Some(group.title.clone()), 
							picture: None
						}
					);
					if let Some(pictures) = group.pictures.clone() {
						for picture in pictures {
							model.slides.push(
								Slide { 
									is_title: false, 
									group_title: None, 
									picture: Some(picture)
								});
						}
					}
				}
			}
		}
		Msg::Next => {
			if let Some(slide) = model.slides.get(model.slide_id.clone()) {
				model.slide = slide.clone();
				model.slide_id += 1;
			}
		}
		Msg::PicLoadEnd => {
			//orders.skip();
			log!("LoadEnd");
			model.pic_loaded = true;
		}
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
	div![
		id!("slideshow"),
		C!("slideshow"),
		if model.slide.is_title || model.slide.group_title.is_some() {
			div![
				C!("container"),
				div![
					C!["hero", "is-large"],
					div![
						C!("hero-body"),
						div![
							C!["is-flex", "is-justify-content-center", "has-text-centered"],
							h1![C!["title", "has-text-link" ], 
								if let Some(group_title) = &model.slide.group_title {
									&group_title
								} else {
									&model.album.title
								}
							],
						],
					],
				]
			]
		} else if let Some(picture) = &model.slide.picture {
			let src = match &model.pic_loaded { 
				true => format!("{}{}.{}", IMG_URI, picture.public_id, picture.format),
				false => format!("{}{}.{}", LOW_URI, picture.public_id, picture.format),
			};
			div![C!["is-flex", "is-justify-content-center", "slideshow-image-container"],
				h2![
					C!["slideshow-caption", "title", "is-3", "mt-5"],
					&picture.caption
				],
				img![
					C!("slideshow-image"),
					attrs! { At::Src => src },
					ev(Ev::LoadEnd, move |_| {
						Msg::PicLoadEnd
					}),
				]
			]
		} else {
			empty!()
		},
		ev(Ev::Click, |_| Msg::Next),
    ]
}
