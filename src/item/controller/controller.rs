use axum::extract::State;
use axum::Json;
use sqlx::PgPool;
use crate::item::model::item::Item;
use crate::item::service::service::Service;

pub async fn get_items(State(pool): State<PgPool>) -> Json<Vec<Item>>{
    let items = Service::get_item_list(&pool);
    Json(items.await)
}