use sqlx::PgPool;
use uuid::Uuid;
use crate::item::model::item::Item;

pub struct Repository;
impl Repository {
    pub async fn inventory_list(pool: &PgPool) -> Vec<Item> {
        sqlx::query_as::<_, Item>("SELECT id,name,quantity,storage_area From inventory")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    pub async fn create_item(pool: &PgPool, name: &String, quantity: &i16, storage_area: &String) -> uuid {
        sqlx::query_as::<_, uuid>("INSERT INTO inventory (name,quantity,storage_area) VALUES ($1,$2,$3) RETURNING id")
            .bind(name)
            .bind(quantity)
            .bind(storage_area)
            .fetch_one(pool)
            .await
            .unwrap()
    }
}
