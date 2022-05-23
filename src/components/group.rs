use seed::{self, prelude::*, *};

use crate::models::group::Group;

// ------ ------
//     Model
// ------ -----
pub struct Model {
    group: Group,
}

impl Model {
    pub fn new() -> Self {
        Self {
            group: Group::new(),
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    TitleChanged(String, Group),
    UpdateGroup(Group),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::TitleChanged(input, group) => {
            model.group.id = group.id;
            model.group.title = input;
            orders.send_msg(Msg::UpdateGroup(model.group.clone()));
        }
        Msg::UpdateGroup(_) => (),
    }
}

pub fn view(group: Group) -> Node<Msg> {
    div![
        C!("box"),
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
		div![
			C!["columns", "m-2"],
			(0..4).map(|_| {
			figure![
				C!["image", "is-96x96"],
					img![ 
						attrs! { At::Src => "https://bulma.io/images/placeholders/96x96.png" }
					]
				]
		})], 
		button![
			C!["button", "is-link", "is-light", "is-small"],
			span![C!("icon"), i![C!("ion-upload")]],
			span!["Add picture"],
			//ev(Ev::Click, |_| Msg::AddGroup),
		],
    ]
}
