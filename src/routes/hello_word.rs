pub async fn hello_word() -> String {
    "Hola mundo desde mi fichero".to_owned()
}

#[tokio::test]
async fn test_something_async() {
    assert_eq!(hello_word().await, "Hola mundo desde mi fichero".to_string())
}


mod test {
    #[tokio::test]
    async fn test_endpoint_hello() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::get(crate::routes::hello_word));

        // initiate the TestClient with the previous declared Router
        let client = TestClient::new(app);
        let res: axum_test_helper::TestResponse = client.get("/").send().await;
        assert_eq!(res.text().await, "Hola mundo desde mi fichero".to_owned());
    }
}