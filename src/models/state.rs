
#[derive(PartialEq)]
pub enum DeleteState {
    AskDelete,
    Deleting,
}

pub struct State {
    pub del_state: DeleteState,
    pub total: usize,
    pub current: i32,
}