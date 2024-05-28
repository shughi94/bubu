use axum::{Router, routing::get, Json};
use serde_json::{Value};
#[path = "nasa/nasa.rs"] mod nasa;

pub async fn create_router() -> Router {
    
    Router::new()
    .route("/", get(root))
    .route("/nasa/potd", get(nasa))
    //.route("/foo/bar", get(bubu).post)
}

// which calls one of these handlers
async fn root() { println!("ok"); }

async fn nasa() -> Json<Value> {
    nasa::potd().await
}

// async fn bubu() -> Json<Value> {
//     Json(json!({ "data": 43 }))
// }