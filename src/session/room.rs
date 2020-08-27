use std::collections::{HashMap, VecDeque};

use crate::json_type::message::Message;
use crate::session::client::Client;

struct CrownInfo {
    owner: Client,
    old_owner: Client,
    start_pos: [f32; 2],
    end_pos: [f32; 2],
    time: i64,
}

struct ClientInfo {
    id: String,
    x: f32,
    y: f32,
}

pub struct Room {
    lobby: bool,
    visible: bool,
    chat: bool,
    crown_solo: bool,
    color: u32,
    crown: CrownInfo,
    ids: HashMap<Client, ClientInfo>,
    chat_log: VecDeque<Message>,
}