use axum::extract::{Path, State};
use axum::Json;
use serde::{Deserialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::item::controller::dto::item_dto::ItemDto;
use crate::item::service::service::Service;

#[derive(Deserialize)]
pub struct CreateItemRequest{
    name: String,
    quantity: i16,
    storage_area: String
}

pub async fn get_items(State(pool): State<PgPool>) -> Json<Vec<ItemDto>>{
    let items = Service::get_item_list(&pool).await;
    Json(ItemDto::to_model_list(items))
}

pub async fn get_item(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Json<ItemDto>{
    let item = Service::get_item(&pool,id).await;
    Json(ItemDto::to_model(item))
}

pub async fn create_item(
    State(pool): State<PgPool>,
    Json(body): Json<CreateItemRequest>) -> Json<Uuid> {
    let item_uuid = Service::create_item(&pool, &body.name, &body.quantity, &body.storage_area).await;
    Json(item_uuid)
}