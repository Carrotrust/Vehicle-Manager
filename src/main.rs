use axum::{Router, routing::get};

mod vehicle;

use vehicle::{get_vehicle, post_vehicle};

fn app() -> Router {
    Router::new().route("/vehicle", get(get_vehicle).post(post_vehicle))
}

#[tokio::main]
async fn main() {
    // 1. create axum server
    let router1 = app();

    // 2. define the IP and port listener(TCP)
    let address = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // 3. Call axum serve to launch the web server
    axum::serve(listener, router1).await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::app;
    use axum::{
        body::{to_bytes, Body},
        http::{Request, StatusCode},
    };
    use serde_json::Value;
    use tower::util::ServiceExt;

    #[tokio::test]
    async fn get_vehicle_returns_sample_vehicle_with_uuid() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/vehicle")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let payload: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(payload["manufacturer"], "toyota");
        assert_eq!(payload["model"], "camry-le");
        assert_eq!(payload["year"], 2018);

        let id = payload["id"].as_str().unwrap();
        assert!(uuid::Uuid::parse_str(id).is_ok());
    }

    #[tokio::test]
    async fn post_vehicle_returns_payload_with_generated_uuid() {
        let response = app()
            .oneshot(
                Request::builder()
                    .uri("/vehicle")
                    .method("POST")
                    .header("content-type", "application/json")
                    .body(Body::from(
                        r#"{"manufacturer":"Tesla","model":"CyberTruck","year":2025}"#,
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let payload: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(payload["manufacturer"], "Tesla");
        assert_eq!(payload["model"], "CyberTruck");
        assert_eq!(payload["year"], 2025);

        let id = payload["id"].as_str().unwrap();
        assert!(uuid::Uuid::parse_str(id).is_ok());
    }
}
