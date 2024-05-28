use axum::Json;
use std::fs;
use serde_json::{Value, json};

pub async fn potd() -> Json<Value> {

    let the_file = "ayylmao/potd.json";

    let file = fs::File::open(the_file).expect("file should open read only");
    let json: serde_json::Value = serde_json::from_reader(file).expect("file should be proper JSON");

    Json(json!(json))
}