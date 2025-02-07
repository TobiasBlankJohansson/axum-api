use std::sync::Arc;
use axum::{extract::{Path, Query, State}, http::StatusCode, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use utoipa::{ToSchema, OpenApi, IntoParams};
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use crate::item::controller::dto::item_dto::ItemDto;
use crate::item::database::database::establish_connection;
use crate::item::error_handler::error_handler::ApiError;
use crate::item::service::service::Service;

pub async fn router() -> OpenApiRouter {
    let pool = Arc::new(establish_connection().await);
    OpenApiRouter::new()
        .routes(routes!(get_items, create_item))
        .routes(routes!(get_item, delete_item, update_item))
        .with_state(pool)
}

#[utoipa::path(
    get,
    path = "/items",
    params(StorageAreaParams),
    responses(
        (status = 200, description = "List all items", body = [ItemDto]),
        (status = 500, description = "Internal Server Error", body = ApiError)
    )
)]
pub async fn get_items(State(pool): State<Arc<PgPool>>, Query(storage_area): Query<StorageAreaParams>)
                       -> Result<Json<Vec<ItemDto>>, ApiError> {
    let items = match storage_area.storage_area {
        None => Service::get_item_list(&pool, "").await?,
        Some(query) => Service::get_item_list(&pool, &query).await?,
    };
    Ok(Json(ItemDto::to_model_list(items)))
}

#[utoipa::path(
    get,
    path = "/items/{id}",
    params(("id" = Uuid, Path, description = "Item ID")),
    responses(
        (status = 200, description = "Get a specific item", body = ItemDto),
        (status = 404, description = "Item not found", body = ApiError)
    )
)]
pub async fn get_item(State(pool): State<Arc<PgPool>>, Path(id): Path<Uuid>) -> Result<Json<ItemDto>, ApiError> {
    let item = Service::get_item(&pool, id).await?;
    Ok(Json(ItemDto::to_model(item)))
}

#[utoipa::path(
    post,
    path = "/items",
    request_body = CreateItemRequest,
    responses(
        (status = 201, description = "Item created", body = Uuid),
        (status = 400, description = "Bad request", body = ApiError)
    )
)]
pub async fn create_item(State(pool): State<Arc<PgPool>>, Json(body): Json<CreateItemRequest>) -> Result<Json<Uuid>, ApiError> {
    let item_uuid = Service::create_item(&pool, &body.name, &body.quantity, &body.storage_area).await?;
    Ok(Json(item_uuid))
}

#[utoipa::path(
    delete,
    path = "/items/{id}",
    params(("id" = Uuid, Path, description = "Item ID")),
    responses(
        (status = 204, description = "Item deleted"),
        (status = 404, description = "Item not found", body = ApiError)
    )
)]
pub async fn delete_item(State(pool): State<Arc<PgPool>>, Path(id): Path<Uuid>) -> Result<impl IntoResponse, ApiError> {
    Service::delete_item(&pool, id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    put,
    path = "/items/{id}",
    params(("id" = Uuid, Path, description = "Item ID")),
    request_body = CreateItemRequest,
    responses(
        (status = 204, description = "Item updated"),
        (status = 404, description = "Item not found", body = ApiError)
    )
)]
pub async fn update_item(
    State(pool): State<Arc<PgPool>>,
    Path(id): Path<Uuid>,
    Json(body): Json<CreateItemRequest>,
) -> Result<impl IntoResponse, ApiError> {
    Service::update_item(&pool, id, &body.name, &body.quantity, &body.storage_area).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Items", description = "Operations related to items")
    )
)]
pub struct ApiDoc;

#[derive(Deserialize, Serialize, ToSchema)]
pub struct CreateItemRequest {
    name: String,
    quantity: i16,
    storage_area: String,
}

#[derive(Deserialize, Serialize, ToSchema, IntoParams)]
pub struct StorageAreaParams {
    storage_area: Option<String>,
}
