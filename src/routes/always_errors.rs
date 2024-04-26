use axum::http::StatusCode;

pub async fn always_errors() -> Result<(), StatusCode> {
    /*
    200 - 299 -> ok
    300 
    400 - 499 -> errores del cliente 
    500 - 599 -> errores server
    y hay uno q es soy una tetera (who knows)
    StatusCode::IM_A_TEAPOT
    */
    //Ok(());
    // 201 recurso creado con exito
    Err(StatusCode::IM_A_TEAPOT)
}


mod test {

    #[tokio::test]
    async fn test_endpoint_always_errors() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::get(crate::routes::always_errors));

        // initiate the TestClient with the previous declared Router
        let client = TestClient::new(app);
        let res: axum_test_helper::TestResponse = client.get("/").send().await;
        assert_eq!(res.status(), axum::http::StatusCode::IM_A_TEAPOT);
    }

}
