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
	
	orders.subscribe(Msg::UrlChanged);

	orders.send_msg(Msg::InitAuth);

	let page = models::page::Page::init(url.to_owned());
    Model {
		header: header::Model::new(url.to_owned(), page.to_owned()),
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
	new_album: new_album::Model,
}

// ------ ------
//    Update
// ------ ------
enum Msg {
	Header(header::Msg),
	NewAlbum(new_album::Msg),
	InitAuth,
	UrlChanged(subs::UrlChanged),
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::InitAuth => {
			orders.send_msg(Msg::Header(header::Msg::InitAuth));
		},
		Msg::Header(msg) => {
			header::update(msg, &mut model.header, &mut orders.proxy(Msg::Header));
		},
		Msg::NewAlbum(msg) => {
			new_album::update(msg, &mut model.new_album, &mut orders.proxy(Msg::NewAlbum));
		},
		Msg::UrlChanged(subs::UrlChanged(mut url)) => {
			let page = match url.next_hash_path_part(){
				Some(models::page::MY_ALBUMS) => models::page::Page::MyAlbums,
				Some(models::page::NEW_ALBUM) => models::page::Page::NewAlbum,
				_ => models::page::Page::MyAlbums,
			};
			model.page = page;
		}
	}
}

// ------ ------
//     View
// ------ ------
fn view(model: &Model) -> Node<Msg> {
	div![
		header::view(&model.header).map_msg(Msg::Header),
		div![C!("container"),
			match &model.page {
				models::page::Page::NewAlbum => new_album::view(&model.new_album),
				models::page::Page::MyAlbums => empty!(),
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