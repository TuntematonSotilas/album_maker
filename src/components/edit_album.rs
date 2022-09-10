use seed::{self, prelude::*, *};

use crate::{
    components::group,
    models::{
        album::Album, group::Group, group_update::UpdateType, page::{TITLE_NEW_ALBUM, TITLE_EDIT_ALBUM},
    }, api::api,
};

use super::notification::NotifType;

// ------ ------
//     Model
// ------ -----
pub struct Model {
	is_new: bool,
    auth_header: String,
    album: Album,
}

impl Model {
    pub const fn new() -> Self {
        Self {
			is_new: true,
            auth_header: String::new(),
            album: Album::new(),
        }
    }
    pub fn is_not_valid(&self) -> bool {
        if self.album.title.is_empty() {
            return true;
        }
        if let Some(groups) = &self.album.groups {
            return groups.iter().any(|g| g.title.is_empty());
        }
        false
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp(Option<String>),
	GetAlbum(String),
	ErrorGet,
    Received(Album),
    Submit,
    Success(String),
    TitleChanged(String),
    ShowNotif(NotifType, String),
    AddGroup,
    Group(group::Msg),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp(id_opt) => {
			match id_opt {
				Some(id) => { 
					model.is_new = false;
					orders.send_msg(Msg::GetAlbum(id)); 
				},
				None => { model.album = Album::new(); }
			}
		}
		Msg::GetAlbum(id) => {
			orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
            orders.perform_cmd(async {
                let opt_album = api::get_album(id, auth).await;
				match opt_album {
					Some(album) => Msg::Received(album),
					None => Msg::ErrorGet,
				} 
            });
		}
		Msg::ErrorGet => {
            error!("Error getting albums");
        }
        Msg::Received(album) => {
            model.album = album;
        }
        Msg::Submit => {
            orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
			let album = model.album.clone();
            orders.perform_cmd(async {
				let opt_id = api::update_album(album, auth).await;
				match opt_id {
					Some(id) => Msg::Success(id),
					None => Msg::ShowNotif(NotifType::Error, "Error when saving".to_string())
				}
			});
        }
        Msg::Success(id) => {
            model.album.id = id;
            orders.send_msg(Msg::ShowNotif(
                NotifType::Success,
                "Album saved".to_string(),
            ));
        }
        Msg::TitleChanged(title) => model.album.title = title,
        Msg::ShowNotif(_, _) => (),
        Msg::AddGroup => {
            if let Some(groups) = &mut model.album.groups {
                groups.push(Group::new());
            }
        }
        Msg::Group(msg) => {
            if let group::Msg::UpdateGroup(ref group_update) = msg {
                if let Some(groups) = &mut model.album.groups {
                    if let Some(group) = groups.iter_mut().find(|g| g.id == group_update.id) {
                        let grp_upd = group_update.clone();
                        match group_update.upd_type {
                            UpdateType::CountFakePictures => {
                                group.count_fake_pictures =
                                    grp_upd.count_fake_pictures.unwrap_or_default();
                            }
                            UpdateType::Title => {
                                group.title = grp_upd.grp_data.unwrap_or_default();
                            }
							UpdateType::Description => {
                                group.description = grp_upd.grp_data.unwrap_or_default();
                            }
                            UpdateType::AddPicture => {
                                if let Some(picture) = grp_upd.picture {
                                    if let Some(pictures) = &mut group.pictures {
                                        pictures.push(picture);
                                        group.count_fake_pictures -= 1;
                                    }
                                }
                            }
                            UpdateType::Caption => {
                                if let Some(pictures) = &mut group.pictures {
                                    if let Some(picture) = pictures.iter_mut().find(|p| {
                                        p.asset_id
                                            == group_update.clone().asset_id.unwrap_or_default()
                                    }) {
                                        picture.caption =
                                            Some(group_update.clone().caption.unwrap_or_default());
                                    }
                                }
                            }
							UpdateType::DeletePicture => {
                                if let Some(pictures) = &mut group.pictures {
									if let Some(pos) = pictures.iter().position(|p| 
										p.asset_id == group_update.clone().asset_id.unwrap_or_default()) {
											pictures.remove(pos);
									}
                                }
                            }
                        }
                    }
                }
            }
            group::update(msg, &mut orders.proxy(Msg::Group));
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
            p![C!["title", "is-5", "has-text-link"], 
				match model.is_new { 
					true => TITLE_NEW_ALBUM,
					false => TITLE_EDIT_ALBUM
				},
			],
			label![C!("label"), "Album name"],
			div![
                C!["field", "has-addons"],
				div![
                    C!("control"),
                    input![
                        C!["input", "is-small", IF!(model.album.title.is_empty() => "is-danger")],
                        attrs! {
                            At::Type => "text",
                            At::Name => "title",
                            At::Placeholder => "Album name",
                            At::Value => model.album.title,
                        },
                        input_ev(Ev::Input, Msg::TitleChanged),
                    ]
                ],
                div![
                    C!("control"),
                    button![
                        C!["button", "is-primary", "is-small"],
                        "Save",
                        ev(Ev::Click, |_| Msg::Submit),
                        attrs! { At::Disabled => model.is_not_valid().as_at_value() },
                    ]
                ]
            ]
        ],
        match &model.album.groups {
            Some(groups) => div![groups
                .iter()
                .map(|group| { group::view(group.clone()).map_msg(Msg::Group) })],
            None => empty![],
        },
        div![
            C!["mt-5"],
            button![
                C!["button", "is-link", "is-light", "is-small"],
                span![C!("icon"), i![C!("ion-plus")]],
                span!["Add group"],
                ev(Ev::Click, |_| Msg::AddGroup),
            ],
        ],
    ]
}
