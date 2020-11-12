use std::fs;
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::*;
use pretty_env_logger;

use crate::database::user::User;

pub(crate) mod user;

struct Database {
    directory: String
}

impl Database {
    fn new(directory: String) -> Self {
        fs::create_dir_all(directory.clone());

        Self {
            directory
        }
    }

    fn get_user_info(&self, _id: String) -> User {
        let mut ret = User {
            color: "".to_string(),
            name: "".to_string(),
            _id,
        };

        return match File::open(_id.to_string() + ".json") {
            Ok(file) => serde_json::from_reader(file)?,
            Err(_) => ret
        };
    }

    fn set_user_info(&self, user: User) {
        match File::create(user._id + ".json") {
            Ok(file) => serde_json::to_writer(file, &user),
            Err(_) => {
                error!("Could not create file!");
            }
        };
    }
}