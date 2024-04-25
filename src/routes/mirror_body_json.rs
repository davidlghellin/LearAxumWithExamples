use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MirrorJson {
    message: String,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct MirrorJsonResponse {
    message: String,
    message_from_server: String,
}

impl From<MirrorJson> for MirrorJsonResponse {
    fn from(value: MirrorJson) -> Self {
        MirrorJsonResponse { message: value.message, message_from_server: "Hola desde axum".to_owned() }
    }
}

pub async fn mirror_body_json(Json(body): Json<MirrorJson>) -> Json<MirrorJsonResponse> {
    // dbg!(&body);
    Json(MirrorJsonResponse { message: body.message, message_from_server: "Hola desde axum".to_owned() })
}

mod test {
    use crate::routes::mirror_body_json::{MirrorJson, MirrorJsonResponse};

    #[tokio::test]
    async fn test_endpoint_mirror_body_json() {
        use axum::Router;
        use axum_test_helper::TestClient;

        // you can replace this Router with your own app
        let app = Router::new().route("/", axum::routing::post(crate::routes::mirror_body_json));

        // initiate the TestClient with the previous declared Router
        let client = TestClient::new(app);
        let mirron_json: MirrorJson = MirrorJson { message: "test".to_owned() };

        let res: axum_test_helper::TestResponse =
            client.post("/").json(&mirron_json).send().await;
        let res: MirrorJson = res.json::<MirrorJson>().await;

        let mirror_json_response_expected: MirrorJsonResponse = MirrorJsonResponse::from(mirron_json);
        let mirror_json_response: MirrorJsonResponse = MirrorJsonResponse::from(res);

        assert_eq!(mirror_json_response, mirror_json_response_expected);
    }
}