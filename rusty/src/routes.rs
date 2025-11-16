use axum::{Router, routing::get, Json};
use serde_json::{Value, json};
use log::info;
#[path = "nasa/nasa.rs"] mod nasa;
#[path = "meal/meal.rs"] mod meal;
#[path = "geo/geo.rs"] mod geo;

pub async fn create_router() -> Router {
    info!("Creating router with routes: /, /nasa/potd, /meal/random");
    Router::new()
    .route("/", get(root))
    .route("/nasa/potd", get(nasa))
    .route("/meal/random", get(meal))
    .route("/geo/random", get(geo))
}

// root
async fn root() -> Json<Value> {
    info!("Root endpoint accessed");
    Json(json!({ "HELLO": "WORLD" }))
}

// nasa
async fn nasa() -> Json<Value> {
    info!("NASA POTD endpoint accessed");
    nasa::potd().await
}

// meal
async fn meal() -> Json<Value> {
    info!("Meal random endpoint accessed");
    meal::random().await
}

// geo
async fn geo() -> Json<Value> {
    info!("Geography random endpoint accessed");
    geo::random().await
}