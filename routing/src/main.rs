extern crate tsukuyomi;

use tsukuyomi::handler::wrap_ready;
use tsukuyomi::App;

fn main() -> tsukuyomi::AppResult<()> {
    let app = App::builder()
        .route(("/", wrap_ready(|_| "Hello, world\n")))
        .mount("/api/v1/", |scope| {
            scope.mount("/posts", |scope| {
                scope.route((
                    "/:id",
                    wrap_ready(|input| format!("get_post(id = {})", &input.params()[0])),
                ));

                scope.route(("/", wrap_ready(|_| "list_posts")));

                scope.route(("/", "POST", wrap_ready(|_| "add_post")));
            });

            scope.mount("/user", |scope| {
                scope.route(("/auth", wrap_ready(|_| "Authentication")));
            });
        })
        .route((
            "/static/*path",
            wrap_ready(|input| format!("path = {}\n", &input.params()[0])),
        ))
        .finish()?;

    tsukuyomi::run(app)
}
