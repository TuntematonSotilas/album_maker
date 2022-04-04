use seed::{self, prelude::*, *};

use crate::models::{page::TITLE_NEW_ALBUM};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
	login: String,
	password: String,
	pwd: String,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	Submit,
	LoginChanged(String),
	PwdChanged(String),
}


pub fn update(msg: Msg, model: &mut Model, _orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Submit => {
            /*orders.skip(); // No need to rerender
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
                    Msg::ShowNotif(NotifType::Success, "Album saved".to_owned())
                } else {
                    Msg::ShowNotif(NotifType::Error, "Error when saving".to_owned())
                }
            });*/
        },
		Msg::LoginChanged(login) => model.login = login,
		Msg::PwdChanged(pwd) => model.pwd = pwd,
	}
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
	div! [C!("box"),
		p![C!("title is-5 has-text-link"), TITLE_NEW_ALBUM],
		div![C!("field"),
			div![C!("control"),
				input![C!("input"),
					attrs!{
						At::Type => "text", 
						At::Name => "login",
						At::Placeholder => "Login",
						At::Value => model.login,
					},
					input_ev(Ev::Input, Msg::LoginChanged),
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
					"Save",
					ev(Ev::Click, |_| Msg::Submit),
				]
			]
		],
	]
}