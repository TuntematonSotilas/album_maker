use crate::models::{album::Album, vars::BASE_URI};
use seed::{self, prelude::*};

pub async fn get_album(id: String, auth: String) -> Option<Album>
{
	let uri = format!("{}getalbum?id={}", BASE_URI, id);
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
			Some(album)
		},
		_ => None
	}
}