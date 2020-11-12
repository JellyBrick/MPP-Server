use std::fs;
use std::fs::File;
use std::io::{Error, Read, Seek, SeekFrom, Write};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use log::*;
use pretty_env_logger;

use crate::database::user::User;

pub(crate) mod user;

pub(crate) struct Database {
    directory: String
}

impl Database {
    pub(crate) fn new(directory: String) -> Self {
        fs::create_dir_all(directory.clone());

        Self {
            directory
        }
    }

    pub(crate) fn get_user_info(&self, _id: String) -> User {
        return match File::open(_id.clone().to_string() + ".json") {
            Ok(file) => serde_json::from_reader(file).expect("Could not open file!"),
            Err(_) => User {
                color: "".to_string(),
                name: "".to_string(),
                _id,
            }
        };
    }

    pub(crate) fn set_user_info(&self, user: User) {
        match File::create(user._id.clone() + ".json") {
            Ok(file) => {
                serde_json::to_writer(file, &user).expect("Could not write file!");
            },
            Err(_) => {
                error!("Could not create file!");
            }
        };
    }
}