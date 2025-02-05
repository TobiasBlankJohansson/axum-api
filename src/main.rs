mod item;

use std::net::SocketAddr;
use axum::handler::Handler;
use axum::{Router, ServiceExt};
use axum::routing::Route;
use dotenv::dotenv;
use sqlx::Any;
use tower_http::cors::CorsLayer;
use item::database::database::establish_connection;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = establish_connection().await;

    let app = Router::new()
        .with_state(pool);

    let addr = SocketAddr::from(([127,0,0,1],3000));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
