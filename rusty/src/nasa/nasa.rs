use axum::{Json};
use serde_json::{Value, json};

pub async fn potd() -> Json<Value> {
    Json(json!({ "data": 42 }))
}