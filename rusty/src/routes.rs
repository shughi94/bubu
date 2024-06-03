use axum::{Router, routing::get, Json};
use serde_json::{Value, json};
#[path = "nasa/nasa.rs"] mod nasa;
#[path = "meal/meal.rs"] mod meal;

pub async fn create_router() -> Router {
    Router::new()
    .route("/", get(root))
    .route("/nasa/potd", get(nasa))
    .route("/meal/random", get(meal))
}

// root
async fn root() -> Json<Value> {
    Json(json!({ "HELLO": "WORLD" }))
}

// nasa
async fn nasa() -> Json<Value> {
    nasa::potd().await
}

// meal
async fn meal() -> Json<Value> {
    meal::random().await
}