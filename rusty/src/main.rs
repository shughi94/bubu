mod routes;
use tower_http::cors::{Any, CorsLayer};
use http::Method;
use log::info;

#[tokio::main]
async fn main() {
    // Initialize logger to write to a file
    flexi_logger::Logger::try_with_env_or_str("info")
        .unwrap()
        .log_to_file(flexi_logger::FileSpec::default().directory("logs").basename("app"))
        .rotate(
            flexi_logger::Criterion::Size(10 * 1024 * 1024), // 10 MB
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(7), // Keep 7 log files
        )
        .format(flexi_logger::detailed_format)
        .start()
        .unwrap();

    info!("Starting server on 0.0.0.0:3000");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server listening on 0.0.0.0:3000");
    axum::serve(listener, routes::create_router().await.layer(cors)).await.unwrap();
}