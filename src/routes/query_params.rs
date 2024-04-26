use axum::{extract::Query, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct QueryParams {
    message: String,
    id: i32,
}

pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}

#[cfg(test)]
mod tests {
    use crate::routes::query_params::QueryParams;

    #[tokio::test]
    async fn test_endpoint_query_params() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/params", axum::routing::get(crate::routes::query_params));

        // initiate the TestClient with the previous declared Router
        let res: axum_test_helper::TestResponse = TestClient::new(app)
            .get("/params?message=hola mundo&id=22")
            .send()
            .await;

        assert_eq!(res.json::<QueryParams>().await, QueryParams { message: "hola mundo".to_owned(), id: 22 });
    }
}
