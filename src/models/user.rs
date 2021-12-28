use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    nickname: String,
    name: String,
    picture: String,
    updated_at: String,
    email: String,
    email_verified: bool,
    sub: String,
}