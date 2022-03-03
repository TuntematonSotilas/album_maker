// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

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


	orders.send_msg(Msg::InitAuth);
	
	let page = models::page::Page::init(url.to_owned());
    Model {
		header: header::Model::new(url.to_owned(), page.to_owned()),
		my_albums: my_albums::Model::default(),
		new_album: new_album::Model::new(),
		page: page.to_owned(),
	}
}

// ------ ------
//     Model
// ------ ------
struct Model {
	header: header::Model,
	page: models::page::Page,
	my_albums: my_albums::Model,
	new_album: new_album::Model,
}

// ------ ------
//    Update
// ------ ------
enum Msg {
	Header(header::Msg),
	MyAlbums(my_albums::Msg),
	NewAlbum(new_album::Msg),
	InitAuth,
	UrlChanged(subs::UrlChanged),
	Fetch,
	SetAuth,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::InitAuth => {
			orders.send_msg(Msg::Header(header::Msg::InitAuth));
		},
		Msg::Header(msg) => {
			match msg {
				header::Msg::UserLoged => {
					orders.send_msg(Msg::SetAuth);
				},
				_ => (),
			}
			header::update(msg, &mut model.header, &mut orders.proxy(Msg::Header));
		},
		Msg::MyAlbums(msg) => {
			my_albums::update(msg, &mut model.my_albums, &mut orders.proxy(Msg::MyAlbums));
		},
		Msg::NewAlbum(msg) => {
			new_album::update(msg, &mut model.new_album, &mut orders.proxy(Msg::NewAlbum));
		},
		Msg::UrlChanged(subs::UrlChanged(mut url)) => {
			let page = match url.next_path_part(){
				Some(models::page::LK_MY_ALBUMS) => models::page::Page::MyAlbums,
				Some(models::page::LK_NEW_ALBUM) => models::page::Page::NewAlbum,
				_ => models::page::Page::MyAlbums,
			};
			model.page = page.to_owned();

			orders.send_msg(Msg::Header(header::Msg::SetPage(page.to_owned())));
			orders.send_msg(Msg::Fetch);
			
		},
		Msg::Fetch => {
			log!("Fetch");
			if model.header.user.is_some() {
				match model.page {
					models::page::Page::MyAlbums => {
						orders.send_msg(Msg::MyAlbums(my_albums::Msg::Fetch));
					},
					_ => (),
				}
			} 
		},
		Msg::SetAuth => {
			log!("SetAuth");
			if let Some(user) = &model.header.user {
				let login = &user.sub;
                let pwd = env!("API_SALT", "Cound not find API_SALT in .env");
                let b64 = base64::encode(format!("{0}:{1}", login, pwd));
                let auth = format!("Basic {0}", b64);
				match model.page {
					models::page::Page::MyAlbums => {
						orders.send_msg(Msg::MyAlbums(my_albums::Msg::SetAuth(auth)));
					},
					models::page::Page::NewAlbum => {
						orders.send_msg(Msg::NewAlbum(new_album::Msg::SetAuth(auth)));
					},
				}
				orders.send_msg(Msg::Fetch);
			}
		}
	}
}

// ------ ------
//     View
// ------ ------
fn view(model: &Model) -> Node<Msg> {
	div![
		header::view(&model.header).map_msg(Msg::Header),
		div![C!("container mt-5"),
			match &model.header.user {
				Some(_) => {
					div![C!["columns", "is-centered"],
						div![C!["column is-half"],
							match &model.page {
								models::page::Page::NewAlbum => new_album::view(&model.new_album).map_msg(Msg::NewAlbum),
								models::page::Page::MyAlbums => my_albums::view(&model.my_albums),
							}
						]
					]
				},
				None => error::view("Please log in to continue".to_string(), "ion-log-in".to_string()),
			},
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