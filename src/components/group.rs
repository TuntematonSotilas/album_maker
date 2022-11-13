use seed::{self, prelude::*, *};
use uuid::Uuid;

use super::picture;
use super::upload;
use crate::models::state::State;
use crate::models::{
    group::Group,
    group_update::{GroupUpdate, UpdateType},
};

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    TitleChanged(String, Uuid),
    UpdateGroup(GroupUpdate),
    Upload(upload::Msg),
    Picture(picture::Msg),
    BeginDeleteGroup(Uuid),
    Drop(Uuid, String),
    DragEnded(String),
    DragOver,
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(input, group_id) => {
            orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                upd_type: UpdateType::Title,
                id: group_id,
                picture: None,
                grp_data: Some(input),
                count_fake_pictures: None,
                asset_id: None,
                caption: None,
            }));
        }
        Msg::Upload(msg) => {
            match msg {
                upload::Msg::Success(ref picture, group_id) => {
                    orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                        upd_type: UpdateType::AddPicture,
                        id: group_id,
                        picture: Some(picture.clone()),
                        grp_data: None,
                        count_fake_pictures: None,
                        asset_id: None,
                        caption: None,
                    }));
                }
                upload::Msg::RenderFakePictures(count, group_id) => {
                    orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                        upd_type: UpdateType::CountFakePictures,
                        id: group_id,
                        picture: None,
                        grp_data: None,
                        count_fake_pictures: Some(count),
                        asset_id: None,
                        caption: None,
                    }));
                }
                _ => (),
            }
            upload::update(msg, &mut orders.proxy(Msg::Upload));
        }
        Msg::Picture(msg) => {
            match msg {
                picture::Msg::UpdateCaption(group_id, ref caption, ref asset_id) => {
                    orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                        upd_type: UpdateType::Caption,
                        id: group_id,
                        picture: None,
                        grp_data: None,
                        count_fake_pictures: None,
                        asset_id: Some(asset_id.clone()),
                        caption: Some(caption.clone()),
                    }));
                }
                picture::Msg::DeletePictureSuccess(group_id, ref asset_id) => {
                    orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                        upd_type: UpdateType::DeletePicture,
                        id: group_id,
                        picture: None,
                        grp_data: None,
                        count_fake_pictures: None,
                        asset_id: Some(asset_id.clone()),
                        caption: None,
                    }));
                }
                _ => (),
            }
            picture::update(msg, &mut orders.proxy(Msg::Picture));
        }
        Msg::UpdateGroup(_)
        | Msg::BeginDeleteGroup(_)
        | Msg::Drop(_, _)
        | Msg::DragEnded(_)
        | Msg::DragOver => (),
    }
}

pub fn view(album_id: String, group: &Group, state_opt: Option<&State>) -> Node<Msg> {
    let grp_id = group.id;
    div![
        C!["box group"],
        if state_opt.is_some() {
            let state = state_opt.unwrap();
            progress![
                C!["progress", "is-danger"],
                attrs! { At::Value => state.current, At::Max => state.total }
            ]
        } else {
            div![
                div![
                    C!("field"),
                    div![
                        C!("control"),
                        div![
                            C!("label"),
                            "Group name",
                            button![
                                C!["delete", "delete-group"],
                                ev(Ev::Click, move |_| Msg::BeginDeleteGroup(grp_id)),
                            ],
                        ],
                        input![
                            C![
                                "input",
                                "is-small",
                                IF!(group.title.is_empty() => "is-danger")
                            ],
                            attrs! {
                                At::Type => "text",
                                At::Name => "title",
                                At::Placeholder => "Group name",
                                At::Value => group.title,
                            },
                            input_ev(Ev::Input, move |input| Msg::TitleChanged(input, grp_id)),
                        ],
                    ],
                ],
                div![
                    group.pictures.as_ref().map_or(empty![], |pictures| {
                        div![pictures.iter().map(|picture| {
                            let asset_id = picture.asset_id.clone();
                            let asset_id2 = picture.asset_id.clone();
                            let grp_id = group.id;
                            div![
                                ev(Ev::DragEnd, move |_| Msg::DragEnded(asset_id)),
                                ev(Ev::Drop, move |_| Msg::Drop(grp_id, asset_id2)),
                                drag_ev(Ev::DragOver, |event| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    event.data_transfer().unwrap().set_drop_effect("move");
                                    Msg::DragOver
                                }),
                                picture::view(group.id, picture).map_msg(Msg::Picture),
                            ]
                        })]
                    }),
                    (0..group.count_fake_pictures).map(|_| {
                        figure![
                            C!["image", "is-128x128", "m-1"],
                            progress![
                                C!["progress", "picture-progress"],
                                attrs! { At::Max => 100 }
                            ],
                        ]
                    }),
                ],
                upload::view(album_id, group.id).map_msg(Msg::Upload),
            ]
        }
    ]
}
