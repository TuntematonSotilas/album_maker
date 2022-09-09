use std::time::SystemTime;

use crate::models::{album::Album, vars::{BASE_URI, DESTROY_URI}};
use seed::{self, prelude::*, *};
use crypto::{sha1::Sha1, digest::Digest};
use web_sys::FormData;

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

pub async fn delete_picture(public_id: String)
{
	let mut ts = String::new();
	let mut signature = String::new();

	let uri = DESTROY_URI.to_string();
	let secret = env!("CLD_API_SECRET");
	let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);
	if let Ok(duration) = now {
		ts = duration.as_millis().to_string();
		let to_hash = format!("{0}&{1}{2}", public_id, ts, secret);
		let mut hasher = Sha1::new();
			hasher.input_str(&to_hash);
		signature = hasher.result_str();
	}
	
	if let Ok(form_data) = FormData::new() {	
		let pub_id_res = form_data.append_with_str("public_id", &public_id);
		let key_res = form_data.append_with_str("api_key", env!("CLD_UPLOAD_PRESET"));
		let secret_res = form_data.append_with_str("api_secret", secret);
		let ts_res = form_data.append_with_str("timestamp", &ts);
		let sign_res = form_data.append_with_str("timestamp", &signature);

		if pub_id_res.is_ok() && key_res.is_ok() && secret_res.is_ok() && ts_res.is_ok() && sign_res.is_ok() {
			let request = Request::new(uri)
				.method(Method::Post)
				.body(JsValue::from(form_data));
			
			let response = fetch(request).await.expect("HTTP request failed");
			if response.status().is_ok() {
				log!("Delete Pic Ok")
			}
		}
	}
}