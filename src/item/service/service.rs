use sqlx::PgPool;
use crate::item::model::item::Item;
use crate::item::repository::repository::Repository;

pub struct Service;

impl Service {
    pub async fn get_item_list(pool: &PgPool) -> Vec<Item>{
        Repository::inventory_list(pool).await
    }
}
