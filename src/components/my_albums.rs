use seed::{self, prelude::*, *};

use crate::{
    api::apifn,
    models::{
        album::Album,
        notif::{Notif, TypeNotifs},
        page::{LK_VIEW_ALBUM, TITLE_MY_ALBUMS},
    },
};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
    auth_header: String,
    albums: Option<Vec<Album>>,
    album_id_to_delete: Option<String>,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp,
    Received(Vec<Album>),
    ErrorGet,
    Delete(String),
    AskDelete(String),
    SuccessDelete(String),
    ErrorDelete,
    CancelDelete,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp => {
            orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
            orders.perform_cmd(async {
                let albums_opt = apifn::get_my_ablums(auth).await;
                match albums_opt {
                    Some(albums) => Msg::Received(albums),
                    None => Msg::ErrorGet,
                }
            });
        }
        Msg::ErrorGet => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Success,
                message: "Error getting albums".to_string(),
            });
        }
        Msg::Received(albums) => {
            model.albums = Some(albums);
        }
        Msg::AskDelete(id) => {
            model.album_id_to_delete = Some(id);
        }
        Msg::CancelDelete => {
            model.album_id_to_delete = None;
        }
        Msg::Delete(id) => {
            orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
            let id_del = id.clone();
            let id_suc = id;
            orders.perform_cmd(async {
                let success = apifn::delete_ablum(id_del, auth).await;
                if success {
                    Msg::SuccessDelete(id_suc)
                } else {
                    Msg::ErrorDelete
                }
            });
        }
        Msg::ErrorDelete => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Success,
                message: "Error deleting album".to_string(),
            });
        }
        Msg::SuccessDelete(id) => {
            if let Some(albums) = &mut model.albums {
                let index = albums.iter().position(|album| *album.id == id).unwrap();
                albums.remove(index);
            }
        }
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["column", "is-centered", "is-half"],
        div![
            C!("box"),
            p![C!["title", "is-5", "has-text-link"], TITLE_MY_ALBUMS],
            if model.albums.is_some() {
                div![model.albums.as_ref().unwrap().iter().map(|album| {
                    let id = album.id.clone();
                    let album_id_to_delete = model.album_id_to_delete.clone();
                    let is_ask =
                        model.album_id_to_delete.is_some() && album_id_to_delete.unwrap() == id;
                    p![
                        C!("panel-block"),
                        div![
                            C!["container", "is-flex", "is-justify-content-space-between"],
                            div![if is_ask {
                                span!["Delete this album ?"]
                            } else {
                                a![
                                    attrs! {
                                        At::Title => "Open",
                                        At::Href => format!("/{}/{}", LK_VIEW_ALBUM, id),
                                    },
                                    &album.title
                                ]
                            }],
                            div![
                                C!["is-align-content-flex-end"],
                                if is_ask {
                                    div![
                                        button![
                                            C!["button", "is-link", "is-light", "is-small", "mr-2"],
                                            span![C!("icon"), i![C!("ion-close-circled")]],
                                            span!["NO"],
                                            ev(Ev::Click, |_| Msg::CancelDelete),
                                        ],
                                        button![
                                            C!["button", "is-danger", "is-light", "is-small"],
                                            span![C!("icon"), i![C!("ion-close-circled")]],
                                            span!["YES"],
                                            ev(Ev::Click, |_| Msg::Delete(id)),
                                        ]
                                    ]
                                } else {
                                    button![
                                        C!["button", "is-link", "is-light", "is-small"],
                                        span![C!("icon"), i![C!("ion-close-circled")]],
                                        span!["Delete"],
                                        ev(Ev::Click, |_| Msg::AskDelete(id)),
                                    ]
                                }
                            ]
                        ]
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
