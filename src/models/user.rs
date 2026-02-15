use serde_derive::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct User {
    id: Option<i32>,
    name: String,
}