use std::{env, process, thread};
use std::borrow::Borrow;
use std::net::TcpListener;
use std::time::Instant;

use log::*;
use pretty_env_logger;
use tungstenite::accept;

mod utils;
mod database;
mod session;
mod json_type;

fn main() {
    pretty_env_logger::init();
    let mut args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        error!("Usage: {} ADMIN_PASSWORD [PORT {{1234}}] [DEFAULT_COLOR {{FF7F00}}] [LISTEN_ADDR]", args[0]);
        process::exit(1);
    }

    let password = &args[1];
    let mut port: u16 = 1234;
    let mut input_address = String::from("0.0.0.0");
    let mut room_color: u64;

    if args.len() >= 3 {
        let parsed_port = (&args[2]).parse::<u16>();

        port = match parsed_port {
            Ok(p) => p,
            Err(error) => {
                error!("Invalid port: {}", error);
                process::exit(1);
            }
        };
    }
    if args.len() >= 4 {
        if args[3].as_bytes()[0] == '#' as u8 {
            args[3].remove(0);
        }
        room_color = u64::from_str_radix(args[3].borrow(), 16).unwrap();

        info!("Set default room color to: {}", room_color.to_string());
    }
    if args.len() >= 5 {
        input_address = args[4].to_string();
    }

    let bind_address = input_address + ":" + &port.to_string();

    info!("Listening on: {}", bind_address);
    let server = TcpListener::bind(bind_address).unwrap();

    for stream in server.incoming() {
        std::thread::spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            loop {
                let msg = websocket.read_message().unwrap();
                if msg.is_binary() {} else if msg.is_text() {
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}