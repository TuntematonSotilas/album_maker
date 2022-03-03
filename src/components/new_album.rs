use seed::{self, prelude::*, *};

use crate::models::{page::TITLE_NEW_ALBUM, album::Album, vars::BASE_URI};

// ------ ------
//     Model
// ------ -----
pub struct Model {
	auth_header: String,
	album: Album,
}

impl Model {
	pub fn new() -> Self {
		Self {
			auth_header: String::new(),
			album: Album::new()
		}
	}
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	SetAuth(String),
	Submit,
    Submited,
    SubmitFailed(String),
	TitleChanged(String),
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::SetAuth(auth_header) => model.auth_header = auth_header,
		Msg::Submit => {
            orders.skip(); // No need to rerender
			let uri = BASE_URI.to_owned() + "editalbum";
			let auth = model.auth_header.to_owned();
            let request = Request::new(uri)
                .method(Method::Put)
				.header(Header::authorization(auth))
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
        },
		Msg::TitleChanged(title) => model.album.title = title,
	}
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
	
	div! [C!("box"),
		p![C!("title is-5 has-text-link"), TITLE_NEW_ALBUM],
		div![C!["field", "has-addons"],
			div![C!("control"),
				input![C!("input"),
					attrs!{
						At::Type => "text", 
						At::Name => "title",
						At::Placeholder => "Title",
						At::Value => model.album.title,
					},
					input_ev(Ev::Input, Msg::TitleChanged),
				]
			],
			div![C!("control"),
				a![C!["button", "is-primary"], 
					"Save",
					ev(Ev::Click, |_| Msg::Submit),
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