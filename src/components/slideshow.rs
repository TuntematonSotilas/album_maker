use crate::{
    api::albumapi,
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
    caption_animate: bool,
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
            caption_animate: false,
            pic_loaded: false,
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
    PicLoadEnd,
    ErrorGetPic,
    ShowAnim,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp(id, share_id) => {
			log!("InitComp slide");
            orders.skip(); // No need to rerender
            model.slide_id = 0;
            model.slide = Slide {
                is_title: false,
                group_title: None,
                picture: None,
            };
            let auth = model.auth_header.clone();
            orders.perform_cmd(async {
                let opt_album = albumapi::get_album(id, share_id, auth).await;
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
        Msg::InitSlides => init_slides(model),
        Msg::Next => {
            if let Some(slide) = model.slides.get(model.slide_id) {
                model.slide = slide.clone();
                model.slide_id += 1;
                model.caption_animate = false;
                model.pic_loaded = false;

                orders.perform_cmd(cmds::timeout(1, || Msg::ShowAnim));

                if !model.slide.is_title && model.slide.group_title.is_none() {
                    if let Some(pic) = slide.picture.clone() {
                        orders.skip();
                        orders.perform_cmd(async move {
                            let url = format!("url({IMG_URI}{}.{})", pic.public_id, pic.format);
                            let response = fetch(url).await.expect("HTTP request failed");
                            if response.status().is_ok() {
                                Msg::PicLoadEnd
                            } else {
                                Msg::ErrorGetPic
                            }
                        });
                    }
                }
            }
        }
        Msg::PicLoadEnd => {
            model.pic_loaded = true;
        }
        Msg::ErrorGetPic => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error getting picture".to_string(),
            });
        }
        Msg::ShowAnim => {
            model.caption_animate = true;
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

    model.slides.push(Slide {
        is_title: true,
        group_title: None,
        picture: cover,
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
    let caption_anim = if model.caption_animate {
        "slideshow-caption-anim"
    } else {
        "slideshow-caption-hide"
    };

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
                h2![
                    C![
                        "slideshow-caption",
                        "title",
                        "is-4",
                        &model.album.caption_color.to_string(),
                        &model.album.caption_style.to_string(),
                        caption_anim
                    ],
                    model
                        .slide
                        .group_title
                        .as_ref()
                        .map_or(&model.album.title, |group_title| group_title)
                ],
            ]
        } else if let Some(picture) = &model.slide.picture {
            let src = match &model.pic_loaded {
                true => format!("{IMG_URI}{}.{}", picture.public_id, picture.format),
                false => format!("{LOW_URI}{}.{}", picture.public_id, picture.format),
            };
            div![
                C![
                    "is-flex",
                    "is-justify-content-center",
                    "slideshow-image-container"
                ],
                img![C!("slideshow-image"), attrs! { At::Src => src },],
                IF!(picture.caption.is_some() =>
                    div![
                        C!["is-flex", "is-justify-content-center"],
                        h2![
                            C!["slideshow-caption", "title", "is-4", "mt-5",
                                &model.album.caption_color.to_string(),
                                &model.album.caption_style.to_string(),
                                caption_anim
                            ],
                            &picture.caption
                        ],
                    ]
                ),
            ]
        } else {
            empty!()
        },
        ev(Ev::Click, |_| Msg::Next),
    ]
}
