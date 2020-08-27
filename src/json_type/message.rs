use serde::{Deserialize, Serialize};

use crate::session::client::Client;

#[derive(Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "m")]
    type_name: String,
    #[serde(rename = "a")]
    content: String,
    #[serde(rename = "p")]
    sender: Client,
    #[serde(rename = "t")]
    time: u64,
}