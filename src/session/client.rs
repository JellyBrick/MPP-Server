use serde::{Deserialize, Serialize};

use crate::database::user::User;

#[derive(Serialize, Deserialize)]
pub(crate) struct Client {
    pub color: u32,
    pub name: String,
    pub _id: String,
    #[serde(rename = "id")]
    pub hash: u32,
    #[serde(skip_serializing)]
    changed: bool,
}

impl Client {
    fn new(hash: u32, id: String, color: u32, name: String) -> Self {
        Self {
            color,
            name,
            _id: id,
            hash,
            changed: false,
        }
    }

    fn get_json() {}

    fn get_db_data(&self) -> User {
        User {
            found: true,
            color: self.color,
            name: self.name.clone(),
        }
    }
}