use std::collections::{HashMap, HashSet};

use crate::session::room::Room;

pub struct MppServer {
    rooms: HashMap<String, Room>
}