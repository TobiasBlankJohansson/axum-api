use sqlx::PgPool;
use uuid::Uuid;
use crate::item::error_handler::error_handler::ApiError;
use crate::item::model::item::Item;
use crate::item::repository::repository::Repository;

pub struct Service;

impl Service {
    pub async fn get_item_list(pool: &PgPool, storage_area: &str) -> Result<Vec<Item>, ApiError> {
        let items = Repository::inventory_list(pool, storage_area).await?;
        Ok(items)
    }

    pub async fn get_item(pool: &PgPool, id: Uuid) -> Result<Item, ApiError> {
        Repository::get_item_by_id(pool, id).await?.ok_or(ApiError::NotFound)
    }

    pub async fn create_item(pool: &PgPool, name: &str, quantity: &i16, storage_area: &str)
                             -> Result<Uuid, ApiError> {
        Repository::create_item(pool, name, quantity, storage_area).await?
            .ok_or(ApiError::NotFound)
    }

    pub async fn delete_item(pool: &PgPool, id: Uuid) -> Result<(), ApiError> {
        Repository::delete_item(pool, id).await
    }

    pub async fn update_item(pool: &PgPool, id: Uuid, name: &str, quantity: &i16, storage_area: &str)
                             -> Result<(), ApiError> {
        Repository::update_item(pool, id, name, quantity, storage_area).await
    }
}
