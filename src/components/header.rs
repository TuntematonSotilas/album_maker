use crate::models::page::{
    Page, LK_MY_ALBUMS, LK_NEW_ALBUM, LK_VIEW_ALBUM, LK_MY_SHARINGS, TITLE_MY_ALBUMS, TITLE_NEW_ALBUM, TITLE_MY_SHARINGS,LK_SHARE,
};
use seed::{self, prelude::*, *};

const TITLE: &str = "Album maker";

// ------ ------
//     Model
// ------ -----
pub struct Model {
    is_menu_open: bool,
    page: Page,
    is_logged: bool,
	share_id: Option<String>,
}

impl Model {
    pub const fn new(page: Page) -> Self {
        Self {
            is_menu_open: false,
            page,
            is_logged: false,
			share_id: None,
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    OpenOrCloseMenu,
    SetPage(Page),
    SetIsLogged,
    ClickLogInOrOut,
    LogInOrOut,
    Fullscreen,
	SetShareId(Option<String>),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::OpenOrCloseMenu => {
            model.is_menu_open = !model.is_menu_open;
        }
        Msg::SetPage(page) => {
            model.page = page;
            if model.is_menu_open {
                model.is_menu_open = false;
            }
        }
        Msg::SetIsLogged => {
            model.is_logged = true;
        }
		Msg::SetShareId(share_id) => {
            model.share_id = share_id;
        }
        Msg::ClickLogInOrOut => {
            if model.is_logged {
                model.is_logged = false;
            }
            orders.send_msg(Msg::LogInOrOut);
        }
        Msg::LogInOrOut => (),
        Msg::Fullscreen => {
            let ele = seed::document().get_element_by_id("slideshow");
            if let Some(ele) = ele {
                let _res = ele.request_fullscreen();
            }
        }
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
	let mut lk_album = format!("/{LK_VIEW_ALBUM}");
	if let Some(share_id) = &model.share_id {
		lk_album = format!("/{LK_SHARE}/{share_id}");
	}

    let c_slide = if model.page == Page::Slideshow || model.page == Page::ShareSlide  {
        "navbar-slideshow"
    } else {
        ""
    };
    let menu_is_active = match &model.is_menu_open {
        true => "is-active",
        false => "",
    };
    nav![
        C!["navbar", c_slide],
        attrs! { At::AriaLabel => "main navigation" },
        IF!(model.page != Page::Slideshow && model.page != Page::ShareSlide =>
            div![
                C!("navbar-brand"),
                a![
                    C!("navbar-item"),
                    attrs! { At::Href => "/" },
                    div![C!("htitle"), div![TITLE]],
                ],
                a![
                    C!["navbar-burger", menu_is_active],
                    attrs! {
                        At::AriaLabel => "menu",
                        At::AriaExpanded => &model.is_menu_open
                    },
                    span![attrs! { At::AriaHidden => "true" }],
                    span![attrs! { At::AriaHidden => "true" }],
                    span![attrs! { At::AriaHidden => "true" }],
                    ev(Ev::Click, |_| Msg::OpenOrCloseMenu),
                ],
            ]
        ),
        div![
            C!["navbar-menu", menu_is_active],
            IF!(model.page != Page::Slideshow && model.page != Page::ShareSlide =>
                div![C!("header-flex"),
                    a![
                        C![
                            "navbar-item",
                            "is-tab",
                            IF!(model.page == Page::MyAlbums => "is-active")
                        ],
                        attrs! { At::Href => format!("/{LK_MY_ALBUMS}") },
                        TITLE_MY_ALBUMS
                    ],
                    a![
                        C![
                            "navbar-item",
                            "is-tab",
                            IF!(model.page == Page::MySharings => "is-active")
                        ],
                        attrs! { At::Href => format!("/{LK_MY_SHARINGS}") },
                        TITLE_MY_SHARINGS
                    ],
                    div![
                        C!("navbar-item"),
                        div![
                            C!("buttons"),
                            a![
                                C!["button", "is-primary"],
                                attrs! { At::Href => format!("/{LK_NEW_ALBUM}") },
                                span![C!("icon"), i![C!("ion-plus")]],
                                span![TITLE_NEW_ALBUM],
                            ]
                        ]
                    ]
                ]
            ),
            div![
                C!("navbar-end"),
                div![
                    C!("navbar-item"),
                    if model.page == Page::Slideshow || model.page == Page::ShareSlide {
                        div![
                            C!("buttons"),
                            a![
                                C!["button", "is-primary", "is-link", "is-light", "is-small"],
                                span![C!("icon"), i![C!("ion-arrow-expand")]],
                                span!["Fullscreen"],
                                ev(Ev::Click, |_| Msg::Fullscreen),
                            ],
                            a![
                                C!["button", "is-primary", "is-link", "is-light", "is-small"],
                                attrs! { At::Href => lk_album },
                                span![C!("icon"), i![C!("ion-close-circled")]],
                                span!["Close"],
                            ],
                        ]
                    } else {
                        div![
                            C!("buttons"),
                            a![
                                C!["button", "is-light"],
                                if model.is_logged {
                                    "Sign out"
                                } else {
                                    "Sign in"
                                },
                                ev(Ev::Click, |_| Msg::ClickLogInOrOut),
                            ]
                        ]
                    }
                ]
            ]
        ]
    ]
}
