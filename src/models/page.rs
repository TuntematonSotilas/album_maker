pub const LK_MY_ALBUMS: &str = "my-albums";
pub const LK_NEW_ALBUM: &str = "new-album";
pub const LK_LOGIN: &str = "login";

pub const TITLE_MY_ALBUMS: &str = "My Albums";
pub const TITLE_NEW_ALBUM: &str = "New Album";
pub const TITLE_LOGIN: &str = "Sign in to your albums";

#[derive(Debug, Clone)]
pub enum Page {
	MyAlbums,
	NewAlbum,
	Login,
}