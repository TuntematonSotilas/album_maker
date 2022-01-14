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
		//.notify(subs::UrlChanged(url.to_owned()));


	orders.send_msg(Msg::InitAuth);
	
	let page = models::page::Page::init(url.to_owned());
    Model {
		header: header::Model::new(url.to_owned(), page.to_owned()),
		my_albums: my_albums::Model::default(),
		new_album: new_album::Model::default(),
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
	InitAuth,
	UrlChanged(subs::UrlChanged),
	Fetch,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::InitAuth => {
			orders.send_msg(Msg::Header(header::Msg::InitAuth));
			orders.send_msg(Msg::Fetch);
		},
		Msg::Header(msg) => {
			header::update(msg, &mut model.header, &mut orders.proxy(Msg::Header));
		},
		Msg::MyAlbums(msg) => {
			my_albums::update(msg, &mut model.my_albums, &mut orders.proxy(Msg::MyAlbums));
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
				log!("user");

				match model.page {
					models::page::Page::MyAlbums => {
						orders.send_msg(Msg::MyAlbums(
							my_albums::Msg::Fetch(model.header.user.to_owned().unwrap())));
					},
					_ => (),
				}
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
					match &model.page {
						models::page::Page::NewAlbum => new_album::view(&model.new_album),
						models::page::Page::MyAlbums => my_albums::view(&model.my_albums),
					}
				},
				None => not_logged::view(),
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