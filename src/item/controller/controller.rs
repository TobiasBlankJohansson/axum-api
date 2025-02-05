use axum::extract::State;
use axum::Json;
use serde::Deserialize;
use sqlx::PgPool;
use crate::item::model::item::Item;
use crate::item::service::service::Service;

#[derive(Deserialize)]
pub struct CreateItemRequest{
    name: String,
    quantity: i16,
    storage_area: String
}

pub async fn get_items(State(pool): State<PgPool>) -> Json<Vec<Item>>{
    let items = Service::get_item_list(&pool);
    Json(items.await)
}

pub async fn create_item(
    State(pool): State<PgPool>,
    Json(body): Json<CreateItemRequest>) -> uuid {
    let item_uuid = Service::create_item(&pool, &body.name, &body.quantity, &body.storage_area).await;
    Json(item_uuid)
}