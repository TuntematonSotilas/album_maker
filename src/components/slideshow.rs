use crate::{
    api::albumapi,
    models::{
        album::Album,
        picture::Picture,
        trip::{TranspMode, Trip},
        vars::{IMG_URI, VERY_LOW_URI},
    },
};
use seed::{self, prelude::*, *};
use std::collections::HashMap;

use super::error;

#[derive(Debug, Clone)]
struct Slide {
    is_title: bool,
    group_title: Option<String>,
    trip: Option<Trip>,
    picture: Option<Picture>,
}

#[derive(Debug)]
enum Element {
    Caption,
    Trip,
    Picture,
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
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
    show_elem: HashMap<String, bool>,
    error: bool,
    cover: String,
}

impl Model {
    pub fn new() -> Self {
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
            error: false,
            show_elem: HashMap::new(),
            cover: String::new(),
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
    ShowCaption,
    PreLoadPic,
    ShowPic,
    ShowTrip,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp(id, share_id) => {
            orders.skip(); // No need to rerender
            model.error = false;
            model.slide_id = 0;
            model.slides = Vec::new();
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
                model
                    .show_elem
                    .entry(Element::Caption.to_string())
                    .and_modify(|e| *e = false)
                    .or_insert(false);
                model
                    .show_elem
                    .entry(Element::Trip.to_string())
                    .and_modify(|e| *e = false)
                    .or_insert(false);
                model
                    .show_elem
                    .entry(Element::Picture.to_string())
                    .and_modify(|e| *e = false)
                    .or_insert(false);

                orders.send_msg(Msg::PreLoadPic);

                orders.perform_cmd(cmds::timeout(300, || Msg::ShowCaption));
                orders.perform_cmd(cmds::timeout(600, || Msg::ShowTrip));
            }
        }
        Msg::ShowCaption => {
            model
                .show_elem
                .entry(Element::Caption.to_string())
                .and_modify(|e| *e = true);
        }
        Msg::ShowTrip => {
            model
                .show_elem
                .entry(Element::Trip.to_string())
                .and_modify(|e| *e = true);
        }
        Msg::PreLoadPic => {
            if let Some(pic) = &model.slide.picture {
                let uri = format!("{IMG_URI}{}.{}", pic.public_id, pic.format);
                orders.perform_cmd(async {
                    let _ok = albumapi::preload_picture(uri).await;
                    Msg::ShowPic
                });
            }
        }
        Msg::ShowPic => {
            model
                .show_elem
                .entry(Element::Picture.to_string())
                .and_modify(|e| *e = true);
        }
    }
}

fn init_slides(model: &mut Model) {
    // Cover
    let grps = model.album.groups.clone().unwrap_or_default();
    for grp in &grps {
        if let Some(pic) = grp.pictures.clone().unwrap_or_default().first() {
            model.cover = format!("url({VERY_LOW_URI}{}.{})", pic.public_id, pic.format);
            break;
        }
    }

    // Slide for album title
    model.slides.push(Slide {
        is_title: true,
        group_title: None,
        picture: None,
        trip: None,
    });

    let groups = model.album.groups.clone().unwrap_or_default();
    for group in &groups {
        // Cover
        model.slides.push(Slide {
            is_title: false,
            group_title: Some(group.title.clone()),
            picture: None,
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
    let s_bkg = style! {
        St::BackgroundImage => model.cover
    };

    let show_cap = model
        .show_elem
        .get(&Element::Caption.to_string())
        .unwrap_or(&false);
    let show_pic = model
        .show_elem
        .get(&Element::Picture.to_string())
        .unwrap_or(&false);

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
                    C!("slideshow-caption-ctn"),
                    IF!(*show_cap =>
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
                    ),
                    trip_view(model),
                ]
            } else if let Some(picture) = &model.slide.picture {
                div![
                    C![
                        "is-flex",
                        "is-justify-content-center",
                        "slideshow-image-container",
                        "is-align-items-center"
                    ],
                    IF!(!show_pic =>
                        div![
                            C!("spiner-pic"),
                            i![C!("ion-load-c")]
                        ]
                    ),
                    IF!(*show_pic =>
                        img![
                            C!["slideshow-image"],
                            attrs! { At::Src => format!("{IMG_URI}{}.{}", picture.public_id, picture.format) }
                        ]
                    ),
                    IF!(*show_pic =>
                        div![
                            C![
                                "slideshow-caption-ctn",
                                "slideshow-caption-pic"
                            ],
                            IF!(*show_cap =>
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

fn trip_view(model: &Model) -> Node<Msg> {
    let show_trip = model
        .show_elem
        .get(&Element::Trip.to_string())
        .unwrap_or(&false);

    model.slide.trip.as_ref().map_or_else(
        || empty!(),
        |trip| {
            let c_show_trip = if *show_trip { "trip-show" } else { "" };

            let veh_icon = match trip.transp_mode {
                TranspMode::Plane => "ion-android-plane trip-veh-icon-plane",
                TranspMode::Train => "ion-android-train",
                TranspMode::Car => "ion-android-car",
            };

            let c_veh_icon_plane = if trip.transp_mode == TranspMode::Plane {
                "trip-veh-icon-plane"
            } else {
                ""
            };
            let c_line = if trip.transp_mode == TranspMode::Plane {
                "trip-line-plane"
            } else {
                ""
            };

            div![
                C!["trip", c_show_trip, &model.album.caption_color.to_string()],
                div![
                    C!("trip-veh-ctn"),
                    div![
                        C!["trip-veh"],
                        div![
                            C!["trip-veh-icon", c_veh_icon_plane],
                            span![C!("icon"), i![C!(veh_icon)]]
                        ]
                    ],
                ],
                div![C!("trip-line-ctn"), div![C!["trip-line", c_line]],],
                div![
                    C!("trip-pins"),
                    span![C!("icon"), i![C!("ion-android-pin")]],
                    span![C!("trip-sep")],
                    span![C!("icon"), i![C!("ion-android-pin")]],
                ],
                div![
                    span![&trip.origin],
                    span![C!("trip-sep")],
                    span![&trip.destination],
                ]
            ]
        },
    )
}
