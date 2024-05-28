use axum::{Router, routing::get, Json};
use serde_json::{Value, json};

pub async fn create_router() -> Router {
    
    Router::new()
    .route("/", get(root))
    .route("/foo", get(json).post(bubu))
    .route("/foo/bar", get(bubu))

}

// which calls one of these handlers
async fn root() { println!("ok"); }

async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

async fn bubu() -> Json<Value> {
    Json(json!({ "data": 42 }))
}