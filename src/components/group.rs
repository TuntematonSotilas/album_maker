use seed::{self, prelude::*, *};

use super::upload;
use crate::models::group::Group;

// ------ ------
//     Model
// ------ -----
pub struct Model {
    group: Group,
    upload: upload::Model,
	count_fake_pictures: u32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            group: Group::new(),
            upload: upload::Model::new(),
			count_fake_pictures: 0,
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    TitleChanged(String, Group),
    Upload(upload::Msg),
    UpdateGroup(Group),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(input, group) => {
            model.group.id = group.id;
            model.group.title = input;
            orders.send_msg(Msg::UpdateGroup(model.group.clone()));
        }
        Msg::Upload(msg) => {
            match msg {
				upload::Msg::Success(ref picture, ref group) => {
					let mut gr = group.clone();
                	if let Some(pictures) = &mut gr.pictures {
                    	pictures.push(picture.clone());
						
						orders.send_msg(Msg::UpdateGroup(gr));
                	}
				},
				upload::Msg::RenderFakePictures(count) => {
					model.count_fake_pictures = count;
				},
				_ => ()
            }
            upload::update(msg, &mut model.upload, &mut orders.proxy(Msg::Upload));
        }
        Msg::UpdateGroup(_) => (),
    }
}

pub fn view(model: &Model, group: Group) -> Node<Msg> {
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
			C!["columns", "m-1"],
			(0..model.count_fake_pictures).map(|_| {
			figure![
				C!["image", "is-128x128", "m-1"],
				progress![
					C!["progress", "picture-progress"],
					attrs! { At::Max => 100 }
				],
			]
		})],
		match gr.clone().pictures {
            Some(pictures) => div![pictures
                .iter()
                .map(|picture| { 
					figure![
						C!["image", "is-128x128"],
						img![
							attrs! { At::Src => picture.url }
						]
					]
				 })],
            None => empty![],
        },
        upload::view(gr).map_msg(Msg::Upload),
    ]
}
