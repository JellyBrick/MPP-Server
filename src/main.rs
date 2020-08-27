use std::{env, process, thread};
use std::borrow::Borrow;
use std::time::Instant;

use actix::{Actor, Addr, StreamHandler};
use actix_web::{App, Error, HttpRequest, HttpResponse, HttpServer, web};
use actix_web_actors::ws;
use log::*;
use pretty_env_logger;

use crate::session::ws_session;

mod utils;
mod database;
mod session;
mod server;
mod json_type;

/// Define http actor
struct WebSocket;

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

async fn mpp_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::MppServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        ws_session::WsSession {
            id: 0,
            heartbeat: Instant::now(),
            room: "lobby".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
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
    HttpServer::new(|| App::new().route("/", web::get().to(mpp_route)))
        .bind(bind_address.clone())?
        .run()
        .await
}