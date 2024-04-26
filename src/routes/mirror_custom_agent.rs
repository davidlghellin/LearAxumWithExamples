use axum::http::HeaderMap;

pub async fn mirror_custom_agent(hearders: HeaderMap) -> String {
    let message_value: &axum::http::HeaderValue = hearders.get("x-message").unwrap();
    let message = message_value.to_str().unwrap().to_owned();
    message
}

#[cfg(test)]
mod tests {
    use axum::headers::HeaderMap;

    #[tokio::test]
    async fn test_endpoint_get_json_endpoint() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::get(crate::routes::mirror_custom_agent));

        // initiate the TestClient with the previous declared Router
        let client = TestClient::new(app);
        let res: axum_test_helper::TestResponse = client
            .get("/")
            .header("x-message".to_owned(), "la cabecera".to_owned())
            .header("value".to_owned(), "value cabecera".to_owned())
            .header("x-my-hdr", "abc".to_owned())
            .send()
            .await;


        let headers = res.into_inner();
        let headers: &HeaderMap = headers.headers();

        let mut headers_exp = HeaderMap::new();
        headers_exp.insert("x-my-hdr", "abc".parse().unwrap());
        // assert_eq!(headers.to_owned(), headers_exp);
    }
}