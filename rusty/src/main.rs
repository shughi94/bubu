mod routes;
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use log::info;

#[tokio::main]
async fn main() {
    
    // Initialize logger to stdout/stderr
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    info!("Starting server on 0.0.0.0:3000");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server listening on 0.0.0.0:3000");
    axum::serve(listener, routes::create_router().await.layer(cors)).await.unwrap();
}