use seed::{self, prelude::*, *};
use uuid::Uuid;

use crate::{models::{picture::Picture, vars::THUMB_URI, notif::{Notif, NotifType}}, api::api};

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    CaptionChanged(Uuid, String, Picture),
    UpdateCaption(Uuid, String, String),
	DeletePicture(Uuid, String, String),
	DeletePictureSuccess(Uuid, String),
	DeleteFail,
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::CaptionChanged(group_id, input, mut picture) => {
            picture.caption = Some(input.clone());
            orders.send_msg(Msg::UpdateCaption(group_id, input, picture.asset_id));
        }
        Msg::UpdateCaption(_, _, _) => (),
		Msg::DeletePicture(group_id, public_id, asset_id) => {
			orders.skip(); // No need to rerender
			orders.perform_cmd(async move {
				let success = api::delete_picture(public_id).await;
				match success {
					true => Msg::DeletePictureSuccess(group_id, asset_id),
					false => Msg::DeleteFail
				}
			});
		},
		Msg::DeletePictureSuccess(_, _) => (),
		Msg::DeleteFail => {
			orders.notify(Notif { 
				notif_type: NotifType::Success, 
				message : "Error deleting picture".to_string()});
		},
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
                            group_id.clone(), input, picture
                        )),
                    ]
                ]
            ],
			div![
				C!("control"),
				button![
					C!["button", "is-danger", "is-light", "is-small"],
					span![C!("icon"), i![C!("ion-close-circled")]],
					span!["Delete"],
					ev(Ev::Click, move |_| Msg::DeletePicture(group_id.clone(), pic_del.public_id, pic_del.asset_id))
				]
			]
        ]
    ]
}
