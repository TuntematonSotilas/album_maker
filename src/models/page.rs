use seed::{prelude::*};

pub const LK_MY_ALBUMS: &str = "my-albums";
pub const LK_NEW_ALBUM: &str = "new-album";

pub const TXT_NEW_ALBUM: &str = "New Album";

#[derive(Debug, Clone)]
pub enum Page {
	MyAlbums,
	NewAlbum
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::MyAlbums,
			Some(LK_MY_ALBUMS) => Self::MyAlbums,
            Some(LK_NEW_ALBUM) => Self::NewAlbum,
			Some(_) => Self::MyAlbums,
        }
    }
}
