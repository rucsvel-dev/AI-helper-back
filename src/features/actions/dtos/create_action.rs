use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestCreateAction {
    pub name: String,
    pub description: String,
    pub action_type: String,
    pub provider: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseCreateAction {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub action_type: String,
    pub provider: String,
}