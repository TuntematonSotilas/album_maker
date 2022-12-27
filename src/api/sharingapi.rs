#![allow(clippy::future_not_send)]

use crate::models::{
    sharing::Sharing,
    vars::BASE_URI,
};
use seed::prelude::*;

pub async fn get_my_sharings(auth: String) -> Option<Vec<Sharing>> {
    let uri = BASE_URI.to_string() + "mysharings";
    let response = Request::new(uri)
        .header(Header::authorization(auth))
        .fetch()
        .await
        .expect("HTTP request failed");

    match response.status().code {
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
    let request = Request::new(uri)
        .method(Method::Post)
        .header(Header::authorization(auth))
        .json(&sharing)
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

pub async fn delete_sharing(id: String, auth: String) -> bool {
    let delete_uri = format!("{BASE_URI}deletesharing?id={id}");
    let delete_request = Request::new(delete_uri)
        .header(Header::authorization(auth))
        .method(Method::Delete);

    let delete_response = fetch(delete_request).await.expect("HTTP request failed");
    delete_response.status().code == 204
}