use seed::{self, prelude::*, *};

use crate::models::{album::Album, page::TITLE_MY_ALBUMS, vars::BASE_URI};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
    auth_header: String,
    albums: Option<Vec<Album>>,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp,
    Received(Vec<Album>),
    Error,
    Delete,
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp => {
            let auth = model.auth_header.clone();
            orders.skip(); // No need to rerender
            orders.perform_cmd(async {
                let uri = BASE_URI.to_string() + "myalbums";
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
                    }
                    _ => Msg::Error,
                }
            });
        }
        Msg::Error => {
            error!("Error getting albums");
        }
        Msg::Received(albums) => {
            model.albums = Some(albums);
        }
        Msg::Delete => {
            log!("delete");
        }
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["column", "is-centered", "is-half"],
        div![
            C!["panel"],
			div![
                C!("box"),
				p![C!["title", "is-5", "has-text-link"], TITLE_MY_ALBUMS],
						
				if model.albums.is_some() {
					div![model.albums.as_ref().unwrap().iter().map(|album| {
						p![
							C!("panel-block"),
							div![
								C!["container", "columns", "is-mobile"],
								div![C!["column", "is-three-quarters"], &album.title],
								div![C!("column"), 
									button![C!("delete"), ev(Ev::Click, |_| Msg::Delete),]
								]
							]
						]
					})]
				} else {
					div![(0..4).map(|_| {
						p![
							C!("panel-block"),
							progress![
								C!["progress", "is-small", "table-progress"],
								attrs! { At::Max => 100 }
							],
						]
					})]
				}
			]
        ]
    ]
}
