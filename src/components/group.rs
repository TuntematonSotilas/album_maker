use seed::{self, prelude::*, *};

use crate::models::group::Group;

use super::upload;

// ------ ------
//     Model
// ------ -----
pub struct Model {
    group: Group,
	upload: upload::Model,
}

impl Model {
    pub fn new() -> Self {
        Self {
            group: Group::new(),
			upload: upload::Model::default(),
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    TitleChanged(String, Group),
    UpdateGroup(Group),
	Upload(upload::Msg),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(input, group) => {
            model.group.id = group.id;
            model.group.title = input;
            orders.send_msg(Msg::UpdateGroup(model.group.clone()));
        }
        Msg::UpdateGroup(_) => (),
		Msg::Upload(msg) => {
            upload::update(msg, &mut model.upload, &mut orders.proxy(Msg::Upload));
        }
    }
}

pub fn view(group: Group) -> Node<Msg> {
    div![
        C!("box group"),
        div![
            C!("field"),
            div![
                C!("control"),
                input![
                    C!("input"),
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
        upload::view().map_msg(Msg::Upload),
    ]
}
