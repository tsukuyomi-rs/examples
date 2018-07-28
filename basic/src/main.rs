extern crate tsukuyomi;

use tsukuyomi::handler::wrap_ready;
use tsukuyomi::{App, Input, Responder};

fn handler(_: &mut Input) -> impl Responder {
    "Hello, world!\n"
}

fn main() -> tsukuyomi::AppResult<()> {
    let app = App::builder().route(("/", wrap_ready(handler))).finish()?;

    tsukuyomi::run(app)
}
