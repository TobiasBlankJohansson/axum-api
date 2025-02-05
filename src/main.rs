mod item;

use std::net::SocketAddr;
use axum::handler::Handler;
use axum::{Router, ServiceExt};
use axum::routing::get;
use dotenv::dotenv;
use item::database::database::establish_connection;
use crate::item::controller::controller::get_items;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = establish_connection().await;

    let app = Router::new()
        .route("/api/items",get(get_items))
        .with_state(pool);

    let addr = SocketAddr::from(([127,0,0,1],3000));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
