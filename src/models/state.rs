#[derive(Eq, PartialEq)]
pub enum TypeDel {
    AskDelete,
    Deleting,
}

pub struct State {
    pub del_state: TypeDel,
    pub total: usize,
    pub current: i32,
}
