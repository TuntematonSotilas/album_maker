#![allow(clippy::future_not_send)]

use gloo_net::http::{Method, Request};

use crate::models::{
    sharing::{AddViewLike, Sharing},
    vars::{AUTH_HEAD, BASE_URI},
};

pub async fn get_my_sharings(auth: String) -> Option<Vec<Sharing>> {
    let uri = BASE_URI.to_string() + "mysharings";
    let response = Request::new(&uri)
        .header(AUTH_HEAD, &auth)
        .send()
        .await
        .expect("HTTP request failed");

    match response.status() {
        200 => {
            let sharings = response
                .json::<Vec<Sharing>>()
                .await
                .expect("deserialization failed");
            Some(sharings)
        }
        _ => None,
    }
}

pub async fn add_sharing(auth: String, sharing: Sharing) -> Option<String> {
    let mut res = None;
    let uri = BASE_URI.to_string() + "addsharing";
    let response = Request::new(&uri)
        .method(Method::POST)
        .header(AUTH_HEAD, &auth)
        .json(&sharing)
        .expect("Serialization failed")
        .send()
        .await
        .expect("HTTP request failed");

    if response.status() == 200 {
        let res_id = response.json::<String>().await;
        if let Ok(id) = res_id {
            res = Some(id);
        }
    }
    res
}

pub async fn delete_sharing(id: String, auth: String) -> bool {
    let delete_uri = format!("{BASE_URI}deletesharing?id={id}");
    let delete_response = Request::new(&delete_uri)
        .header(AUTH_HEAD, &auth)
        .method(Method::DELETE)
        .send()
        .await
        .expect("HTTP request failed");

    delete_response.status() == 204
}

pub async fn add_view_like(auth: String, add_view_like: AddViewLike) {
    let uri = BASE_URI.to_string() + "addviewlike";
    _ = Request::new(&uri)
        .method(Method::POST)
        .header(AUTH_HEAD, &auth)
        .json(&add_view_like)
        .expect("Serialization failed")
        .send()
        .await
        .expect("HTTP request failed");
}
