use serde::{Deserialize, Serialize};

use crate::database::user::User;
use crate::session::client::Client;

#[derive(Serialize, Deserialize)]
pub struct RequestMessage {
    #[serde(rename = "m")]
    type_name: String,
    #[serde(rename = "a")]
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseMessage {
    #[serde(rename = "m")]
    type_name: String,
    #[serde(rename = "a")]
    content: String,
    #[serde(rename = "p")]
    sender: User,
    #[serde(rename = "t")]
    time: u64,
}