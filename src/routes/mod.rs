mod hello_word;
mod mirror_body_string;


use axum::{body::Body, routing::{get, post}, Router};

use hello_word::hello_word;
use mirror_body_string::mirror_body_string;

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(hello_word))
        .route("/mirror_body_string", post(mirror_body_string))
}
