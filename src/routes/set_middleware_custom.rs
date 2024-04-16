use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

use super::read_middleware_custom_headers::HeaderMEssage;

pub async fn set_middleware_custom<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_err| StatusCode::BAD_REQUEST)?
        .to_owned();

    let extension = request.extensions_mut();
    extension.insert(HeaderMEssage(message.to_owned()));

    Ok(next.run(request).await)
}
