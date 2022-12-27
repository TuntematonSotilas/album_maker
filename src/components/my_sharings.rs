use seed::{self, prelude::*, *};
use crate::{models::{sharing::Sharing, notif::{Notif, TypeNotifs}, page::TITLE_MY_SHARINGS}, api::sharingapi};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
    auth_header: String,
    sharings: Option<Vec<Sharing>>
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp,
    Received(Vec<Sharing>),
    ErrorGet,
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
            p![C!["title", "is-5", "has-text-link"], TITLE_MY_SHARINGS],
            if model.sharings.is_some() {
                div![model.sharings.as_ref().unwrap().iter().map(|sharing| {
                    p![
                        C!("panel-block"),
                        div![
                            C!["container", "is-flex", "is-justify-content-space-between"],
                            div![&sharing.album_name],
                            div![
                                C!["is-align-content-flex-end"],
                                button![
                                    C!["button", "is-link", "is-light", "is-small"],
                                    span![C!("icon"), i![C!("ion-close-circled")]],
                                    span!["Delete"],
                                    //ev(Ev::Click, |_| Msg::AskDelete(id_del)),
                                ]
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
