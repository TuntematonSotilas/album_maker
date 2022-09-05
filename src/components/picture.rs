use seed::{self, prelude::*, *};
use uuid::Uuid;

use crate::models::{picture::Picture, vars::THUMB_URI};

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    CaptionChanged(Uuid, String, Picture),
    UpdateCaption(Uuid, String, String),
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CaptionChanged(group_id, input, mut picture) => {
            picture.caption = Some(input.clone());
            orders.send_msg(Msg::UpdateCaption(group_id, input, picture.asset_id));
        }
        Msg::UpdateCaption(_, _, _) => (),
    }
}

pub fn view(group_id: Uuid, picture: Picture) -> Node<Msg> {
    div![
        C!["container", "columns", "is-vcentered", "is-mobile"],
        div![
            C!["column", "is-flex-grow-0"],
            figure![
                C!["image", "is-128x128"],
                img![attrs!{ At::Src => format!("{}{}.{}", THUMB_URI, picture.public_id, picture.format) }]
            ]
        ],
        div![
            C!("column"),
            div![
                C!("field"),
				label![C!("label"), "Caption"],
                div![
                    C!("control"),
                    input![
                        C!["input", "is-small"],
                        attrs! {
                            At::Type => "text",
                            At::Name => "caption",
                            At::Placeholder => "Caption",
                            At::Value => picture.clone().caption.unwrap_or_default(),
                        },
                        input_ev(Ev::Input, move |input| Msg::CaptionChanged(
                            group_id, input, picture
                        )),
                    ]
                ]
            ],
        ]
    ]
}
