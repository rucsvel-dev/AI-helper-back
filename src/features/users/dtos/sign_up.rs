use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestSignUp {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseSignUp {
    pub id: i32,
    pub token: String,
}