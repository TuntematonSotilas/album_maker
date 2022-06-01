use seed::{self, prelude::*, *};

use super::upload;
use crate::models::{
    group::Group,
    group_update::{GroupUpdate, UpdateType},
    vars::THUMB_URI,
};

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    TitleChanged(String, Group),
    Upload(upload::Msg),
    UpdateGroup(GroupUpdate),
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(input, mut group) => {
            group.title = input.clone();
            orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                upd_type: UpdateType::Title,
                id: Some(group.id),
                picture: None,
                title: Some(input),
                count_fake_pictures: None,
            }));
        }
        Msg::Upload(msg) => {
            match msg {
                upload::Msg::Success(ref picture, group_id) => {
                    orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                        upd_type: UpdateType::AddPicture,
                        id: Some(group_id),
                        picture: Some(picture.clone()),
                        title: None,
                        count_fake_pictures: None,
                    }));
                }
                upload::Msg::RenderFakePictures(count, group_id) => {
                    orders.send_msg(Msg::UpdateGroup(GroupUpdate {
                        upd_type: UpdateType::CountFakePictures,
                        id: Some(group_id),
                        picture: None,
                        title: None,
                        count_fake_pictures: Some(count),
                    }));
                }
                _ => (),
            }
            upload::update(msg, &mut orders.proxy(Msg::Upload));
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
                ]
            ]
        ],
        div![
            match gr.clone().pictures {
                Some(pictures) => div![pictures.iter().map(|picture| {
                    figure![
                        C!["image", "is-128x128"],
                        img![attrs! { At::Src =>
                            THUMB_URI.to_string() +
                            picture.public_id.as_str() +
                            "." +
                            picture.format.as_str()
                        }]
                    ]
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
