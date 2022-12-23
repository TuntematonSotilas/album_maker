use super::{picture::Picture, state::TypeDel};
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum UpdateType {
    Title,
    CountFakePictures,
    AddPicture,
    DeletePicture,
    Caption,
    DelState,
    SetGroupCover,
    SetAlbumCover,
}

#[derive(Debug, Clone)]
pub struct GroupUpdate {
    pub upd_type: UpdateType,
    pub id: Uuid,
    pub grp_data: Option<String>,
    pub picture: Option<Picture>,
    pub count_fake_pictures: Option<u32>,
    pub asset_id: Option<String>,
    pub caption: Option<String>,
    pub del_state: Option<TypeDel>,
}
