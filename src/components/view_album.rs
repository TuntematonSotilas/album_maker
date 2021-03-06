use crate::models::{
    album::Album,
    vars::{BASE_URI, VIEW_URI},
};
use seed::{self, prelude::*, *};

// ------ ------
//     Model
// ------ -----
pub struct Model {
    auth_header: String,
    album: Album,
}

impl Model {
    pub const fn new() -> Self {
        Self {
            auth_header: String::new(),
            album: Album::new(),
        }
    }
}

// ------ ------
//    Update
// ------ ------
pub enum Msg {
    SetAuth(String),
    InitComp(String),
    ErrorGet,
    Received(Album),
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::SetAuth(auth_header) => model.auth_header = auth_header,
        Msg::InitComp(id) => {
            orders.skip(); // No need to rerender
            let auth = model.auth_header.clone();
            let uri = BASE_URI.to_string() + "getalbum?id=" + id.as_str();
            orders.perform_cmd(async {
                let response = Request::new(uri)
                    .header(Header::authorization(auth))
                    .fetch()
                    .await
                    .expect("HTTP request failed");

                match response.status().code {
                    200 => {
                        let album = response
                            .json::<Album>()
                            .await
                            .expect("deserialization failed");
                        Msg::Received(album)
                    }
                    _ => Msg::ErrorGet,
                }
            });
        }
        Msg::ErrorGet => {
            error!("Error getting albums");
        }
        Msg::Received(album) => {
            model.album = album;
        }
    }
}

// ------ ------
//     View
// ------ ------
pub fn view(model: &Model) -> Node<Msg> {
    div![
        C!["column", "is-two-thirds"],
        p![C!["title", "is-5", "has-text-link"], &model.album.title],
        match &model.album.groups {
            Some(groups) => div![groups.iter().map(|group| {
                div![
                    C!("box"),
                    p![C!["title", "is-6", "has-text-link"], &group.title],
					p![C!["subtitle", "is-7", "has-text-primary	"], &group.description],
					div![
						match &group.pictures {
							Some(pictures) => div![
								C!["is-flex", "is-flex-wrap-wrap", "is-justify-content-center"],
								pictures.iter().map(|picture| {
									div![
									
										figure![
											C!["image", "pic-view", "m-1"],
											img![attrs! { At::Src =>
												VIEW_URI.to_string() +
												picture.public_id.as_str() +
												"." +
												picture.format.as_str()
											}]
										],
										span![picture.caption.clone().unwrap_or_default()],
									]
								}
							)],
							None => empty![],
                    	}
					]
                ]
            })],
            None => empty![],
        },
    ]
}
