use axum::{Router, routing::get};

mod vehicle;

use vehicle::{get_vehicle, post_vehicle};

#[tokio::main]
async fn main() {
    // 1. create axum server
    let router1 = Router::new().route("/vehicle", get(get_vehicle).post(post_vehicle));

    // 2. define the IP and port listener(TCP)
    let address = "0.0.0.0:6570";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    // 3. Call axum serve to launch the web server
    axum::serve(listener, router1).await.unwrap();
}
