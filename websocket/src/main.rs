extern crate futures;
#[macro_use]
extern crate failure;
extern crate base64;
extern crate http;
extern crate sha1;
extern crate tokio_codec;
extern crate tokio_io;
extern crate tsukuyomi;
extern crate websocket;

mod ws;

use http::Response;
use tsukuyomi::{handler, App, Input};

fn websocket(input: &mut Input) -> tsukuyomi::Result<Response<()>> {
    ws::start(input)
}

fn main() -> tsukuyomi::AppResult<()> {
    let app = App::builder()
        .route(("/ws", handler::ready_handler(websocket)))
        .finish()?;

    tsukuyomi::run(app)
}
