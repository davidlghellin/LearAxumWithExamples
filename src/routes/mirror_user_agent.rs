use axum::{headers::UserAgent, TypedHeader};

pub async fn mirror_user_agent(TypedHeader(user_agent): TypedHeader<UserAgent>) -> String {
    user_agent.to_string()
}


#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_endpoint_get_json_endpoint() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::get(crate::routes::mirror_user_agent));

        // initiate the TestClient with the previous declared Router
        let client = TestClient::new(app);
        let res: axum_test_helper::TestResponse = client
            .get("/")
            .send()
            .await;
        let result= res.into_inner().content_length();
       // assert_eq!(headers.to_owned(), HeaderMap::new());
    }
}