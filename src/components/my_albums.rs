use std::collections::HashMap;

use seed::{self, prelude::*, *};

use crate::{
    api::apifn,
    models::{
        album::Album,
        notif::{Notif, TypeNotifs},
        page::{LK_VIEW_ALBUM, TITLE_MY_ALBUMS},
        state::{State, TypeDel},
    },
};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
    auth_header: String,
    albums: Option<Vec<Album>>,
    states: HashMap<String, State>,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp,
    Received(Vec<Album>),
    ErrorGet,
    DeleteAllPics(String),
    DeleteAlbum(String),
    AskDelete(String),
    SuccessDelete(String),
    ErrorDelete(String),
    CancelDelete(String),
    SuccessDeleteOnePic(String),
    ErrorDeleteOnePic,
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
                notif_type: TypeNotifs::Error,
                message: "Error getting albums".to_string(),
            });
        }
        Msg::Received(albums) => {
            model.albums = Some(albums);
        }
        Msg::AskDelete(id) => {
            model.states.insert(
                id,
                State {
                    del_state: TypeDel::AskDelete,
                    total: 0,
                    current: 0,
                },
            );
        }
        Msg::CancelDelete(id) => {
            model.states.remove(&id);
        }
        Msg::DeleteAllPics(album_id) => delete_all_pics(model, orders, album_id.as_str()),
        Msg::ErrorDeleteOnePic => {
            error!("Error deleting picture");
        }
        Msg::SuccessDeleteOnePic(id) => {
            if let Some(delete_state) = model.states.get_mut(&id) {
                delete_state.current += 1;
            }
        }
        Msg::DeleteAlbum(id) => {
            let auth = model.auth_header.clone();
            let id_del = id.clone();
            orders.perform_cmd(async {
                let success = apifn::delete_ablum(id_del, auth).await;
                if success {
                    Msg::SuccessDelete(id)
                } else {
                    Msg::ErrorDelete(id)
                }
            });
        }
        Msg::ErrorDelete(id) => {
            model.states.remove(&id);
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
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

fn delete_all_pics(model: &mut Model, orders: &mut impl Orders<Msg>, album_id: &str) {
    if let Some(delete_state) = model.states.get_mut(album_id) {
        delete_state.del_state = TypeDel::Deleting;
        //Delete all pictures
        if let Some(albums) = model.albums.clone() {
            if let Some(album) = albums.iter().find(|a| a.id == album_id) {
                if let Some(groups) = album.groups.clone() {
                    let grp_pic_ids = groups.iter().map(|g| {
                        g.pictures.clone().map_or_else(Vec::new, |pictures| {
                            pictures.iter().map(|p| p.public_id.clone()).collect()
                        })
                    });
                    let pic_ids: Vec<String> = grp_pic_ids.into_iter().flatten().collect();
                    delete_state.total = pic_ids.len();

                    for pic_id in pic_ids {
                        let id_success = album_id.to_string();
                        orders.perform_cmd(async move {
                            let res = apifn::delete_picture(pic_id).await;
                            if res {
                                Msg::SuccessDeleteOnePic(id_success)
                            } else {
                                Msg::ErrorDeleteOnePic
                            }
                        });
                    }
                }
            }
        }
        orders.send_msg(Msg::DeleteAlbum(album_id.to_string()));
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
                    let id_del = album.id.clone();
					let id_can = album.id.clone();
                    let state_opt = model.states.get(&id_del);
                    p![
                        C!("panel-block"),
                        div![
                            C!["container", "is-flex", "is-justify-content-space-between"],
                            div![
								if state_opt.is_some() {
									let state = state_opt.unwrap();
									match state.del_state {
										TypeDel::AskDelete => {
											span!["Delete this album ?"]
										},
										TypeDel::Deleting => {
											progress![
												C!["progress", "is-danger"],
												attrs! { At::Value => state.current, At::Max => state.total }
											]
										}
									}
								}
								else {
									a![
										attrs! {
											At::Title => "Open",
											At::Href => format!("/{}/{}", LK_VIEW_ALBUM, id_del),
										},
										&album.title
									]
								}
							],
                            div![
                                C!["is-align-content-flex-end"],
                                if state_opt.is_some() {
									if state_opt.unwrap().del_state == TypeDel::AskDelete {
										div![
											button![
												C!["button", "is-link", "is-light", "is-small", "mr-2"],
												span![C!("icon"), i![C!("ion-close-circled")]],
												span!["NO"],
												ev(Ev::Click, |_| Msg::CancelDelete(id_del)),
											],
											button![
												C!["button", "is-danger", "is-light", "is-small"],
												span![C!("icon"), i![C!("ion-close-circled")]],
												span!["YES"],
												ev(Ev::Click, |_| Msg::DeleteAllPics(id_can)),
											]
										]
									} else {
										empty!()
									}
                                } else {
                                    button![
                                        C!["button", "is-link", "is-light", "is-small"],
                                        span![C!("icon"), i![C!("ion-close-circled")]],
                                        span!["Delete"],
                                        ev(Ev::Click, |_| Msg::AskDelete(id_del)),
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
