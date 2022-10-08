use std::collections::HashMap;

use seed::{self, prelude::*, *};
use uuid::Uuid;

use crate::{
    api::apifn,
    components::group,
    models::{
        album::Album,
        group::Group,
        group_update::{GroupUpdate, UpdateType},
        notif::{Notif, TypeNotifs},
        page::{TITLE_EDIT_ALBUM, TITLE_NEW_ALBUM},
        state::{State, TypeDel},
    },
};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    is_new: bool,
    auth_header: String,
    album: Album,
    states: HashMap<String, State>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            is_new: true,
            auth_header: String::new(),
            album: Album::new(),
            states: HashMap::new(),
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
    TitleChanged(String),
    AddGroup,
    Group(group::Msg),
    NotifySuccess(String),
    NotifyError,
    DeleteGroup(Uuid),
    ErrorDeleteOnePic,
    SuccessDeleteOnePic(String),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp(id_opt) => match id_opt {
            Some(id) => {
                model.is_new = false;
                orders.send_msg(Msg::GetAlbum(id));
            }
            None => {
                model.album = Album::new();
            }
        },
        Msg::GetAlbum(id) => {
            orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
            orders.perform_cmd(async {
                let opt_album = apifn::get_album(id, auth).await;
                match opt_album {
                    Some(album) => Msg::Received(album),
                    None => Msg::ErrorGet,
                }
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
        }
        Msg::Submit => {
            orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
            let album = model.album.clone();
            orders.perform_cmd(async {
                let opt_id = apifn::update_album(album, auth).await;
                match opt_id {
                    Some(id) => Msg::NotifySuccess(id),
                    None => Msg::NotifyError,
                }
            });
        }
        Msg::NotifySuccess(id) => {
            model.album.id = id;
            orders.notify(Notif {
                notif_type: TypeNotifs::Success,
                message: "Album saved".to_string(),
            });
        }
        Msg::NotifyError => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Login error".to_string(),
            });
        }
        Msg::TitleChanged(title) => model.album.title = title,
        Msg::AddGroup => {
            if let Some(groups) = &mut model.album.groups {
                groups.push(Group::new());
            }
        }
        Msg::Group(msg) => {
            match msg {
                group::Msg::UpdateGroup(ref group_update) => {
                    if let Some(groups) = &mut model.album.groups {
                        update_group(group_update, groups);
                    }
                }
                group::Msg::BeginDeleteGroup(id) => {
                    model.states.insert(
                        id.to_string(),
                        State {
                            del_state: TypeDel::Deleting,
                            total: 0,
                            current: 0,
                        },
                    );
                    orders.send_msg(Msg::DeleteGroup(id));
                }
                _ => (),
            }
            group::update(msg, &mut orders.proxy(Msg::Group));
        }
        Msg::DeleteGroup(id) => delete_group(model, orders, id),
        Msg::SuccessDeleteOnePic(id) => {
            if let Some(delete_state) = model.states.get_mut(&id) {
                delete_state.current += 1;
            }
        }
        Msg::ErrorDeleteOnePic => {
            error!("Error deleting picture");
        }
    }
}

fn delete_group(model: &mut Model, orders: &mut impl Orders<Msg>, group_id: Uuid) {
    let album_id = model.album.id.clone();
    if let Some(groups) = &mut model.album.groups {
        if let Some(group) = groups.iter().find(|g| g.id == group_id) {
            // Delete all pics
            let pic_ids = group.pictures.clone().map_or_else(Vec::new, |pictures| {
                pictures.iter().map(|p| p.public_id.clone()).collect()
            });
            for pic_id in pic_ids {
                let album_id = album_id.clone();
                orders.perform_cmd(async move {
                    let album_id = album_id.clone();
                    let res = apifn::delete_picture(pic_id).await;
                    if res {
                        Msg::SuccessDeleteOnePic(album_id)
                    } else {
                        Msg::ErrorDeleteOnePic
                    }
                });
            }
        }
        if let Some(index) = groups.iter().position(|g| g.id == group_id) {
            groups.remove(index);
        }
    }
}

fn update_group(group_update: &GroupUpdate, groups: &mut [Group]) {
    if let Some(group) = groups.iter_mut().find(|g| g.id == group_update.id) {
        let grp_upd = group_update.clone();
        match group_update.upd_type {
            UpdateType::CountFakePictures => {
                group.count_fake_pictures = grp_upd.count_fake_pictures.unwrap_or_default();
            }
            UpdateType::Title => {
                group.title = grp_upd.grp_data.unwrap_or_default();
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
                    if let Some(picture) = pictures
                        .iter_mut()
                        .find(|p| p.asset_id == group_update.clone().asset_id.unwrap_or_default())
                    {
                        picture.caption = Some(group_update.clone().caption.unwrap_or_default());
                    }
                }
            }
            UpdateType::DeletePicture => {
                if let Some(pictures) = &mut group.pictures {
                    if let Some(pos) = pictures.iter().position(|p| {
                        p.asset_id == group_update.clone().asset_id.unwrap_or_default()
                    }) {
                        pictures.remove(pos);
                    }
                }
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
            p![
                C!["title", "is-5", "has-text-link"],
                if model.is_new {
                    TITLE_NEW_ALBUM
                } else {
                    TITLE_EDIT_ALBUM
                }
            ],
            label![C!("label"), "Album name"],
            div![
                C!["field", "has-addons"],
                div![
                    C!("control"),
                    input![
                        C![
                            "input",
                            "is-small",
                            IF!(model.album.title.is_empty() => "is-danger")
                        ],
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
            Some(groups) => div![groups.iter().map(|group| {
                let state_opt = model.states.get(&group.id.to_string());
                group::view(model.album.id.clone(), group.clone(), state_opt).map_msg(Msg::Group)
            })],
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
