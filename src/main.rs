#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use std::{env, process, thread};
use std::borrow::Borrow;
use std::error::Error;

use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite::handshake::server::{ErrorResponse, Response};
use tungstenite::handshake::server::Request;

pub mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let try_socket = TcpListener::bind(bind_address.clone()).await;
    let mut listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", bind_address);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");

    let mut client_ip = addr.ip().to_string();

    let callback = |request: &Request, response: Response| -> Result<Response, ErrorResponse> {
        if let Some(header) = request.headers().get("X-Real-IP") {
            client_ip = match header.to_str() {
                Ok(s) => s.to_owned(),
                Err(_) => client_ip
            }
        }

        let (mut parts, body) = response.into_parts();

        let response_with_protocol = Response::from_parts(parts, body);

        Ok(response_with_protocol)
    };

    let ws_stream = tokio_tungstenite::accept_hdr_async(stream, callback)
        .await
        .expect("Error during the websocket handshake occurred");
}