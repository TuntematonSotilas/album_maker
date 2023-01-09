#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub enum TypeDel {
    #[default]
    AskDelete,
    Deleting,
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub del_state: TypeDel,
    pub total: usize,
    pub current: i32,
}
