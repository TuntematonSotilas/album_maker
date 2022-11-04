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

pub fn view(album_id: String, group: Group, state_opt: Option<&State>) -> Node<Msg> {
    let gr_t = group.clone();
    let gr_d = group.clone();
    let gr_dd = group.clone();
    let gr_p = group;

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
                                ev(Ev::Click, move |_| Msg::BeginDeleteGroup(gr_d.id)),
                            ],
                        ],
                        input![
                            C![
                                "input",
                                "is-small",
                                IF!(gr_t.title.is_empty() => "is-danger")
                            ],
                            attrs! {
                                At::Type => "text",
                                At::Name => "title",
                                At::Placeholder => "Group name",
                                At::Value => gr_t.title,
                            },
                            input_ev(Ev::Input, move |input| Msg::TitleChanged(input, gr_t.id)),
                        ],
                    ],
                ],
                div![
                    gr_p.pictures.as_ref().map_or(empty![], |pictures| {
                        let pictures_cl = pictures.clone();
                        div![pictures_cl.iter().map(|picture| {
                            let pic_cl = picture.clone();
                            let pic_id = picture.asset_id.clone();
                            let pic_idd = picture.asset_id.clone();
                            let gr_dd = gr_dd.clone().id;
                            div![
                                attrs! { At::Draggable => true },
                                ev(Ev::DragEnd, move |_| { Msg::DragEnded(pic_id) }),
                                ev(Ev::Drop, move |_| { Msg::Drop(gr_dd, pic_idd) }),
                                drag_ev(Ev::DragOver, |event| {
                                    event.stop_propagation();
                                    event.prevent_default();
                                    event.data_transfer().unwrap().set_drop_effect("move");
                                    Msg::DragOver
                                }),
                                picture::view(gr_p.id, pic_cl).map_msg(Msg::Picture),
                            ]
                        })]
                    }),
                    (0..gr_p.count_fake_pictures).map(|_| {
                        figure![
                            C!["image", "is-128x128", "m-1"],
                            progress![
                                C!["progress", "picture-progress"],
                                attrs! { At::Max => 100 }
                            ],
                        ]
                    }),
                ],
                upload::view(album_id, gr_p.id).map_msg(Msg::Upload),
            ]
        }
    ]
}
