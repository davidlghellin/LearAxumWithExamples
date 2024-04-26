use axum::extract::Path;

pub async fn path_variables(Path(id): Path<i32>) -> String {
    dbg!(id);
    id.to_string()
}

pub async fn hard_coded_patch() -> String {
    "Tienes 15!".to_owned()
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_endpoint_path_variables() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/num/:id", axum::routing::get(crate::routes::path_variables));

        // initiate the TestClient with the previous declared Router
        let res: axum_test_helper::TestResponse = TestClient::new(app)
            .get("/num/5")
            .send()
            .await;

        assert_eq!(res.text().await, "5".to_owned());
    }

    #[tokio::test]
    async fn test_endpoint_hard_coded_patch() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::get(crate::routes::hard_coded_patch));

        // initiate the TestClient with the previous declared Router
        let res: axum_test_helper::TestResponse = TestClient::new(app)
            .get("/")
            .send()
            .await;

        assert_eq!(res.text().await, "Tienes 15!".to_owned());
    }
}