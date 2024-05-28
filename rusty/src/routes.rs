use axum::{Router, routing::get, Json};
use serde_json::{Value, json};
#[path = "nasa/nasa.rs"] mod nasa;

pub async fn create_router() -> Router {
    Router::new()
    .route("/", get(root))
    .route("/nasa/potd", get(nasa))
}

// root
async fn root() -> Json<Value> {
    Json(json!({ "HELLO": "WORLD" }))
}

// nasa
async fn nasa() -> Json<Value> {
    nasa::potd().await
}