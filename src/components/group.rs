use seed::{self, prelude::*, *};

use super::picture;
use super::upload;
use crate::models::{
    group::Group,
    group_update::{GroupUpdate, UpdateType},
};

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    TitleChanged(String, Group),
	DescChanged(String, Group),
    UpdateGroup(GroupUpdate),
    Upload(upload::Msg),
    Picture(picture::Msg),
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(input, mut group) => {
            group.title = input.clone();
            orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                upd_type: UpdateType::Title,
                id: group.id,
                picture: None,
                title: Some(input),
                count_fake_pictures: None,
                asset_id: None,
                caption: None,
            }));
        },
		Msg::DescChanged(input, mut group) => {
            group.title = input.clone();
            orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                upd_type: UpdateType::Title,
                id: group.id,
                picture: None,
                title: Some(input),
                count_fake_pictures: None,
                asset_id: None,
                caption: None,
            }));
        },
        Msg::Upload(msg) => {
            match msg {
                upload::Msg::Success(ref picture, group_id) => {
                    orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                        upd_type: UpdateType::AddPicture,
                        id: group_id,
                        picture: Some(picture.clone()),
                        title: None,
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
                        title: None,
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
            if let picture::Msg::UpdateCaption(group_id, ref caption, ref asset_id) = msg {
                orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                    upd_type: UpdateType::Caption,
                    id: group_id,
                    picture: None,
                    title: None,
                    count_fake_pictures: None,
                    asset_id: Some(asset_id.clone()),
                    caption: Some(caption.clone()),
                }));
            }
            picture::update(msg, &mut orders.proxy(Msg::Picture));
        }
        Msg::UpdateGroup(_) => (),
    }
}

pub fn view(group: Group) -> Node<Msg> {
    let gr = group.clone();
    div![
        C!("box group"),
        div![
            C!("field"),
            div![
                C!("control"),
                input![
                    C!["input", IF!(group.title.is_empty() => "is-danger")],
                    attrs! {
                        At::Type => "text",
                        At::Name => "title",
                        At::Placeholder => "Group name",
                        At::Value => group.title,
                    },
                    input_ev(Ev::Input, move |input| Msg::TitleChanged(input, group)),
                ],
			],
		],
		div![
            C!("field"),
			textarea![
				C!("textarea"),
				attrs! {
					At::Placeholder => "description"
				},
			]
		],
        div![
            match gr.pictures.clone() {
                Some(pictures) => div![pictures.iter().map(|picture| {
                    picture::view(gr.id, picture.clone()).map_msg(Msg::Picture)
                })],
                None => empty![],
            },
            (0..gr.count_fake_pictures).map(|_| {
                figure![
                    C!["image", "is-128x128", "m-1"],
                    progress![
                        C!["progress", "picture-progress"],
                        attrs! { At::Max => 100 }
                    ],
                ]
            }),
        ],
        upload::view(gr.id).map_msg(Msg::Upload),
    ]
}
