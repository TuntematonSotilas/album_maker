use serde::{Deserialize, Serialize};

use super::{group::Group, caption::{CaptionStyle, CaptionColor}};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Album {
    #[serde(rename(deserialize = "_id"))]
    pub id: String,
    pub title: String,
	pub caption_style: CaptionStyle,
	pub caption_color: CaptionColor, 
    pub groups: Option<Vec<Group>>,
}

impl Album {
    pub const fn new() -> Self {
        Self {
            id: String::new(),
            title: String::new(),
			caption_style: CaptionStyle::Round,
			caption_color: CaptionColor::Black,
            groups: Some(Vec::new()),
        }
    }
}
