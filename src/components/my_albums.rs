use seed::{self, prelude::*, *};

use crate::{models::{page::TITLE_MY_ALBUMS, vars::BASE_URI, album::Album}, components::error};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
	auth_header: String,
	is_forbidden: bool,
	albums: Option<Vec<Album>>,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
	SetAuth(String),
	Fetch,
	Received(Vec<Album>),
	Forbidden,
	Error,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
	match msg {
		Msg::SetAuth(auth_header) => model.auth_header = auth_header,
		Msg::Fetch => {
			let auth = model.auth_header.to_owned();
			orders.skip(); // No need to rerender
			orders.perform_cmd(async {
                let uri = BASE_URI.to_owned() + "myalbums";
				let response = Request::new(uri)
                    .header(Header::authorization(auth))
                    .fetch()
                    .await
                    .expect("HTTP request failed");

				match response.status().code {
					200 => {
						let albums = response
							.json::<Vec<Album>>()
							.await
							.expect("deserialization failed");
						Msg::Received(albums)
					},
					403 => {
						Msg::Forbidden
					},
					_ => Msg::Error,
				}

            });
		},
		Msg::Error => {
			log!("Error");
		}
		Msg::Forbidden => {
			model.is_forbidden = true;
		},
		Msg::Received(albums) => {
            model.albums = Some(albums);
        }
	}
}

// ------ ------
//     View
// ------ ------
pub fn view<Ms>(model: &Model) -> Node<Ms> {
	
	match &model.is_forbidden {
		true => error::view("Forbidden".to_string(), "ion-alert-circled".to_string()),
		false => {
			if !&model.albums.is_some() || model.albums.as_ref().unwrap().is_empty() {
				error::view("No data".to_string(), "ion-search".to_string()) 
			} else {
				div![C!("panel is-link"),
					p![C!("panel-heading"), TITLE_MY_ALBUMS],
					model.albums.as_ref().unwrap().iter().map(|album| {
						a![C!("panel-block"),
							&album.title
						]
					})
				]
			}	
		}
	}
}