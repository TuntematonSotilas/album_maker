use super::picture::Picture;
use uuid::Uuid;

#[derive(Debug, Clone, Copy)]
pub enum UpdateType {
    Title,
    CountFakePictures,
    AddPicture,
}

#[derive(Debug, Clone)]
pub struct GroupUpdate {
    pub upd_type: UpdateType,
    pub id: Option<Uuid>,
    pub title: Option<String>,
    pub picture: Option<Picture>,
    pub count_fake_pictures: Option<u32>,
}
