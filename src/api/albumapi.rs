#![allow(clippy::future_not_send)]

use crate::models::{
    album::Album,
    picture::Picture,
    vars::{BASE_URI, DESTROY_URI, UPLOAD_URI},
};
use hex::ToHex;
use load_dotenv::load_dotenv;
use ring::digest;
use seed::prelude::*;
use web_sys::FormData;

pub async fn get_my_ablums(auth: String) -> Option<Vec<Album>> {
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
            Some(albums)
        }
        _ => None,
    }
}

pub async fn get_album(id: String, auth: String) -> Option<Album> {
    let uri = format!("{BASE_URI}getalbum?id={id}");
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
        }
        _ => None,
    }
}

pub async fn update_album(album: Album, auth: String) -> Option<String> {
    let mut res = None;
    let uri = BASE_URI.to_string() + "editalbum";
    let request = Request::new(uri)
        .method(Method::Put)
        .header(Header::authorization(auth))
        .json(&album)
        .expect("Serialization failed");

    let response = fetch(request).await.expect("HTTP request failed");

    if response.status().is_ok() {
        let res_id = response.json::<String>().await;
        if let Ok(id) = res_id {
            res = Some(id);
        }
    }
    res
}

pub async fn delete_ablum(id: String, auth: String) -> bool {
    let delete_uri = format!("{BASE_URI}deletealbum?id={id}");
    let delete_request = Request::new(delete_uri)
        .header(Header::authorization(auth))
        .method(Method::Delete);

    let delete_response = fetch(delete_request).await.expect("HTTP request failed");
    delete_response.status().code == 204
}

pub async fn upload_picture(form_data: FormData) -> Option<Picture> {
    let mut res = None;
    let uri = UPLOAD_URI.to_string();
    let request = Request::new(uri)
        .method(Method::Post)
        .body(JsValue::from(form_data));

    let response = fetch(request).await.expect("HTTP request failed");
    if response.status().is_ok() {
        let res_pic = response.json::<Picture>().await;
        if let Ok(picture) = res_pic {
            res = Some(picture);
        }
    }
    res
}

pub async fn delete_picture(public_id: String) -> bool {
    load_dotenv!();
    let mut res = false;
    let uri = DESTROY_URI.to_string();
    let apikey = env!("CLD_API_KEY");
    let secret = env!("CLD_API_SECRET");
    let ts = js_sys::Date::now().to_string();

    let to_hash = format!("public_id={public_id}&timestamp={ts}{secret}");
    let digest = digest::digest(&digest::SHA1_FOR_LEGACY_USE_ONLY, to_hash.as_bytes());

    let hash = digest.as_ref();
    let signature = hash.to_hex();

    if let Ok(form_data) = FormData::new() {
        let pub_id_res = form_data.append_with_str("public_id", &public_id);
        let key_res = form_data.append_with_str("api_key", apikey);
        let secret_res = form_data.append_with_str("api_secret", secret);
        let ts_res = form_data.append_with_str("timestamp", &ts);
        let sign_res = form_data.append_with_str("signature", &signature);

        if pub_id_res.is_ok()
            && key_res.is_ok()
            && secret_res.is_ok()
            && ts_res.is_ok()
            && sign_res.is_ok()
        {
            let request = Request::new(uri)
                .method(Method::Post)
                .body(JsValue::from(form_data));

            let response = fetch(request).await.expect("HTTP request failed");
            if response.status().is_ok() {
                res = true;
            }
        }
    }
    res
}
