use seed::{self, prelude::*, *};

use crate::models::{page::TITLE_LOGIN, vars::BASE_URI};

use super::notification::NotifType;

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
	username: String,
	password: String,
	pwd: String,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	Submit,
	UsernameChanged(String),
	PwdChanged(String),
	ShowNotif(NotifType, String),
	SetAuth(String),
}


pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Submit => {
            orders.skip(); // No need to rerender
			let uri = BASE_URI.to_owned() + "login";
			let encoded = String::new();
			let request = Request::new(uri)
                .method(Method::Post)
				.json(&encoded)
                .expect("Serialization failed");

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("HTTP request failed");

                if response.status().is_ok() {
					Msg::SetAuth(encoded)
                } else {
                    Msg::ShowNotif(NotifType::Error, "Error when saving".to_owned())
                }
            });
        },
		Msg::UsernameChanged(username) => model.username = username,
		Msg::PwdChanged(pwd) => model.pwd = pwd,
		Msg::ShowNotif(_, _) => (),
		Msg::SetAuth(_) => (),
	}
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
	div![C!["columns", "is-centered"],
		div![C!["column is-half"],
			div! [C!("box"),
				p![C!("title is-5 has-text-link"), TITLE_LOGIN],
				div![C!("field"),
					div![C!("control"),
						input![C!("input"),
							attrs!{
								At::Type => "text", 
								At::Name => "username",
								At::Placeholder => "Username",
								At::Value => model.username,
							},
							input_ev(Ev::Input, Msg::UsernameChanged),
						]
					],
				],
				div![C!("field"),
					div![C!("control"),
						input![C!("input"),
							attrs!{
								At::Type => "password", 
								At::Name => "pwd",
								At::Value => model.pwd,
							},
							input_ev(Ev::Input, Msg::PwdChanged),
						]
					]
				],
				div![C!("field"),
					div![C!("control"),
						a![C!["button", "is-primary"], 
							"LOGIN",
							ev(Ev::Click, |_| Msg::Submit),
						]
					]
				],
			],
		],
	]
}