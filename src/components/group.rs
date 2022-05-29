use seed::{self, prelude::*, *};
use uuid::Uuid;

use super::upload;
use crate::models::{group::Group, picture::Picture, vars::THUMB_URI};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    upload: upload::Model,
}

impl Model {
    pub fn new() -> Self {
        Self {
            upload: upload::Model::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum UpdateType {
    Title,
    CountFakePictures,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    TitleChanged(String, Group),
    Upload(upload::Msg),
    UpdateGroup(Group, UpdateType),
    AddPicture(Picture, Uuid),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(input, mut group) => {
            group.title = input;
            orders.send_msg(Msg::UpdateGroup(group, UpdateType::Title));
        }
        Msg::Upload(msg) => {
            match msg {
                upload::Msg::Success(ref picture, ref group_id) => {
                    orders.send_msg(Msg::AddPicture(picture.clone(), *group_id));
                }
                upload::Msg::RenderFakePictures(count, ref group) => {
                    let mut gr = group.clone();
                    gr.count_fake_pictures = count;
                    orders.send_msg(Msg::UpdateGroup(gr, UpdateType::CountFakePictures));
                }
                _ => (),
            }
            upload::update(msg, &mut model.upload, &mut orders.proxy(Msg::Upload));
        }
        Msg::UpdateGroup(_, _) | Msg::AddPicture(_, _) => (),
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
                        img![
							attrs! { At::Src => 
								THUMB_URI.to_string() +
								picture.public_id.as_str() + 
								"." + 
								picture.format.as_str() 
							}
						]
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
        upload::view(gr).map_msg(Msg::Upload),
    ]
}
