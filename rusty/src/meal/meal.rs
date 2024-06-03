use axum::Json;
use serde_json::{json, Map, Value};
use std::fs;

async fn is_empty(value: &Value) -> bool {
    match value {
        Value::Null => true,
        Value::String(s) => s.is_empty() || s == " ",
        Value::Array(arr) => arr.is_empty(),
        Value::Object(map) => map.is_empty(),
        _ => false,
    }
}

async fn remove_empty(json_value: Value) -> Value {
    let mut json = Map::new();

    for (key, value) in json_value["meals"][0].as_object().unwrap() {
        if !is_empty(value).await {
            json.insert(key.to_owned(), value.to_owned());
        }
    }

    Value::Object(json)
}

async fn combine_fields(json_value: Value) -> Value {
    let mut json = Map::new();

    let mut count: i32 = 0;

    for (key, value) in json_value.as_object().unwrap() {
        // we just hope strMeasure and strIngredients match always in number
        if key.contains("strMeasure") {
            count += 1;
        } else if key.contains("strIngredient") {
        } else {
            json.insert(key.to_owned(), value.to_owned());
        }
    }

    for n in 1..count {
        let ingredients = "strIngredient".to_owned() + &n.to_string();
        let measure = "strMeasure".to_owned() + &n.to_string();

        let val1 = json_value[ingredients].to_string();
        let val2: String = json_value[measure].to_string();

        let full_str = val1.to_owned() + &val2;
        json.insert(n.to_string(), serde_json::Value::String(full_str));
    }

    Value::Object(json)
}

pub async fn random() -> Json<Value> {
    let the_file = "ayylmao/random_meal.json";

    let file = fs::File::open(the_file).expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    let new_json = remove_empty(json).await;
    let final_json = combine_fields(new_json).await;

    // println!("{}", serde_json::to_string_pretty(&final_json).unwrap());
    Json(json!(final_json))
}
