use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    nickname: String,
    pub name: String,
    picture: String,
    updated_at: String,
    email: String,
    email_verified: bool,
    sub: String,
}