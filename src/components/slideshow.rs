use crate::{
    api::apifn,
    models::{
        album::Album,
        notif::{Notif, TypeNotifs},
        picture::Picture,
        vars::{IMG_URI, LOW_URI, VERY_LOW_URI},
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
            model.slide_id = 0;
            model.slide = Slide {
                is_title: false,
                group_title: None,
                picture: None,
            };
            let auth = model.auth_header.clone();
            orders.perform_cmd(async {
                let opt_album = apifn::get_album(id, auth).await;
                opt_album.map_or(Msg::ErrorGet, Msg::Received)
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
            orders.send_msg(Msg::InitSlides);
            orders.send_msg(Msg::Next);
        }
        Msg::InitSlides => {
            model.slides.push(Slide {
                is_title: true,
                group_title: None,
                picture: None,
            });
            if let Some(groups) = model.album.groups.clone() {
                for group in &groups {
                    model.slides.push(Slide {
                        is_title: false,
                        group_title: Some(group.title.clone()),
                        picture: None,
                    });
                    if let Some(pictures) = group.pictures.clone() {
                        for picture in pictures {
                            model.slides.push(Slide {
                                is_title: false,
                                group_title: None,
                                picture: Some(picture),
                            });
                        }
                    }
                }
            }
        }
        Msg::Next => {
            if let Some(slide) = model.slides.get(model.slide_id) {
                model.slide = slide.clone();
                model.slide_id += 1;
            }
        }
        Msg::PicLoadEnd => {
            model.pic_loaded = true;
        }
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
    let mut s_bkg = style! {};
    if let Some(picture) = &model.slide.picture {
        s_bkg = style! {
            St::BackgroundImage => format!("url({}{}.{})", VERY_LOW_URI, picture.public_id, picture.format),
        }
    }

    div![
        id!("slideshow"),
        C!("slideshow"),
        s_bkg,
        if model.slide.is_title || model.slide.group_title.is_some() {
            div![
                C!("container"),
                div![
                    C!["hero", "is-large"],
                    div![
                        C!("hero-body"),
                        div![
                            C!["is-flex", "is-justify-content-center", "has-text-centered"],
                            h1![
                                C!["title", "has-text-link"],
                                model
                                    .slide
                                    .group_title
                                    .as_ref()
                                    .map_or(&model.album.title, |group_title| group_title)
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
            div![
                C![
                    "is-flex",
                    "is-justify-content-center",
                    "slideshow-image-container"
                ],
                div![
                    C!("slideshow-caption-anim"),
                    h2![
                        C!["slideshow-caption", "title", "is-4", "mt-5", 
                            &model.album.caption_color.to_string(), 
                            &model.album.caption_style.to_string() 
                        ],
                        &picture.caption
                    ],
                ],
                img![
                    C!("slideshow-image"),
                    attrs! { At::Src => src },
                    ev(Ev::LoadEnd, move |_| { Msg::PicLoadEnd }),
                ]
            ]
        } else {
            empty!()
        },
        ev(Ev::Click, |_| Msg::Next),
    ]
}
