pub async fn mirror_body_string(body: String) -> String {
    // si queremos extraer solamente el body como cadena
    body
}

mod test {

    #[tokio::test]
    async fn test_endpoint_mirror_body_string() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::post(crate::routes::mirror_body_string));

        // initiate the TestClient with the previous declared Router
        let client = TestClient::new(app);
        let res: axum_test_helper::TestResponse =
            client.post("/").body("hola".to_owned()).send().await;

        let res = res.text().await;
        assert_eq!(res, "hola".to_owned());
    }
}
