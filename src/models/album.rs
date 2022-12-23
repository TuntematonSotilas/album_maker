use serde::{Deserialize, Serialize};

use super::{
    caption::{Color, Style},
    group::Group,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub title: String,
    pub caption_style: Style,
    pub caption_color: Color,
    pub groups: Option<Vec<Group>>,
    pub cover: String,
}

impl Album {
    pub const fn new() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
            caption_style: Style::Round,
            caption_color: Color::Black,
            groups: Some(Vec::new()),
            cover: String::new(),
        }
    }
}
