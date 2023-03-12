use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestDeleteAction {
    pub action_id: i32
}