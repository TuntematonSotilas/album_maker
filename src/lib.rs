// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use components::notification::NotifType;
use models::page::LK_LOGIN;
use seed::{prelude::*, *};
use crate::components::*;

mod models;
mod components;

// ------ ------
//     Init
// ------ ------
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
	
	orders
		.subscribe(Msg::UrlChanged);
	
	let page = models::page::Page::init(url.to_owned());
    Model {
		is_logged: false,
		header: header::Model::new(page.to_owned()),
		notification: notification::Model::default(),
		my_albums: my_albums::Model::default(),
		new_album: new_album::Model::new(),
		page: page.to_owned(),
		login: login::Model::default(),
	}
}

// ------ ------
//     Model
// ------ ------
struct Model {
	is_logged: bool,
	header: header::Model,
	page: models::page::Page,
	my_albums: my_albums::Model,
	new_album: new_album::Model,
	notification: notification::Model,
	login: login::Model,
}

// ------ ------
//    Update
// ------ ------
enum Msg {
	Header(header::Msg),
	MyAlbums(my_albums::Msg),
	NewAlbum(new_album::Msg),
	Login(login::Msg),
	UrlChanged(subs::UrlChanged),
	Fetch,
	SetAuth(String),
	Notification(notification::Msg),
	ShowNotif(NotifType, String),
	SetIsLogged,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::Header(msg) => {
			match msg {
				header::Msg::LogInOrOut => {
					match model.is_logged {
						true => {
							orders.send_msg(Msg::SetAuth("".to_owned()));
							model.is_logged = false;
						}
						false => {
							let url = Url::new().add_path_part(LK_LOGIN);
							orders.notify(subs::UrlRequested::new(url));
						},
					}
				},
				_ => (),
			};
			header::update(msg, &mut model.header, &mut orders.proxy(Msg::Header));
		},
		Msg::Notification(msg) => {
			notification::update(msg, &mut model.notification, &mut orders.proxy(Msg::Notification));
		},
    	Msg::ShowNotif(notif_type, message) => {
			orders.send_msg(Msg::Notification(notification::Msg::Show(notif_type, message)));
		},
		Msg::MyAlbums(msg) => {
			my_albums::update(msg, &mut model.my_albums, &mut orders.proxy(Msg::MyAlbums));
		},
		Msg::NewAlbum(msg) => {
			match msg {
				new_album::Msg::ShowNotif(ref notif_type, ref message) => {
					orders.send_msg(Msg::ShowNotif(notif_type.to_owned(), message.to_owned()));
				},
				_ => (),
			}
			new_album::update(msg, &mut model.new_album, &mut orders.proxy(Msg::NewAlbum));
		},
		Msg::UrlChanged(subs::UrlChanged(mut url)) => {
			let page = match url.next_path_part(){
				Some(models::page::LK_MY_ALBUMS) => models::page::Page::MyAlbums,
				Some(models::page::LK_NEW_ALBUM) => models::page::Page::NewAlbum,
				Some(models::page::LK_LOGIN) => models::page::Page::Login,
				_ => models::page::Page::MyAlbums,
			};
			model.page = page.to_owned();

			orders.send_msg(Msg::Header(header::Msg::SetPage(page.to_owned())));
			orders.send_msg(Msg::Fetch);
			
		},
		Msg::Fetch => {
			if model.is_logged {
				match model.page {
					models::page::Page::MyAlbums => {
						orders.send_msg(Msg::MyAlbums(my_albums::Msg::Fetch));
					},
					_ => (),
				}
			} 
		},
		Msg::Login(msg) => {
			match msg {
				login::Msg::SetAuth(ref auth) => {
					orders.send_msg(Msg::SetIsLogged);
					orders.send_msg(Msg::SetAuth(auth.to_owned()));
					orders.notify(subs::UrlRequested::new(Url::new()));
				},
				login::Msg::ShowNotif(ref notif_type, ref message) => {
					orders.send_msg(Msg::ShowNotif(notif_type.to_owned(), message.to_owned()));
				},
				_ => (),
			}
			login::update(msg, &mut model.login, &mut orders.proxy(Msg::Login));
		},
		Msg::SetIsLogged => {
			model.is_logged = true;
			orders.send_msg(Msg::Header(header::Msg::SetIsLogged));
		},
		Msg::SetAuth(auth) => {
			orders.send_msg(Msg::NewAlbum(new_album::Msg::SetAuth(auth.to_owned())));
			orders.send_msg(Msg::MyAlbums(my_albums::Msg::SetAuth(auth.to_owned())));
		},
	}
}

// ------ ------
//     View
// ------ ------
fn view(model: &Model) -> Node<Msg> {
	div![
		notification::view(&model.notification).map_msg(Msg::Notification),
		header::view(&model.header).map_msg(Msg::Header),
		div![C!["container", "mt-5"],
			match &model.page {
				models::page::Page::Login => login::view(&model.login).map_msg(Msg::Login),
				_ => match &model.is_logged {
					true => {
						div![C!["columns", "is-centered"],
							match &model.page {
								models::page::Page::NewAlbum => new_album::view(&model.new_album).map_msg(Msg::NewAlbum),
								models::page::Page::MyAlbums => my_albums::view(&model.my_albums),
								_ => empty!(),
							}
						]
					},
					false => error::view("Please log in to continue".to_string(), "ion-log-in".to_string()),
				},
			}
		]
	]
}

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}