use crate::models::page::{Page, LK_MY_ALBUMS, LK_NEW_ALBUM, TITLE_MY_ALBUMS, TITLE_NEW_ALBUM};
use seed::{self, prelude::*, *};

const TITLE: &str = "Album maker";

// ------ ------
//     Model
// ------ -----
pub struct Model {
    is_menu_open: bool,
    page: Page,
    is_logged: bool,
}

impl Model {
    pub const fn new(page: Page) -> Self {
        Self {
            is_menu_open: false,
            page,
            is_logged: false,
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
        Msg::ClickLogInOrOut => {
            if model.is_logged {
                model.is_logged = false;
            }
            orders.send_msg(Msg::LogInOrOut);
        }
        Msg::LogInOrOut => (),
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
    let menu_is_active = match &model.is_menu_open {
        true => "is-active",
        false => "",
    };
    nav![
        C!("navbar"),
        attrs! { At::AriaLabel => "main navigation" },
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
        ],
        div![
            C!["navbar-menu", menu_is_active],
            a![
                C![
                    "navbar-item",
                    "is-tab",
                    IF!(matches!(model.page, Page::MyAlbums) => "is-active")
                ],
                attrs! { At::Href => format!("/{}", LK_MY_ALBUMS) },
                TITLE_MY_ALBUMS
            ],
            div![
                C!("navbar-item"),
                div![
                    C!("buttons"),
                    a![
                        C!["button", "is-primary"],
                        attrs! { At::Href => format!("/{}", LK_NEW_ALBUM) },
                        span![C!("icon"), i![C!("ion-plus")]],
                        span![TITLE_NEW_ALBUM],
                    ],
                ],
            ],
            div![
                C!("navbar-end"),
                div![
                    C!("navbar-item"),
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
                ]
            ]
        ]
    ]
}
