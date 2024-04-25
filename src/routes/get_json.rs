use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}

impl Data {
    #![allow(dead_code)]
    fn new(message: String, count: i32, username: String) -> Data {
        Data {
            message: message,
            username: username,
            count: count,
        }
    }
}

pub async fn get_json() -> Json<Data> {
    let data: Data = Data {
        message: "yo soy data".to_owned(),
        count: 48,
        username: "user-name".to_owned(),
    };
    Json(data)
}

#[cfg(test)]
mod tests {
    use crate::routes::get_json::get_json;
    use crate::routes::get_json::Data;

    #[test]
    fn basic_test() {
        let d1 = Data::new("mensaje".to_owned(), 32, "count".to_owned());
        assert_eq!(d1.message, d1.message)
    }
    #[test]
    fn basic_test_2() {
        let d1 = Data::new("mensaje".to_owned(), 32, "count".to_owned());
        assert_eq!(d1.message, "mensaje".to_owned())
    }

    #[tokio::test]
    async fn get_json_test() {
        let json = axum::Json(Data {
            message: "yo soy data".to_owned(),
            count: 48,
            username: "user-name".to_owned(),
        });
        assert_eq!(get_json().await.0, json.0)
    }

    #[tokio::test]
    async fn test_endpoint_get_json_endpoint() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::get(crate::routes::get_json));

        // initiate the TestClient with the previous declared Router
        let client = TestClient::new(app);
        let res: axum_test_helper::TestResponse = client.get("/").send().await;
        
        assert_eq!(
            res.json::<Data>().await,
            Data {
                message: "yo soy data".to_owned(),
                count: 48,
                username: "user-name".to_owned(),
            }
        );
    }
}
