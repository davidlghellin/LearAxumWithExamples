use axum::{routing::get, Router};

#[tokio::main]
async fn main() {

    let app: Router = Router::new().route("/", get(||async   {"Hola mundo"}      ));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    
}
