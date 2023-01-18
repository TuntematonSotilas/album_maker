#[derive(Eq, PartialEq, Debug, Clone, Default)]
pub enum DeleteStatus {
    #[default]
    AskDelete,
    Deleting,
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub delete_status: DeleteStatus,
    pub total: usize,
    pub current: i32,
}
