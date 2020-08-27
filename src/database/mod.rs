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

    fn get_user_info(hash: u32) -> User {
        let mut ret = User {
            found: false,
            color: 0,
            name: "".to_string(),
        };

        let mut file = match File::open(hash.to_string()) {
            Ok(file) => file,
            Err(_) => return ret
        };

        ret.found = true;

        file.seek(SeekFrom::Start(0));

        ret.color = match file.read_u32::<LittleEndian>() {
            Ok(uint) => uint,
            Err(_) => return ret
        };
        file.seek(SeekFrom::Start(5));
        file.read_to_string(&mut ret.name);

        return ret;
    }

    fn set_user_info(hash: u32, user: User) {
        let mut file = match File::create(hash.to_string()) {
            Ok(file) => file,
            Err(_) => {
                error!("Could not create file!");
                return;
            }
        };

        file.write_u32::<LittleEndian>(user.color);
        file.write_all(user.name.as_bytes());
    }
}