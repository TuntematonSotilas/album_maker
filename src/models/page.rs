pub const LK_MY_ALBUMS: &str = "my-albums";
pub const LK_NEW_ALBUM: &str = "new-album";
pub const LK_VIEW_ALBUM: &str = "album";
pub const LK_EDIT_ALBUM: &str = "edit-album";
pub const LK_LOGIN: &str = "login";

pub const TITLE_MY_ALBUMS: &str = "My albums";
pub const TITLE_NEW_ALBUM: &str = "New album";
pub const TITLE_EDIT_ALBUM: &str = "Edit";
pub const TITLE_LOGIN: &str = "Sign in to your albums";

#[derive(Debug, Clone)]
pub enum Page {
    MyAlbums,
	NewAlbum,
    EditAlbum,
    ViewAlbum,
    Login,
}
