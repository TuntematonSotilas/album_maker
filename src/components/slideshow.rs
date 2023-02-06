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

                orders.perform_cmd(cmds::timeout(300, || Msg::ShowCaption));
                orders.perform_cmd(cmds::timeout(600, || Msg::ShowTrip));
                orders.perform_cmd(cmds::timeout(3000, || Msg::ShowPic));
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

    let show_cap = model.show_elem.get(&Element::Caption.to_string()).unwrap_or(&false);
    let show_pic = model.show_elem.get(&Element::Picture.to_string()).unwrap_or(&false);

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
                let hide = if *show_pic {
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
    let show_trip = model.show_elem.get(&Element::Trip.to_string()).unwrap_or(&false);

    model.slide.trip.as_ref().map_or_else(
        || empty!(),
        |trip| {
            let c_show_trip = if *show_trip { "trip-show" } else { "" };
            let veh_icon = match trip.transp_mode {
                TranspMode::Plane => "ion-android-plane",
                TranspMode::Train => "ion-android-train",
                TranspMode::Car => "ion-android-car",
            };
            let c_rotate = if trip.transp_mode == TranspMode::Plane {
                "trip-veh-rotate"
            } else {
                ""
            };

            div![
                C!["trip", c_show_trip, &model.album.caption_color.to_string()],
                div![
                    C!("trip-veh-ctn"),
                    div![
                        C!("trip-veh"),
                        div![
                            C!["trip-veh-icon", c_rotate], 
                            span![C!("icon"), i![C!(veh_icon)]]
                        ]
                    ],
                ],
                div![C!("trip-line-ctn"), div![C!("trip-line")],],
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
