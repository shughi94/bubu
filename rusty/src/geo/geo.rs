use axum::Json;
use serde_json::{Value, json};
use rand::Rng;
use std::fs;
use log::{info, error};

// let geo_url = "https://restcountries.com/v3.1/all?fields=name,capital";
const THE_FILE: &str = "ayylmao/geo.json";

pub async fn random() -> Json<Value> {
    info!("Geography random endpoint accessed");

    let file = fs::File::open(THE_FILE).unwrap_or_else(|e| {
        error!("Failed to open file {}: {}", THE_FILE, e);
        panic!("file should open read only: {}", e);
    });
    let json: serde_json::Value =
        serde_json::from_reader(file).unwrap_or_else(|e| {
            error!("Failed to parse JSON from file {}: {}", THE_FILE, e);
            panic!("file should be proper JSON: {}", e);
        });

    let total_countries = json.as_array().unwrap().len();
    let num = rand::rng().random_range(0..total_countries);
    info!("num: {}", num);

    let name = json[num]["name"]["common"]
        .as_str()
        .unwrap_or_else(|| {
            error!("Failed to extract country name as string");
            ""
        })
        .to_string();
    
    let capital = json[num]["capital"][0]
        .as_str()
        .unwrap_or_else(|| {
            error!("Failed to extract capital as string");
            ""
        })
        .to_string();

    info!("Selected country: {}, capital: {}", name, capital);
    Json(json!({ "country": name, "capital": capital }))
}