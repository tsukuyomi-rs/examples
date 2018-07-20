extern crate tsukuyomi;
#[macro_use]
extern crate serde;
extern crate futures;
extern crate http;

use futures::prelude::*;
use http::Method;

use tsukuyomi::handler::{async_handler, ready_handler};
use tsukuyomi::json::{Json, JsonErrorHandler};
use tsukuyomi::output::HttpResponse;
use tsukuyomi::{App, Error, Input};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

impl HttpResponse for User {}

fn get_json(_: &mut Input) -> Json<User> {
    Json(User {
        name: "Sakura Kinomoto".into(),
        age: 13,
    })
}

fn read_json_payload(input: &mut Input) -> impl Future<Item = Json<User>, Error = Error> + Send + 'static {
    input.body_mut().read_all().convert_to::<Json<User>>().map(|user| {
        println!("Received: {:?}", user);
        user
    })
}

fn main() -> tsukuyomi::AppResult<()> {
    let app = App::builder()
        .route(("/", Method::GET, ready_handler(get_json)))
        .route(("/", Method::POST, async_handler(read_json_payload)))
        .error_handler(JsonErrorHandler::new())
        .finish()?;

    tsukuyomi::run(app)
}
