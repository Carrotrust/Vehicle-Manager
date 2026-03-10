use axum::{Json, debug_handler};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vehicle {
    manufacturer: String,
    model: String,
    year: u32,
    id: Option<String>,
}

#[debug_handler]
pub async fn get_vehicle() -> Json<Vehicle> {
    println!("caller called get vehicle");
    Json::from(Vehicle {
        manufacturer: String::from("toyota"),
        model: "camry-le".to_string(),
        year: 2018,
        id: Some(uuid::Uuid::new_v4().to_string()),
    })
}

pub async fn post_vehicle(Json(mut v): Json<Vehicle>) -> Json<Vehicle> {
    println!(
        "Manufacturer: {}, Model: {}, year: {}",
        v.manufacturer, v.model, v.year
    );
    v.id = Some(uuid::Uuid::new_v4().to_string());
    Json::from(v)
}
