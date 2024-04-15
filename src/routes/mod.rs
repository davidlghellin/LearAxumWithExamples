mod hello_word;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;

use axum::{body::Body, routing::{get, post}, Router};

use hello_word::hello_word;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::{hard_coded_patch, path_variables};

pub fn create_routes() -> Router<Body> {
    Router::new()
        .route("/", get(hello_word))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        // se ejecutará la de 15, porque el match es más exaustivo
        .route("/path_variables/15", get(hard_coded_patch))
}
