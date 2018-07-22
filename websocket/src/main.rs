extern crate futures;
extern crate tsukuyomi;
extern crate tsukuyomi_websocket;

use futures::prelude::*;
use tsukuyomi::{App, Input};
use tsukuyomi_websocket::{start, OwnedMessage};

fn websocket(input: &mut Input) -> tsukuyomi::handler::Handle {
    start(input, |cx| {
        let (sink, stream) = cx.stream.split();
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
    let app = App::builder().route(("/ws", websocket)).finish()?;

    tsukuyomi::run(app)
}
