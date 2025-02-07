use sqlx::PgPool;
use uuid::Uuid;
use crate::item::error_handler::database_error_details::DatabaseErrorDetails;
use crate::item::error_handler::error_handler::{ApiError};
use crate::item::model::item::Item;

pub struct Repository;
impl Repository {
    pub async fn inventory_list(pool: &PgPool, storage_area: &str) -> Result<Vec<Item>, ApiError> {
        let query = format!("%{}%", storage_area);
        let items = sqlx::query_as::<_, Item>("SELECT * From inventory WHERE storage_area LIKE $1")
            .bind(query)
            .fetch_all(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(DatabaseErrorDetails::from(e)))?;
        Ok(items)
    }

    pub async fn get_item_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Item>, ApiError> {
        let items = sqlx::query_as::<_, Item>("SELECT * FROM inventory WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(DatabaseErrorDetails::from(e)))?;
        Ok(items)
    }

    pub async fn create_item(pool: &PgPool, name: &str, quantity: &i16, storage_area: &str)
        -> Result<Option<Uuid>, ApiError> {
        let items = sqlx::query_scalar::<_, Uuid>("INSERT INTO inventory (name,quantity,storage_area) VALUES ($1,$2,$3) RETURNING id")
            .bind(name)
            .bind(quantity)
            .bind(storage_area)
            .fetch_optional(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(DatabaseErrorDetails::from(e)))?;
        Ok(items)
    }

    pub async fn delete_item(pool: &PgPool, id: Uuid) -> Result<(), ApiError> {
        sqlx::query::<_>("DELETE FROM inventory WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(DatabaseErrorDetails::from(e)))?;
        Ok(())
    }

    pub async fn update_item(pool: &PgPool, id: Uuid, name: &str, quantity: &i16, storage_area: &str) -> Result<(), ApiError> {
        sqlx::query::<_>("UPDATE inventory SET name = $1, quantity = $2, storage_area = $3 WHERE id = $4")
            .bind(name)
            .bind(quantity)
            .bind(storage_area)
            .bind(id)
            .execute(pool)
            .await
            .map_err(|e| ApiError::DatabaseError(DatabaseErrorDetails::from(e)))?;
        Ok(())
    }
}
