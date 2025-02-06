mod item;

use std::net::SocketAddr;
use axum::{Router};
use axum::routing::{delete, get, post, put};
use dotenv::dotenv;
use tower_http::cors::{Any, CorsLayer};
use item::database::database::establish_connection;
use crate::item::controller::controller::{create_item, delete_item, get_item, get_items, update_item};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any);

    let pool = establish_connection().await;

    let app = Router::new()
        .route("/api/items", get(get_items))
        .route("/api/items/:id", get(get_item))
        .route("/api/items", post(create_item))
        .route("/api/items/:id", delete(delete_item))
        .route("/api/items/:id", put(update_item))
        .with_state(pool)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}
