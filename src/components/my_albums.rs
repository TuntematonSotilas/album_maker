use seed::{self, prelude::*, *};

use crate::models::{album::Album, page::TITLE_MY_ALBUMS, vars::BASE_URI};

// ------ ------
//     Model
// ------ -----
#[derive(Default)]
pub struct Model {
    auth_header: String,
    albums: Option<Vec<Album>>,
	album_id_to_delete: Option<String>,
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp,
    Received(Vec<Album>),
    ErrorGet,
    Delete(String),
	AskDelete(String),
	ErrorDelete,
	SuccessDelete,
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
                    _ => Msg::ErrorGet,
                }
            });
        }
        Msg::ErrorGet => {
            error!("Error getting albums");
        }
		Msg::Received(albums) => {
            model.albums = Some(albums);
        }
		Msg::AskDelete(id) => {
			model.album_id_to_delete = Some(id);
		}
        Msg::Delete(id) => {
            let auth = model.auth_header.clone();
			let uri = BASE_URI.to_string() + "deletealbum?id=" + id.as_str();
            orders.skip(); // No need to rerender
            orders.perform_cmd(async {
                let response = Request::new(uri)
                    .header(Header::authorization(auth))
                    .fetch()
                    .await
                    .expect("HTTP request failed");

                match response.status().code {
                    204 => Msg::SuccessDelete,
                    _ => Msg::ErrorDelete,
                }
            });
        }
		Msg::ErrorDelete => {
            error!("Error deleting albums");
        }
		Msg::SuccessDelete => {
            error!("Error deleting albums");
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
            C!("box"),
            p![C!["title", "is-5", "has-text-link"], TITLE_MY_ALBUMS],
            if model.albums.is_some() {
               div![model.albums.as_ref().unwrap().iter().map(|album| {
				   let id = album.id.clone();
                    p![
                        C!("panel-block"),
                        div![
                            C!["container", "columns", "is-mobile"],
                            a![
								C!["column", "is-three-quarters"], 
								attrs! { 
									At::Title => "Open",
									At::Href => "/album/".to_string() + id.as_str() 
								},
								&album.title
							],
                            div![
                                C!("column"),
								button![
									C!["button", "is-link", "is-light", "is-small"],
									span![C!("icon"), i![C!("ion-close-circled")]],
									span!["Delete"],
									ev(Ev::Click,|_| Msg::AskDelete(id)),
								]
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
}
