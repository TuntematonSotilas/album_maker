use seed::{self, prelude::*, *};

use crate::{
    components::group,
    models::{album::Album, group::Group, page::TITLE_NEW_ALBUM, vars::BASE_URI},
};

use super::{group::UpdateType, notification::NotifType};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    auth_header: String,
    album: Album,
    group: group::Model,
}

impl Model {
    pub fn new() -> Self {
        Self {
            auth_header: String::new(),
            album: Album::new(),
            group: group::Model::new(),
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
    InitComp,
    Submit,
    Success(String),
    TitleChanged(String),
    ShowNotif(NotifType, String),
    AddGroup,
    Group(group::Msg),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => {
            model.auth_header = auth_header;
        }
        Msg::InitComp => {
            model.album = Album::new();
            model.group = group::Model::new();
        }
        Msg::Submit => {
            orders.skip(); // No need to rerender
            let uri = BASE_URI.to_string() + "editalbum";
            let auth = model.auth_header.clone();
            let request = Request::new(uri)
                .method(Method::Put)
                .header(Header::authorization(auth))
                .json(&model.album)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");

                if response.status().is_ok() {
                    let res_id = response.json::<String>().await;
                    if let Ok(id) = res_id {
                        Msg::Success(id)
                    } else {
                        Msg::ShowNotif(NotifType::Error, "Error when saving".to_string())
                    }
                } else {
                    Msg::ShowNotif(NotifType::Error, "Error when saving".to_string())
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
            match msg {
                group::Msg::UpdateGroup(ref group_update, update_type) => {
                    if let Some(groups) = &mut model.album.groups {
                        if let Some(group) = groups.iter_mut().find(|g| g.id == group_update.id) {
                            let grp_upd = group_update.clone();
                            match update_type {
                                UpdateType::CountFakePictures => {
                                    group.count_fake_pictures = grp_upd.count_fake_pictures;
                                }
                                UpdateType::Title => group.title = grp_upd.title,
                            }
                        }
                    }
                }
                group::Msg::AddPicture(ref picture, group_id) => {
                    if let Some(groups) = &mut model.album.groups {
                        if let Some(group) = groups.iter_mut().find(|g| g.id == group_id) {
                            if let Some(pictures) = &mut group.pictures {
                                pictures.push(picture.clone());
                                group.count_fake_pictures -= 1;
                            }
                        }
                    }
                }
                _ => (),
            }
            group::update(msg, &mut model.group, &mut orders.proxy(Msg::Group));
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
            p![C!["title", "is-5", "has-text-link"], TITLE_NEW_ALBUM],
            div![
                C!["field", "has-addons"],
                div![
                    C!("control"),
                    input![
                        C!["input", IF!(model.album.title.is_empty() => "is-danger")],
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
                        C!["button", "is-primary"],
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
