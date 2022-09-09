use super::picture::Picture;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum UpdateType {
    Title,
	Description,
    CountFakePictures,
    AddPicture,
	DeletePicture,
    Caption,
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
}
