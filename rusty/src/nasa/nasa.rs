use axum::Json;
use std::fs;
use serde_json::{Value, json};
use log::{info, error};

pub async fn potd() -> Json<Value> {

    let the_file = "ayylmao/potd.json";
    info!("Reading NASA POTD from file: {}", the_file);

    let file = fs::File::open(the_file).unwrap_or_else(|e| {
        error!("Failed to open file {}: {}", the_file, e);
        panic!("file should open read only: {}", e);
    });
    let json: serde_json::Value = serde_json::from_reader(file).unwrap_or_else(|e| {
        error!("Failed to parse JSON from file {}: {}", the_file, e);
        panic!("file should be proper JSON: {}", e);
    });

    info!("Successfully loaded NASA POTD");
    Json(json!(json))
}