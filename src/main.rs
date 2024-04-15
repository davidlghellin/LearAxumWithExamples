use axum::{routing::get, Router};

#[tokio::main]
async fn main() {

    let app: Router = Router::new().route("/", get(hola_mundo));
    
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    
}

async fn hola_mundo() -> String{
    "Hola mundo".to_owned()
}
