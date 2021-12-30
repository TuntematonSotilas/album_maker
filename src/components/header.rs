use seed::{self, prelude::*, *};
use load_dotenv::load_dotenv;

use crate::models::user::User;

load_dotenv!();

const TITLE: &str = "Album maker";

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
	user: Option<User>,
	base_url: Url,
}

impl Model {
	pub fn new(base_url: Url) -> Self {
		Model { 
			user: None, 
			base_url: base_url
		}
	}
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	InitAuth,
	AuthInitialized(Result<JsValue, JsValue>),
	RedirectingToLogIn(Result<(), JsValue>),
	LogInOrOut,
	LogIn,
	LogOut,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::InitAuth => {
			orders.skip(); // No need to rerender
			orders.perform_cmd(async { 
				let auth_domain = env!("AUTH_DOMAIN", "Cound not find AUTH_DOMAIN in .env");
				let auth_client_id = env!("AUTH_CLIENT_ID", "Cound not find AUTH_CLIENT_ID in .env");
				Msg::AuthInitialized(
					init_auth(auth_domain.to_owned(), auth_client_id.to_owned()).await
				)
			});
		},
		Msg::AuthInitialized(Ok(user)) => {
            if not(user.is_undefined()) {
                match serde_wasm_bindgen::from_value(user) {
                    Ok(user) => {
						log!(user);
						model.user = Some(user)
					},
                    Err(error) => error!("User deserialization failed!", error),
                }
            }
            let search = model.base_url.search_mut();
            if search.remove("code").is_some() && search.remove("state").is_some() {        
                model.base_url.go_and_replace();
            }
        },
		Msg::AuthInitialized(Err(error)) => {
            error!("Auth initialization failed!", error);
        },
		Msg::RedirectingToLogIn(result) => {
            if let Err(error) = result {
                error!("Redirect to log in failed!", error);
            }
        },
		Msg::LogInOrOut => {
			match model.user.is_some() {
				true => {orders.send_msg(Msg::LogOut);},
				false => {orders.send_msg(Msg::LogIn);},
			}
		},
		Msg::LogIn => {
            orders.perform_cmd(async { Msg::RedirectingToLogIn(
                redirect_to_log_in().await
            )});
        },
		Msg::LogOut => {
            if let Err(error) = logout() {
                error!("Cannot log out!", error);
            } else {
                model.user = None;
            }
        },
	}
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
	div![
		C!("header"),
		div![C!("header__title"), 
			TITLE
		],
		div![				
			IF!(model.user.is_some() => div![
				C!("header__name"), 
				&model.user.as_ref().unwrap().name
			]),
			button![C!("header__btn"), 
				match model.user.is_some() {
					true => "LOGOUT",
					false => "LOGIN",
				},
				ev(Ev::Click, |_| Msg::LogInOrOut),
			]
		]
	]
}

// Mapping for JS functions
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn init_auth(domain: String, client_id: String) -> Result<JsValue, JsValue>;

	#[wasm_bindgen(catch)]
	async fn redirect_to_log_in() -> Result<(), JsValue>;
	
	#[wasm_bindgen(catch)]
	fn logout() -> Result<(), JsValue>;
}