use crate::{
    api::albumapi,
    models::{
        album::Album,
        picture::Picture,
        vars::{IMG_URI, VERY_LOW_URI}, trip::Trip,
    },
};
use seed::{self, prelude::*, *};

use super::error;

#[derive(Debug, Clone)]
struct Slide {
    is_title: bool,
    group_title: Option<String>,
	trip: Option<Trip>,
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
    caption_animate: bool,
    pic_loaded: bool,
    error: bool,
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
				trip: None,
            },
            slide_id: 0,
            caption_animate: false,
            pic_loaded: false,
            error: false,
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp(Option<String>, Option<String>),
    InitSlides,
    ErrorGet,
    Received(Album),
    Next,
    ShowAnim,
    ShowPic,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp(id, share_id) => {
            orders.skip(); // No need to rerender
            model.error = false;
            model.slide_id = 0;
            model.slide = Slide {
                is_title: false,
                group_title: None,
                picture: None,
				trip: None,
            };
            let auth = model.auth_header.clone();
            orders.perform_cmd(async {
                let opt_album = albumapi::get_album(id, share_id, auth).await;
                opt_album.map_or(Msg::ErrorGet, Msg::Received)
            });
        }
        Msg::ErrorGet => {
            model.error = true;
        }
        Msg::Received(album) => {
            model.album = album;
            orders.send_msg(Msg::InitSlides);
            orders.send_msg(Msg::Next);
        }
        Msg::InitSlides => init_slides(model),
        Msg::Next => {
            if let Some(slide) = model.slides.get(model.slide_id) {
                model.slide = slide.clone();
                model.slide_id += 1;
                model.caption_animate = false;
                model.pic_loaded = false;

                orders.perform_cmd(cmds::timeout(300, || Msg::ShowAnim));
                orders.perform_cmd(cmds::timeout(3000, || Msg::ShowPic));
            }
        }
        Msg::ShowAnim => {
            model.caption_animate = true;
        }
        Msg::ShowPic => {
            model.pic_loaded = true;
        }
    }
}

fn init_slides(model: &mut Model) {
    // Cover
    let mut cover: Option<Picture> = None;
    let grps = model.album.groups.clone().unwrap_or_default();
    for grp in &grps {
        if let Some(pic) = grp
            .pictures
            .clone()
            .unwrap_or_default()
            .iter()
            .find(|p| p.asset_id == model.album.cover)
        {
            cover = Some(pic.clone());
            break;
        }
    }

	// Slide for album title
    model.slides.push(Slide {
        is_title: true,
        group_title: None,
        picture: cover,
		trip: None,
    });

    let groups = model.album.groups.clone().unwrap_or_default();
    for group in &groups {
        // Cover
        let mut grp_cover: Option<Picture> = None;
        if let Some(pic) = group
            .pictures
            .clone()
            .unwrap_or_default()
            .iter()
            .find(|p| p.asset_id == group.cover)
        {
            grp_cover = Some(pic.clone());
        }
        model.slides.push(Slide {
            is_title: false,
            group_title: Some(group.title.clone()),
            picture: grp_cover,
			trip: group.trip.clone(),
        });
        if let Some(pictures) = group.pictures.clone() {
            for picture in pictures {
                model.slides.push(Slide {
                    is_title: false,
                    group_title: None,
                    picture: Some(picture),
					trip: None,
                });
            }
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
            St::BackgroundImage => format!("url({VERY_LOW_URI}{}.{})", picture.public_id, picture.format),
        };
    }

    if model.error {
        error::view(
            "Forbidden".to_string(),
            "ion-android-remove-circle".to_string(),
        )
    } else {
        div![
            id!("slideshow"),
            C!("slideshow"),
            s_bkg,
            if model.slide.is_title || model.slide.group_title.is_some() {
                div![
                    C![
                        "is-flex",
                        "is-justify-content-center",
                        "is-align-items-center",
                        "slideshow-caption-ctn"
                    ],
                    if let Some(trip) = &model.slide.trip {
                        div![
                            C!("trip"),
                            span![&trip.origin],
                            span![&trip.destination],
                        ]
                    } else {
                        empty!()
                    },
                    IF!(model.caption_animate =>
                        h2![
                            C![
                                "slideshow-caption",
                                "title",
                                "is-4",
                                &model.album.caption_color.to_string(),
                                &model.album.caption_style.to_string(),
                                "slideshow-caption-anim"
                            ],
                            model
                                .slide
                                .group_title
                                .as_ref()
                                .map_or(&model.album.title, |group_title| group_title)
                        ]
                    )
                ]
            } else if let Some(picture) = &model.slide.picture {
                let hide = if model.pic_loaded {
                    ""
                } else {
                    "slideshow-image-hide"
                };

                div![
                    C![
                        "is-flex",
                        "is-justify-content-center",
                        "slideshow-image-container",
                        "is-align-items-center"
                    ],
                    img![
                        C!["slideshow-image", hide],
                        attrs! { At::Src => format!("{IMG_URI}{}.{}", picture.public_id, picture.format) }
                    ],
                    IF!(model.pic_loaded =>
                        div![
                            C![
                                "is-flex",
                                "is-justify-content-center",
                                "is-align-items-end",
                                "slideshow-caption-ctn"],
                            IF!(model.caption_animate =>
                                h2![
                                    C!["slideshow-caption", "title", "is-5", "mt-5",
                                        &model.album.caption_color.to_string(),
                                        &model.album.caption_style.to_string(),
                                        "slideshow-caption-anim"
                                    ],
                                    &picture.caption
                                ]
                            )
                        ]
                    )
                ]
            } else {
                empty!()
            },
            ev(Ev::Click, |_| Msg::Next),
        ]
    }
}
