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

#[cfg(test)]
mod tests {
    use super::{Vehicle, get_vehicle, post_vehicle};
    use axum::Json;

    #[tokio::test]
    async fn get_vehicle_returns_expected_sample_vehicle() {
        let Json(vehicle) = get_vehicle().await;

        assert_eq!(vehicle.manufacturer, "toyota");
        assert_eq!(vehicle.model, "camry-le");
        assert_eq!(vehicle.year, 2018);

        let id = vehicle.id.as_deref().unwrap();
        assert!(uuid::Uuid::parse_str(id).is_ok());
    }

    #[tokio::test]
    async fn post_vehicle_adds_id_and_preserves_input_fields() {
        let input = Vehicle {
            manufacturer: "Tesla".to_string(),
            model: "CyberTruck".to_string(),
            year: 2025,
            id: None,
        };

        let Json(vehicle) = post_vehicle(Json(input)).await;

        assert_eq!(vehicle.manufacturer, "Tesla");
        assert_eq!(vehicle.model, "CyberTruck");
        assert_eq!(vehicle.year, 2025);

        let id = vehicle.id.as_deref().unwrap();
        assert!(uuid::Uuid::parse_str(id).is_ok());
    }
}
