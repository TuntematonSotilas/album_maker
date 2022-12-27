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
            let albums = response
                .json::<Vec<Sharing>>()
                .await
                .expect("deserialization failed");
            Some(albums)
        }
        _ => None,
    }
}