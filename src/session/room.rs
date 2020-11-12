use std::collections::{HashMap, VecDeque};

use crate::database::user::User;
use crate::json_type::message::ResponseMessage;

struct CrownInfo {
    owner: User,
    old_owner: User,
    start_pos: [f32; 2],
    end_pos: [f32; 2],
    time: i64,
}

struct CursorInfo {
    id: String,
    x: f32,
    y: f32,
}

struct RoomSetting {
    lobby: bool,
    visible: bool,
    chat: bool,
    owner_only: bool,
    color: u32,
}

pub struct Room {
    setting: RoomSetting,
    crown: CrownInfo,
    user: HashMap<User, CursorInfo>,
    chat_log: VecDeque<ResponseMessage>,
}