use super::picture;
use super::upload;
use crate::models::state::DeleteStatus;
use crate::models::trip::TranspMode;
use crate::models::trip::Trip;
use crate::models::trip::TRANSP_MODE;
use crate::models::{
    group::Group,
    group_update::{GroupUpdate, UpdateType},
};
use seed::{self, prelude::*, *};
use uuid::Uuid;

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
    TripChanged(Uuid, Option<TranspMode>, String, String),
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
                delete_status: None,
                trip: None,
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
                        delete_status: None,
                        trip: None,
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
                        delete_status: None,
                        trip: None,
                    }));
                }
                _ => (),
            }
            upload::update(msg, &mut orders.proxy(Msg::Upload));
        }
        Msg::Picture(msg) => update_picture(msg, orders),
        Msg::BeginDeleteGroup(group_id) => {
            orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                upd_type: UpdateType::DeleteState,
                id: group_id,
                picture: None,
                grp_data: None,
                count_fake_pictures: None,
                asset_id: None,
                caption: None,
                delete_status: Some(DeleteStatus::Deleting),
                trip: None,
            }));
        }
        Msg::TripChanged(group_id, transp_mode, origin, destination) => {
            let mut trip: Option<Trip> = None;
            if let Some(transp_mode) = transp_mode {
                trip = Some(Trip {
                    transp_mode,
                    origin,
                    destination,
                });
            }
            orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                upd_type: UpdateType::TripChanged,
                id: group_id,
                picture: None,
                grp_data: None,
                count_fake_pictures: None,
                asset_id: None,
                caption: None,
                delete_status: None,
                trip,
            }));
        }
        Msg::UpdateGroup(_) | Msg::Drop(_, _) | Msg::DragEnded(_) | Msg::DragOver => (),
    }
}

fn update_picture(msg: picture::Msg, orders: &mut impl Orders<Msg>) {
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
                delete_status: None,
                trip: None,
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
                delete_status: None,
                trip: None,
            }));
        }
        picture::Msg::SetAlbumCover(group_id, ref asset_id) => {
            orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                upd_type: UpdateType::SetAlbumCover,
                id: group_id,
                picture: None,
                grp_data: None,
                count_fake_pictures: None,
                asset_id: Some(asset_id.clone()),
                caption: None,
                delete_status: None,
                trip: None,
            }));
        }
        picture::Msg::SetGroupCover(group_id, ref asset_id) => {
            orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                upd_type: UpdateType::SetGroupCover,
                id: group_id,
                picture: None,
                grp_data: None,
                count_fake_pictures: None,
                asset_id: Some(asset_id.clone()),
                caption: None,
                delete_status: None,
                trip: None,
            }));
        }
        _ => (),
    }
    picture::update(msg, &mut orders.proxy(Msg::Picture));
}

pub fn view(album_id: String, album_cover: &str, group: &Group) -> Node<Msg> {
    let grp_id = group.id;
    div![
        C!["box group"],
        if group.state.is_some() {
            let state = group.state.clone().unwrap();
            progress![
                C!["progress", "is-danger"],
                attrs! { At::Value => state.current, At::Max => state.total }
            ]
        } else {
            div![
                div![
                    C!["label", "control", "field"],
                    "Group name",
                    button![
                        C!["delete", "delete-group"],
                        ev(Ev::Click, move |_| Msg::BeginDeleteGroup(grp_id)),
                    ]
                ],
                input![
                    C![
                        "field",
                        "input",
                        "is-small",
                        IF!(group.title.is_empty() => "is-danger")
                    ],
                    attrs! {
                        At::Type => "text",
                        At::Name => "Group name",
                        At::Placeholder => "Group name",
                        At::Value => group.title,
                    },
                    input_ev(Ev::Input, move |input| Msg::TitleChanged(input, grp_id)),
                ],
                span![C!["label"], "Trip"],
                view_trip(group),
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
                                picture::view(group.id, picture, album_cover, group.cover.as_str())
                                    .map_msg(Msg::Picture),
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

fn view_trip(group: &Group) -> Node<Msg> {
    let grp_id = group.id;
    let inp_ori = group.trip.clone().unwrap_or_default().origin;
    let inp_dest = group.trip.clone().unwrap_or_default().destination;
    let inp_mode = group.trip.clone().unwrap_or_default().transp_mode;
    let inp_mode2 = group.trip.clone().unwrap_or_default().transp_mode;
    let trip = group.trip.clone().unwrap_or_default();
    div![
        div![
            C!["field", "select", "is-small"],
            select![
				option![
					"None",
					ev(Ev::Click, move |_| Msg::TripChanged(
						grp_id,
						None,
						String::new(),
						String::new())
					),
					attrs!(At::Selected => (group.trip.is_none()).as_at_value() )
				],
                TRANSP_MODE.iter().map(|mode| {
                    let origin = group.trip.clone().unwrap_or_default().origin;
                    let destination = group.trip.clone().unwrap_or_default().destination;
                    option![
                        mode.to_string(),
                        ev(Ev::Click, move |_| Msg::TripChanged(
                            grp_id,
                            Some(mode.clone()),
                            origin,
                            destination)
                        ),
                        attrs!(At::Selected => (mode == &inp_mode && group.trip.is_some()).as_at_value() )
                    ]
                }),
            ]
        ],
        span![C!["label"], "Origin"],
        input![C!["field", "input", "is-small"],
            attrs! {
                At::Type => "text",
                At::Name => "origin",
                At::Placeholder => "Origin",
                At::Value => trip.origin,
            },
            input_ev(Ev::Input, move |input| Msg::TripChanged(
                grp_id,
                Some(inp_mode),
                input,
                inp_dest)),
       ],
       span![C!["label"], "Destination"],
       input![C!["field", "input", "is-small"],
            attrs! {
                At::Type => "text",
                At::Name => "destination",
                At::Placeholder => "Destination",
                At::Value => trip.destination,
            },
            input_ev(Ev::Input, move |input| Msg::TripChanged(
                grp_id,
                Some(inp_mode2),
                inp_ori,
                input)),
        ],
   ]
}
