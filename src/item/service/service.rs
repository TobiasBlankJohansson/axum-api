use sqlx::PgPool;
use crate::item::model::item::Item;
use crate::item::repository::repository::Repository;

pub struct Service;

impl Service {
    pub async fn get_item_list(pool: &PgPool) -> Vec<Item> {
        Repository::inventory_list(pool).await
    }

    pub async fn create_item(pool: &PgPool, name: &String, quantity: &i16, storage_area: &String) -> uuid {
        Repository::create_item(pool, name, quantity, storage_area).await
    }
}
