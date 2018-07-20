extern crate tsukuyomi;

use tsukuyomi::handler::ready_handler;
use tsukuyomi::App;

fn main() -> tsukuyomi::AppResult<()> {
    let app = App::builder()
        .route(("/", ready_handler(|_| "Hello, world\n")))
        .mount("/api/v1/", |scope| {
            scope.mount("/posts", |scope| {
                scope.route((
                    "/:id",
                    ready_handler(|input| format!("get_post(id = {})", &input.params()[0])),
                ));

                scope.route(("/", ready_handler(|_| "list_posts")));

                scope.route(("/", "POST", ready_handler(|_| "add_post")));
            });

            scope.mount("/user", |scope| {
                scope.route(("/auth", ready_handler(|_| "Authentication")));
            });
        })
        .route((
            "/static/*path",
            ready_handler(|input| format!("path = {}\n", &input.params()[0])),
        ))
        .finish()?;

    tsukuyomi::run(app)
}
