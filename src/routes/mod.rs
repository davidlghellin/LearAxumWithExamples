mod hello_word;
mod mirror_body_string;
mod mirror_body_json;
mod path_variables;
mod query_params;
mod mirror_user_agent;
mod mirror_custom_agent;

use axum::{body::Body, http::Method, routing::{get, post}, Router};
use tower_http::cors::{Any, CorsLayer};

use hello_word::hello_word;
use mirror_body_string::mirror_body_string;
use mirror_body_json::mirror_body_json;
use path_variables::{hard_coded_patch, path_variables};
use query_params::query_params;
use mirror_user_agent::mirror_user_agent;
use mirror_custom_agent::mirror_custom_agent;

pub fn create_routes() -> Router<Body> {
    let cors = CorsLayer::new()
    // allow `GET` and `POST` when accessing the resource
    .allow_methods([Method::GET, Method::POST])
    // allow requests from any origin
    .allow_origin(Any);

    Router::new()
        .route("/", get(hello_word))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        // se ejecutará la de 15, porque el match es más exaustivo
        .route("/path_variables/15", get(hard_coded_patch))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_agent", get(mirror_custom_agent))
        .layer(cors)
}
