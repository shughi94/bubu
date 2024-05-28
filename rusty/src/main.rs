mod routes;
use tower_http::cors::{Any, CorsLayer};
use http::Method;

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes::create_router().await.layer(cors)).await.unwrap();
}