#[derive(Eq, PartialEq, Debug, Clone)]
pub enum TypeDel {
    AskDelete,
    Deleting,
}

#[derive(Debug, Clone)]
pub struct State {
    pub del_state: TypeDel,
    pub total: usize,
    pub current: i32,
}
