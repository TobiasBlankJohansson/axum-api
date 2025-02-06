use axum::extract::{Path, State};
use axum::http::{StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize};
use sqlx::PgPool;
use uuid::Uuid;
use crate::item::controller::dto::item_dto::ItemDto;
use crate::item::error_handler::error_handler::ApiError;
use crate::item::service::service::Service;

#[derive(Deserialize)]
pub struct CreateItemRequest{
    name: String,
    quantity: i16,
    storage_area: String
}

pub async fn get_items(State(pool): State<PgPool>) -> Result<Json<Vec<ItemDto>>, ApiError>{
    let items = Service::get_item_list(&pool).await?;
    Ok(Json(ItemDto::to_model_list(items)))
}

pub async fn get_item(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<Json<ItemDto>, ApiError>{
    let item = Service::get_item(&pool,id).await?;
    Ok(Json(ItemDto::to_model(item)))
}

pub async fn create_item(
    State(pool): State<PgPool>,
    Json(body): Json<CreateItemRequest>) -> Result<Json<Uuid>, ApiError> {
    let item_uuid = Service::create_item(&pool, &body.name, &body.quantity, &body.storage_area).await?;
    Ok(Json(item_uuid))
}
pub async fn delete_item(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, ApiError> {
    Service::delete_item(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_item(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateItemRequest>) -> Result<impl IntoResponse, ApiError> {
    Service::update_item(&pool, id, &body.name, &body.quantity, &body.storage_area).await?;
    Ok(StatusCode::NO_CONTENT)
}