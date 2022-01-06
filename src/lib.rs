// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};
use crate::components::header;

mod models;
mod components;

const MY_ALBUMS: &str = "my-albums";
const NEW_ALBUM: &str = "new-album";

// ------ ------
//     Init
// ------ ------
fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
	orders.send_msg(Msg::InitAuth);
    Model {
		header: header::Model::new(url)
	}
}

// ------ ------
//     Urls
// ------ ------
struct_urls!();
impl<'a> Urls<'a> {
    fn home(self) -> Url {
        self.base_url()
    }
	fn my_albums(self) -> Url {
        self.base_url().add_path_part(MY_ALBUMS)
    }
	fn new_album(self) -> Url {
        self.base_url().add_path_part(NEW_ALBUM)
    }
}

// ------ ------
//     Model
// ------ ------
struct Model {
	header: header::Model,
}

// ------ ------
//    Update
// ------ ------
enum Msg {
	Header(header::Msg),
	InitAuth,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::InitAuth => {
			orders.send_msg(Msg::Header(header::Msg::InitAuth));
		},
		Msg::Header(msg) => {
			header::update(msg, &mut model.header, &mut orders.proxy(Msg::Header));
		},
	}
}

// ------ ------
//     View
// ------ ------
fn view(model: &Model) -> Node<Msg> {
	div![
		header::view(&model.header).map_msg(Msg::Header),
	]
}

// ------ ------
//     Start
// ------ ------
#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}