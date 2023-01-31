use crate::{
    api::albumapi,
    models::{
        album::Album,
        picture::Picture,
        vars::{IMG_URI, VERY_LOW_URI}, trip::{Trip, TranspMode},
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
    show_caption: bool,
    show_trip: bool,
    show_pic: bool,
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
            show_caption: false,
            show_trip: false,
			show_pic: false,
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
                model.show_caption = false;
                model.show_trip = false;
				model.show_pic = false;
				
                orders.perform_cmd(cmds::timeout(300, || Msg::ShowCaption));
				orders.perform_cmd(cmds::timeout(600, || Msg::ShowTrip));
                orders.perform_cmd(cmds::timeout(3000, || Msg::ShowPic));
            }
        }
        Msg::ShowCaption => {
            model.show_caption = true;
        }
		Msg::ShowTrip => {
            model.show_trip = true;
        }
        Msg::ShowPic => {
            model.show_pic = true;
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
					C!("slideshow-caption-ctn"),
                    IF!(model.show_caption =>
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
					if let Some(trip) = &model.slide.trip {
						let mut show_trip = "";
						if model.show_trip {
							show_trip = "trip-show";
						}
						let veh_icon = match trip.transp_mode {
							TranspMode::Plane => "ion-android-plane",
							TranspMode::Train => "ion-android-train",
							TranspMode::Car => "ion-android-car",
						};
						div![
							C![
								"trip", 
								show_trip,
								&model.album.caption_color.to_string()
							],
							div![
								C!("trip-veh-ctn"), 
								div![C!("trip-veh"), 
									div![
										C!("trip-veh-icon"),
										span![C!("icon"), i![C!(veh_icon)]]
									]
								],
							],
							div![
								C!("trip-line-ctn"),
								div![C!("trip-line")],
							],
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
					} else {
						empty!()
					}
				]
            } else if let Some(picture) = &model.slide.picture {
                let hide = if model.show_pic {
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
                    IF!(model.show_pic =>
                        div![
                            C![
                                "slideshow-caption-ctn",
                                "slideshow-caption-pic"
                            ],
                            IF!(model.show_caption =>
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
