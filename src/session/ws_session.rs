use std::time::Instant;

use actix::Addr;

use crate::server;

pub(crate) struct WsSession {
    /// unique session id
    pub(crate) id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection?
    pub(crate) heartbeat: Instant,
    /// joined room
    pub(crate) room: String,
    /// peer name
    pub(crate) name: Option<String>,
    /// server
    pub(crate) addr: Addr<server::MppServer>,
}