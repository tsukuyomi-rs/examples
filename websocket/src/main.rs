extern crate futures;
extern crate tsukuyomi;

use futures::prelude::*;
use tsukuyomi::handler::wrap_ready;
use tsukuyomi::websocket::{start, OwnedMessage};
use tsukuyomi::{App, Input, Responder};

fn echo_back(input: &mut Input) -> impl Responder {
    start(input, |transport, _cx| {
        let (sink, stream) = transport.split();
        stream
            .take_while(|m| Ok(!m.is_close()))
            .filter_map(|m| {
                println!("Message from client: {:?}", m);
                match m {
                    OwnedMessage::Ping(p) => Some(OwnedMessage::Pong(p)),
                    OwnedMessage::Pong(_) => None,
                    _ => Some(m),
                }
            })
            .forward(sink)
            .and_then(|(_, sink)| sink.send(OwnedMessage::Close(None)))
            .then(|_| Ok(()))
    })
}

fn main() -> tsukuyomi::AppResult<()> {
    let app = App::builder().route(("/ws", wrap_ready(echo_back))).finish()?;

    tsukuyomi::run(app)
}
