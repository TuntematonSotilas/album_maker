use seed::{self, prelude::*, *};

use crate::models::{page::TITLE_NEW_ALBUM, album::{Album, self}};

// ------ ------
//     Model
// ------ -----
pub struct Model {
	album: Album,
}

impl Model {
	pub fn new() -> Self {
		Self {
			album: Album::new()
		}
	}
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	Submit,
    Submited,
    SubmitFailed(String),
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Submit => {
            orders.skip(); // No need to rerender

            // Created outside async block because of lifetime reasons
            // (we can't use reference to `model.form` in async function).
            let request = Request::new("/")
                .method(Method::Post)
                .json(&model.album)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");

                if response.status().is_ok() {
                    Msg::Submited
                } else {
                    Msg::SubmitFailed(response.status().text)
                }
            });
        },
		Msg::Submited => {
            log!("Submit succeeded");
        }
        Msg::SubmitFailed(reason) => {
            log!("Submit failed {0}", reason);
        }
	}
}

// ------ ------
//     View
// ------ ------
pub fn view<Ms>(_model: &Model) -> Node<Ms> {
	
	div! [C!("box"),
		p![C!("title is-5 has-text-link"), TITLE_NEW_ALBUM],
		div![C!["field", "has-addons"],
			div![C!("control"),
				input![C!("input"),
					attrs!{
						At::Type => "text", 
						At::Name => "title",
						At::Placeholder => "Title",
					}
				]
			],
			div![C!("control"),
				a![C!["button", "is-primary"], 
					"Save"
				]
			]
		],
		div![C!("field"),
			div![C!("control"),
				a![C!["button", "is-link", "is-light", "is-small"],
					span![C!("icon"),
						i![C!("ion-plus")]
					],
					span!["Add Group"],
				],
			]
		],
	]
}