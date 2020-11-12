use crate::database::user::User;
use crate::session::room::Room;

struct Participant {
    _id: String,
    name: String,
    color: String,
    room: Option<Room>,
    updates: bool,
}

impl Participant {
    pub fn new(_id: String, name: String, color: String) -> Self {
        Self {
            _id,
            name,
            color,
            room: None,
            updates: false,
        }
    }

    pub fn get_user(&self) -> User {
        Database::new("database/").get_user_info(self._id.clone())
    }

    pub fn set_user(&self) {
        Database::new("database/").set_user_info(
            User {
                color,
                name,
                _id,
            }
        )
    }

    pub fn get_json(&self) -> String {
        self.get_user().to_string()
    }
}