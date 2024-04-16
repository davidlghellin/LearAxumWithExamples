use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    BoxError, Json,
};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct RequestUserVali {
    #[validate(
        email(message = "Usuario invalido"),
        contains(pattern = "gmail", message = "tiene q ser gmail")
    )]
    pub user: String,
    #[validate(length(min = 8, message = "pass demasiado corta"))]
    pub pass: String,
}

pub async fn custom_json_extractor(user: RequestUserVali) {
    dbg!(user);
}

#[async_trait]
impl<B> FromRequest<B> for RequestUserVali
where
    B: HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>, // required by `async_trait`
{
    type Rejection = (StatusCode, String);

    async fn from_request(requesq: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(user) = requesq
            .extract::<Json<RequestUserVali>>()
            .await
            .map_err(|error| (StatusCode::BAD_REQUEST, format!("{}", error)))?;

        // nos valida y nos da todos los errores
        if let Err(error) = user.validate() {
            return Err((StatusCode::BAD_REQUEST, format!("{}", error)));
        }

        Ok(user)
    }
}
