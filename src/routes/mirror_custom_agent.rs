use axum::http::HeaderMap;

pub async fn mirror_custom_agent(hearders: HeaderMap) -> String {
    let message_value: &axum::http::HeaderValue = hearders.get("x-message").unwrap();
    let message = message_value.to_str().unwrap().to_owned();
    message
}
