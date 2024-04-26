use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn returns_201() -> Response {
    (StatusCode::CREATED, "Devolvemos 201").into_response()
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_endpoint_returns_201_status() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::get(crate::routes::returns_201));

        // initiate the TestClient with the previous declared Router
        let res: axum_test_helper::TestResponse = TestClient::new(app)
            .get("/")
            .send()
            .await;

        assert_eq!(res.status(), axum::http::StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_endpoint_returns_201_text() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::get(crate::routes::returns_201));

        // initiate the TestClient with the previous declared Router
        let res: axum_test_helper::TestResponse = TestClient::new(app)
            .get("/")
            .send()
            .await;

        assert_eq!(res.text().await, "Devolvemos 201".to_owned());
    }
}