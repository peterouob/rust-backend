use serde_derive::{Deserialize, Serialize};
use uuid::{uuid, Uuid};

#[derive(Serialize,Deserialize,Debug)]
pub struct User {
    id: Option<Uuid>,
    name: String,
    password: String
}

impl User {
    pub fn new(name: String,password: String) -> Self {
        User{
            id: Some(Uuid::new_v4()),
            name,
            password
        }
    }
}