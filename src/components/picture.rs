use seed::{self, prelude::*, *};
use uuid::Uuid;

use crate::{
    api::apifn,
    models::{
        notif::{Notif, TypeNotifs},
        picture::Picture,
        vars::THUMB_URI,
    },
};

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    UpdateCaption(Uuid, String, String),
    DeletePicture(Uuid, String, String),
    DeletePictureSuccess(Uuid, String),
    DeleteFail,
	SetAlbumCover(Uuid, String),
	SetGroupCover(Uuid, String),
}

pub fn update(msg: Msg, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::DeletePicture(group_id, public_id, asset_id) => {
            orders.skip(); // No need to rerender
            orders.perform_cmd(async move {
                let success = apifn::delete_picture(public_id).await;
                if success {
                    Msg::DeletePictureSuccess(group_id, asset_id)
                } else {
                    Msg::DeleteFail
                }
            });
        }
        Msg::DeletePictureSuccess(_, _) 
			| Msg::UpdateCaption(_, _, _) 
			| Msg::SetAlbumCover(_, _)
			| Msg::SetGroupCover(_, _)=> (),
        Msg::DeleteFail => {
            orders.notify(Notif {
                notif_type: TypeNotifs::Error,
                message: "Error deleting picture".to_string(),
            });
        }
    }
}

pub fn view(group_id: Uuid, picture: &Picture, album_cover :String, group_cover: String ) -> Node<Msg> {
    let asset_id = picture.asset_id.clone();
    let asset_id2 = picture.asset_id.clone();
	let asset_id3 = picture.asset_id.clone();
	let asset_id4 = picture.asset_id.clone();
    let public_id = picture.clone().public_id;
    div![
        C!["container", "columns", "is-vcentered", "is-mobile"],
        div![
            attrs! { At::Draggable => true },
            C!["column", "is-flex-grow-0"],
            span![C!("icon"), i![C!("ion-drag")]],
        ],
        div![
            C!["column", "is-flex-grow-0"],
            figure![
                C!["image", "is-128x128"],
                img![
                    attrs! { At::Src => format!("{}{}.{}", THUMB_URI, picture.public_id, picture.format) }
                ]
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
                            At::Value => picture.caption.clone().unwrap_or_default(),
                        },
                        input_ev(Ev::Input, move |input| Msg::UpdateCaption(group_id, input, asset_id)),
                    ]
                ],
				div![
					C!("field"),
					input![
						C!["switch", "is-outlined", "is-small", "is-info"],
						attrs!{ 
							At::Type => "checkbox" ,
							At::Checked => (album_cover == picture.asset_id.clone()).as_at_value()
						},
					],
					label![
						C!("mr-2"),
						"Album cover",
						ev(Ev::Click, move |_| Msg::SetAlbumCover(group_id, asset_id2)),
					],
					input![
						C!["switch", "is-outlined", "is-small", "is-info"],
						attrs!{ 
							At::Type => "checkbox",
							At::Checked => (group_cover == picture.asset_id.clone()).as_at_value()
						},
					],
					label![
						"Group cover",
						ev(Ev::Click, move |_| Msg::SetGroupCover(group_id, asset_id3)),
					]
				],
				div![
					C!("control"),
					button![
						C!["button", "is-link", "is-light", "is-small"],
						span![C!("icon"), i![C!("ion-close-circled")]],
						span!["Delete"],
						ev(Ev::Click, move |_| Msg::DeletePicture(group_id, public_id, asset_id4))
					]
				]
			]
        ]
    ]
}
