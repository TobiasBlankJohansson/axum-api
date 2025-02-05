use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::item::model::item::Item;
use crate::item::service::service::Service;

#[derive(Deserialize)]
pub struct CreateItemRequest{
    name: String,
    quantity: i16,
    storage_area: String
}

#[derive(Serialize)]
pub struct ItemDto {
    id: Uuid,
    name: String,
    quantity: i16,
    storageArea: String
}
impl ItemDto{
    pub fn to_model(item: Item) -> ItemDto{
        ItemDto{
            id: item.id,
            name: item.name,
            quantity: item.quantity,
            storageArea: item.storage_area
        }
    }
}

pub async fn get_items(State(pool): State<PgPool>) -> Json<Vec<Item>>{
    let items = Service::get_item_list(&pool).await;
    Json(items)
}

pub async fn get_item(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> ItemDto{

}

pub async fn create_item(
    State(pool): State<PgPool>,
    Json(body): Json<CreateItemRequest>) -> Json<Uuid> {
    let item_uuid = Service::create_item(&pool, &body.name, &body.quantity, &body.storage_area).await;
    Json(item_uuid)
}