use axum::Json;
use serde_json::{Value, json};
use rand::{Rng, seq::SliceRandom};
use std::fs;
use log::{info, error};
use std::collections::HashSet;

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
    info!("Selected country index: {}", num);

    let name = json[num]["name"]["common"]
        .as_str()
        .unwrap_or_else(|| {
            error!("Failed to extract country name as string");
            ""
        })
        .to_string();
    
    let correct_capital = json[num]["capital"][0]
        .as_str()
        .unwrap_or_else(|| {
            error!("Failed to extract capital as string");
            ""
        })
        .to_string();

    // Get 3 random incorrect capitals
    let mut rng = rand::rng();
    let mut wrong_capitals = Vec::new();
    let mut used_indices = HashSet::new();
    used_indices.insert(num); // Don't use the correct country

    while wrong_capitals.len() < 3 {
        let random_index = rng.random_range(0..total_countries);
        
        // Skip if we already used this country or it doesn't have a capital
        if used_indices.contains(&random_index) {
            continue;
        }

        if let Some(capital_array) = json[random_index]["capital"].as_array() {
            if let Some(capital_val) = capital_array.get(0) {
                if let Some(capital_str) = capital_val.as_str() {
                    wrong_capitals.push(capital_str.to_string());
                    used_indices.insert(random_index);
                }
            }
        }
    }

    // Combine correct capital with wrong capitals and shuffle
    let mut all_capitals = vec![correct_capital.clone()];
    all_capitals.extend(wrong_capitals);
    all_capitals.shuffle(&mut rng);

    info!("Selected country: {}, correct capital: {}", name, correct_capital);
    info!("Choices: {:?}", all_capitals);

    Json(json!({ 
        "country": name, 
        "capital": correct_capital,
        "choices": all_capitals
    }))
}