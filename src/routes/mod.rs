mod hello_word;

use axum::{body::Body, routing::get, Router};

use hello_word::hello_word;

pub fn create_routes() -> Router<Body> {
    Router::new().route("/", get(hello_word))
}
