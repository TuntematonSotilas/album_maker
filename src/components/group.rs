use seed::{self, prelude::*, *};
use uuid::Uuid;

use super::picture;
use super::upload;
use crate::models::state::DeleteState;
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
    Delete(Uuid),
	DeleteGroup(Uuid),
	AskDeleteGroup(Uuid),
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
        Msg::UpdateGroup(_) => (),
        Msg::Delete(group_id) => {
			orders.send_msg(Msg::DeleteGroup(group_id));
		}
		Msg::AskDeleteGroup(group_id) => (),
		Msg::DeleteGroup(_) => (),
    }
}

pub fn view(album_id: String, group: Group, state_opt: Option<&State>) -> Node<Msg> {
    let gr_t = group.clone();
	let gr_d = group.clone();
    let gr_p = group;

	let mut c_state = "";
	if state_opt.is_some() {
		c_state = "hide";
	};

    div![
        C!("box group"),
        div![
            C!("field"),
            div![
                C!("control"),
                div![
                    C!("label"),
                    "Group name",
                    button![C!["delete", "delete-group"], ev(Ev::Click, move |_| Msg::AskDeleteGroup(gr_d.id)),],
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
        div![C!(c_state),
            match gr_p.pictures.clone() {
                Some(pictures) => div![pictures.iter().map(|picture| {
                    picture::view(gr_p.id, picture.clone()).map_msg(Msg::Picture)
                })],
                None => empty![],
            },
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
