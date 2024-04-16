use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    user: String,
    pass: String,
    otro: Option<String>,
}

pub async fn validate_data_with_serde(Json(user): Json<RequestUser>) {
    dbg!(user);
  /*  Json(RequestUser{
        username:user.username,
        password:"Secre".to_owned(),
    });*/
}
