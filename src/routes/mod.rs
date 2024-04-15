mod hello_word;
mod mirror_body_string;
mod mirror_body_json;


use axum::{body::Body, routing::{get, post}, Router};

use hello_word::hello_word;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(hello_word))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
}
