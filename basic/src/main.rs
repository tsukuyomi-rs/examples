extern crate tsukuyomi;

use tsukuyomi::handler::ready_handler;
use tsukuyomi::App;

fn main() -> tsukuyomi::AppResult<()> {
    let app = App::builder()
        .route(("/", ready_handler(|_| "Hello, world!\n")))
        .finish()?;

    tsukuyomi::run(app)
}
