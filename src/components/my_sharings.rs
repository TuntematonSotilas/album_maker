use crate::{
    api::sharingapi,
    models::{
        notif::{Notif, TypeNotifs},
        page::TITLE_MY_SHARINGS,
        sharing::Sharing,
    },
};
use seed::{self, prelude::*, *};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
    auth_header: String,
    sharings: Option<Vec<Sharing>>,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp,
    Received(Vec<Sharing>),
    ErrorGet,
    Delete(String),
    SuccessDelete(String),
    ErrorDelete,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp => {
            orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
            orders.perform_cmd(async {
                let sharings_opt = sharingapi::get_my_sharings(auth).await;
                sharings_opt.map_or(Msg::ErrorGet, Msg::Received)
            });
        }
        Msg::ErrorGet => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error getting sharings".to_string(),
            });
        }
        Msg::Received(sharings) => {
            model.sharings = Some(sharings);
        }
        Msg::Delete(id) => {
            let auth = model.auth_header.clone();
            let id_del = id.clone();
            orders.perform_cmd(async {
                let success = sharingapi::delete_sharing(id_del, auth).await;
                if success {
                    Msg::SuccessDelete(id)
                } else {
                    Msg::ErrorDelete
                }
            });
        }
        Msg::SuccessDelete(id) => {
            if let Some(sharings) = &mut model.sharings {
                let index = sharings.iter().position(|s| *s.id == id).unwrap();
                sharings.remove(index);
            }
        }
        Msg::ErrorDelete => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error deleting sharing".to_string(),
            });
        }
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["column", "is-centered", "is-three-fifths"],
        div![
            C!("box"),
            p![C!["title", "is-5", "has-text-link"], TITLE_MY_SHARINGS],
            if model.sharings.is_some() {
                div![model.sharings.as_ref().unwrap().iter().map(|sharing| {
                    let base_url = web_sys::window().unwrap().location().origin().unwrap();
                    let id_del = sharing.id.clone();
                    let id_del_mob = sharing.id.clone();
                    div![
                        p![
                            C!("panel-block"),
                            div![
                                C![
                                    "container",
                                    "is-flex",
                                    "is-justify-content-space-between",
                                    "is-align-items-center"
                                ],
                                div![&sharing.album_name],
                                div![
                                    C!("is-flex"),
                                    div![
                                        C!["tag", "is-link", "is-light", "ml-2"],
                                        attrs! {At::Title => "Number of views"},
                                        span![C!("icon"), i![C!("ion-eye")]],
                                        &sharing.nb_view
                                    ],
                                    div![
                                        C!["tag", "is-danger", "is-light", "ml-2"],
                                        attrs! {At::Title => "Number of likes"},
                                        span![C!("icon"), i![C!("ion-heart")]],
                                        &sharing.nb_like
                                    ],
                                ],
                                div![
                                    C!["has-text-grey", "is-size-7", "ml-2", "is-hidden-mobile"],
                                    format!("{base_url}/share/{}", &sharing.id)
                                ],
                                div![
                                    C!["is-align-content-flex-end", "is-hidden-mobile"],
                                    button![
                                        C!["button", "is-link", "is-light", "is-small", "ml-2"],
                                        span![C!("icon"), i![C!("ion-close-circled")]],
                                        span!["Delete"],
                                        ev(Ev::Click, |_| Msg::Delete(id_del)),
                                    ]
                                ]
                            ]
                        ],
                        p![
                            C!["panel-block", "is-hidden-desktop", "is-hidden-tablet"],
                            div![
                                C!["has-text-grey", "is-size-7"],
                                format!("{base_url}/share/{}", &sharing.id)
                            ],
                            div![
                                C!["is-align-content-flex-end"],
                                button![
                                    C!["button", "is-link", "is-light", "is-small", "ml-2"],
                                    span![C!("icon"), i![C!("ion-close-circled")]],
                                    span!["Delete"],
                                    ev(Ev::Click, |_| Msg::Delete(id_del_mob)),
                                ]
                            ]
                        ],
                    ]
                })]
            } else {
                div![(0..4).map(|_| {
                    p![
                        C!("panel-block"),
                        progress![
                            C!["progress", "is-small", "table-progress"],
                            attrs! { At::Max => 100 }
                        ],
                    ]
                })]
            }
        ]
    ]
}
