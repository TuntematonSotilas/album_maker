use seed::{self, prelude::*, *};
use uuid::Uuid;

use crate::{
    api::albumapi,
    components::group,
    models::{
        album::Album,
        caption::{Color, Style, COLORS},
        group::Group,
        group_update::{GroupUpdate, UpdateType},
        notif::{Notif, TypeNotifs},
        page::{TITLE_EDIT_ALBUM, TITLE_NEW_ALBUM},
        state::{DeleteStatus, State},
    },
};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    is_new: bool,
    auth_header: String,
    album: Album,
    id_pic_drag: String,
}

impl Model {
    pub const fn new() -> Self {
        Self {
            is_new: true,
            auth_header: String::new(),
            album: Album::new(),
            id_pic_drag: String::new(),
        }
    }
    pub fn is_not_valid(&self) -> bool {
        if self.album.title.is_empty() {
            return true;
        }
        let groups = self.album.groups.clone().unwrap_or_default();
        return groups.iter().any(|g| g.title.is_empty());
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
    StyleChanged(Style),
    ColorChanged(Color),
    AddGroup,
    Group(group::Msg),
    NotifySuccess(String),
    NotifyError,
    DeleteGroup(Uuid),
    ErrorDeleteOnePic,
    SuccessDeleteOnePic(Uuid),
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
                let opt_album = albumapi::get_album(Some(id), None, auth).await;
                opt_album.map_or(Msg::ErrorGet, Msg::Received)
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
                let opt_id = albumapi::update_album(album, auth).await;
                opt_id.map_or(Msg::NotifyError, Msg::NotifySuccess)
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
        Msg::StyleChanged(style) => model.album.caption_style = style,
        Msg::ColorChanged(color) => model.album.caption_color = color,
        Msg::AddGroup => {
            if let Some(groups) = &mut model.album.groups {
                groups.push(Group::new());
            }
        }
        Msg::Group(msg) => {
            match msg {
                group::Msg::UpdateGroup(ref group_update) => {
                    update_group(group_update, &mut model.album, orders);
                }
                group::Msg::DragEnded(ref id_pic_drag) => {
                    model.id_pic_drag = id_pic_drag.clone();
                }
                group::Msg::Drop(group_id, ref id_pic_drop) => {
                    drop_pic(model, group_id, id_pic_drop);
                }
                _ => (),
            }
            group::update(msg, &mut orders.proxy(Msg::Group));
        }
        Msg::DeleteGroup(id) => delete_group(model, orders, id),
        Msg::SuccessDeleteOnePic(group_id) => {
            let group_update = GroupUpdate {
                upd_type: UpdateType::DeleteState,
                id: group_id,
                picture: None,
                grp_data: None,
                count_fake_pictures: None,
                asset_id: None,
                caption: None,
                delete_status: Some(DeleteStatus::Deleting),
            };
            update_group(&group_update, &mut model.album, orders);
        }
        Msg::ErrorDeleteOnePic => {
            error!("Error deleting picture");
        }
    }
}

fn drop_pic(model: &mut Model, group_id: Uuid, id_pic_drop: &str) {
    let id_pic_drag = &model.id_pic_drag;
    if let Some(groups) = &mut model.album.groups {
        if let Some(group) = groups.iter_mut().find(|g| g.id == group_id) {
            if let Some(pictures) = &mut group.pictures {
                let pos1 = pictures.iter().position(|p| p.asset_id == *id_pic_drag);
                let pos2 = pictures.iter().position(|p| p.asset_id == id_pic_drop);
                if let (Some(pos1), Some(pos2)) = (pos1, pos2) {
                    pictures.swap(pos1, pos2);
                }
            }
        }
    }
}

fn delete_group(model: &mut Model, orders: &mut impl Orders<Msg>, group_id: Uuid) {
    if let Some(groups) = &mut model.album.groups {
        if let Some(group) = groups.iter().find(|g| g.id == group_id) {
            // Delete all pics
            let pic_ids = group.pictures.clone().map_or_else(Vec::new, |pictures| {
                pictures.iter().map(|p| p.public_id.clone()).collect()
            });
            for pic_id in pic_ids {
                orders.perform_cmd(async move {
                    let res = albumapi::delete_picture(pic_id).await;
                    if res {
                        Msg::SuccessDeleteOnePic(group_id)
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

fn update_group(group_update: &GroupUpdate, album: &mut Album, orders: &mut impl Orders<Msg>) {
    if let Some(groups) = &mut album.groups {
        if let Some(group) = groups.iter_mut().find(|g| g.id == group_update.id) {
            let grp_upd = group_update.clone();
            match group_update.upd_type {
                UpdateType::CountFakePictures => {
                    group.count_fake_pictures = grp_upd.count_fake_pictures.unwrap_or_default();
                }
                UpdateType::Title => {
                    group.title = grp_upd.grp_data.unwrap_or_default();
                }
                UpdateType::SetAlbumCover => {
                    if album.cover == grp_upd.asset_id.unwrap_or_default() {
                        album.cover = String::new();
                    } else {
                        album.cover = group_update.clone().asset_id.unwrap_or_default();
                    }
                }
                UpdateType::SetGroupCover => {
                    if group.cover == grp_upd.asset_id.unwrap_or_default() {
                        group.cover = String::new();
                    } else {
                        group.cover = group_update.clone().asset_id.unwrap_or_default();
                    }
                }
                UpdateType::AddPicture => {
                    let picture = grp_upd.picture.unwrap_or_default();
                    if let Some(pictures) = &mut group.pictures {
                        pictures.push(picture);
                        group.count_fake_pictures -= 1;
                    }
                }
                UpdateType::Caption => {
                    if let Some(pictures) = &mut group.pictures {
                        if let Some(picture) = pictures
                            .iter_mut()
                            .find(|p| p.asset_id == grp_upd.clone().asset_id.unwrap_or_default())
                        {
                            picture.caption = grp_upd.caption;
                        }
                    }
                }
                UpdateType::DeletePicture => {
                    if let Some(pictures) = &mut group.pictures {
                        if let Some(pos) = pictures.iter().position(|p| {
                            p.asset_id == grp_upd.clone().asset_id.unwrap_or_default()
                        }) {
                            pictures.remove(pos);
                        }
                    }
                }
                UpdateType::DeleteState => {
                    if let Some(del_state) = &group_update.delete_status {
                        let mut total = 0;
                        if let Some(pictures) = &mut group.pictures {
                            total = pictures.len();
                        }
                        let mut current = 0;
                        if let Some(state) = &mut group.state {
                            current = state.current + 1;
                        } else {
                            orders.send_msg(Msg::DeleteGroup(group.id));
                        }
                        match del_state {
                            DeleteStatus::Deleting => {
                                group.state = Some(State {
                                    delete_status: DeleteStatus::Deleting,
                                    total,
                                    current,
                                });
                            }
                            DeleteStatus::AskDelete => (),
                        }
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
                            At::Name => "Album name",
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
            ],
            caption_view(model),
        ],
        &model
            .album
            .groups
            .as_ref()
            .map_or(empty!(), |groups| div![groups.iter().map(|group| {
                group::view(model.album.id.clone(), model.album.cover.as_str(), group)
                    .map_msg(Msg::Group)
            })],),
        div![
            C!["mt-5"],
            button![
                C!["button", "is-link", "is-light", "is-small"],
                span![C!("icon"), i![C!("ion-plus")]],
                span!["Add group"],
                attrs! { At::Disabled => model.album.id.is_empty().as_at_value() },
                ev(Ev::Click, |_| Msg::AddGroup),
            ],
        ],
    ]
}

fn caption_view(model: &Model) -> Node<Msg> {
    div![
        label![C!("label"), "Caption style"],
        label![
            C!["field", "radio", "album-edit-radio"],
            input![
                C!("mr-1"),
                attrs! {
                    At::Type => "radio",
                    At::Name => "Round",
                    At::Checked => (model.album.caption_style == Style::Round).as_at_value(),
                },
                ev(Ev::Click, |_| Msg::StyleChanged(Style::Round)),
            ],
            Style::Round.to_string()
        ],
        label![
            C!["radio", "album-edit-radio"],
            input![
                C!("mr-1"),
                attrs! {
                    At::Type => "radio",
                    At::Name => "Square",
                    At::Checked => (model.album.caption_style == Style::Square).as_at_value(),
                },
                ev(Ev::Click, |_| Msg::StyleChanged(Style::Square)),
            ],
            Style::Square.to_string()
        ],
        label![C!("label"), "Caption color"],
        div![
            C!("is-flex"),
            COLORS.iter().map(|color| {
                let c_selected = if &model.album.caption_color == color {
                    "album-edit-color-selected"
                } else {
                    ""
                };
                let color = color.clone();
                span![
                    C!["album-edit-color", "mr-1", color.to_string(), c_selected],
                    ev(Ev::Click, |_| Msg::ColorChanged(color)),
                ]
            })
        ]
    ]
}
