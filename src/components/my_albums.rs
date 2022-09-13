use std::collections::HashMap;

use seed::{self, prelude::*, *};

use crate::{
    api::apifn,
    models::{
        album::Album,
        notif::{Notif, TypeNotifs},
        page::{LK_VIEW_ALBUM, TITLE_MY_ALBUMS},
    },
};

#[derive(PartialEq)]
enum State {
	AskDelete,
	Deleting
}
// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
    auth_header: String,
    albums: Option<Vec<Album>>,
	states: HashMap<String, State>,
	total_pic_to_delete: usize,
	nb_pic_deleted: i32,
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
    ErrorDelete,
    CancelDelete(String),
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
			model.states.entry(id).or_insert(State::AskDelete);
        }
        Msg::CancelDelete(id) => {
            model.states.remove(&id);
        }
        Msg::DeleteAllPics(id) => {
			
            let id = id.clone();
			let id_f = id.clone();
			let id_del = id.clone();

			model.states.entry(id).or_insert(State::Deleting);

			let entry = model.states.get(&id_f);
			if entry.is_some() {
				let e = entry.unwrap();
				match e {
					State::Deleting => log!("Deleting"),
					State::AskDelete => log!("Deleting"),
				}
			} else {
				log!("none");
			}

			orders.force_render_now();

			/*orders.skip(); // No need to rerender

			//Delete all pictures
			if let Some(albums) = model.albums.clone() {
				if let Some(album) = albums.iter().find(|a| a.id == id_f ) {
					if let Some(groups) = album.groups.clone() {
						let grp_pic_ids = groups.iter().map(|g| {
							if let Some(pictures) = g.pictures.clone() {
								let pic_p_ids:Vec<String> = pictures.iter().map(|p| p.public_id.clone()).collect();
								log!("a-", pic_p_ids);
								pic_p_ids
							} else {
								Vec::new()
							}
						});
						let pic_ids: Vec<String> = grp_pic_ids.into_iter().flatten().collect();
						model.total_pic_to_delete = pic_ids.len();
						log!(pic_ids);

						orders.perform_cmd(async {
							let mut all_success = true;
							for pic_id in pic_ids {
								let pic_id = pic_id.clone();
								let res = apifn::delete_picture(pic_id).await;
								if res {
									all_success = true;
									//model.nb_pic_deleted += 1;
								} else {
									all_success = false;
									break;
								}
							}
							if all_success {
								log("all_success");
								Msg::DeleteAlbum(id_del)
							} else {
								Msg::ErrorDelete
							}
						});
						
					}
				}
			}*/
			
			
        }
		Msg::DeleteAlbum(id) => {
			let auth = model.auth_header.clone();
			let id_del = id.clone();
			orders.perform_cmd(async {
                let success = apifn::delete_ablum(id_del, auth).await;
                if success {
                    Msg::SuccessDelete(id)
                } else {
                    Msg::ErrorDelete
                }
            });
		}
        Msg::ErrorDelete => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error deleting album".to_string(),
            });
        }
        Msg::SuccessDelete(id) => {
            if let Some(albums) = &mut model.albums {
                let index = albums.iter().position(|album| *album.id == id).unwrap();
                //albums.remove(index);
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
                    let id_del = album.id.clone();
					let id_can = album.id.clone();
                    let state = model.states.get(&id_del);
                    p![
                        C!("panel-block"),
                        div![
                            C!["container", "is-flex", "is-justify-content-space-between"],
                            div![
								if state.is_some() {
									match state.unwrap() {
										State::AskDelete => {
											span!["Delete this album ?"]
										},
										State::Deleting => {
											progress![
												C!["progress", "is-danger"],
												attrs! { At::Value => "5", At::Max => "10" },
												"5"
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
                                if state.is_some() {
									if state.unwrap() == &State::AskDelete {
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
