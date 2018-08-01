extern crate pretty_env_logger;
extern crate tsukuyomi;
extern crate tsukuyomi_staticfile;

use tsukuyomi::App;
use tsukuyomi_staticfile::StaticFiles;

fn main() -> tsukuyomi::AppResult<()> {
    pretty_env_logger::init();

    let app = App::builder()
        .route(StaticFiles::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")).with_prefix("/public"))
        .finish()?;

    tsukuyomi::run(app)
}
