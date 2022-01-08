use seed::{prelude::*};

pub const MY_ALBUMS: &str = "my-albums";
pub const NEW_ALBUM: &str = "new-album";

#[derive(Debug, Clone)]
pub enum Page {
	MyAlbums,
	NewAlbum
}

impl Page {
    pub fn init(mut url: Url) -> Self {
        match url.next_path_part() {
            None => Self::MyAlbums,
			Some(MY_ALBUMS) => Self::MyAlbums,
            Some(NEW_ALBUM) => Self::NewAlbum,
			Some(_) => Self::MyAlbums,
        }
    }
}
