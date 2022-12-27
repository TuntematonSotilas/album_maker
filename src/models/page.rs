pub const LK_MY_ALBUMS: &str = "my-albums";
pub const LK_NEW_ALBUM: &str = "new-album";
pub const LK_VIEW_ALBUM: &str = "album";
pub const LK_EDIT_ALBUM: &str = "edit-album";
pub const LK_SLIDESHOW: &str = "slideshow";
pub const LK_LOGIN: &str = "login";
pub const LK_MY_SHARINGS: &str = "my-sharings";
pub const LK_SHARE: &str = "share";
pub const LK_SHARESLIDE: &str = "shareslide";

pub const TITLE_MY_ALBUMS: &str = "My albums";
pub const TITLE_MY_SHARINGS: &str = "My sharings";
pub const TITLE_NEW_ALBUM: &str = "New album";
pub const TITLE_EDIT_ALBUM: &str = "Edit album";
pub const TITLE_SLIDESHOW: &str = "Slideshow";
pub const TITLE_LOGIN: &str = "Sign in to your albums";

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Page {
    MyAlbums,
    NewAlbum,
    EditAlbum,
    ViewAlbum,
    Slideshow,
    Login,
    MySharings,
    Share,
	ShareSlide,
}
