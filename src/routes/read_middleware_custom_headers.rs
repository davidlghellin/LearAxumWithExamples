use axum::Extension;

#[derive(Clone)]
pub struct HeaderMEssage(pub String);

pub async fn read_middleware_custom_headers(
    Extension(message): Extension<HeaderMEssage>,
) -> String {
    message.0
}
