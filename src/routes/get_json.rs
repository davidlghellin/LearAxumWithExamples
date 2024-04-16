use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

pub async fn get_json() -> Json<Data> {
    let data: Data = Data {
        message: "yo soy data".to_owned(),
        count: 48,
        username: "user-name".to_owned(),
    };
    Json(data)
}
