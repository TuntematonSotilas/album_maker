use seed::{self, prelude::*, *};
use uuid::Uuid;

use crate::{models::{picture::Picture, vars::THUMB_URI}, api::api};

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    CaptionChanged(Uuid, String, Picture),
    UpdateCaption(Uuid, String, String),
	DeletePicture(String),
	DeleteSuccess,
	DeleteFail,
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CaptionChanged(group_id, input, mut picture) => {
            picture.caption = Some(input.clone());
            orders.send_msg(Msg::UpdateCaption(group_id, input, picture.asset_id));
        }
        Msg::UpdateCaption(_, _, _) => (),
		Msg::DeletePicture(public_id) => {
			orders.skip(); // No need to rerender
			orders.perform_cmd(async {
				let success = api::delete_picture(public_id).await;
				match success {
					true => Msg::DeleteSuccess,
					false => Msg::DeleteFail
				}
			});
		},
		Msg::DeleteSuccess => {},
		Msg::DeleteFail => {},
    }
}

pub fn view(group_id: Uuid, picture: Picture) -> Node<Msg> {
	let pic_del = picture.clone();
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
			div![
				C!("control"),
				button![
					C!["button", "is-link", "is-light", "is-small"],
					span![C!("icon"), i![C!("ion-close-circled")]],
					span!["Delete"],
					ev(Ev::Click, |_| Msg::DeletePicture(pic_del.public_id))
				]
			]
        ]
    ]
}
