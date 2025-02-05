use sqlx::PgPool;
use crate::item::model::item::Item;

pub async fn inventory_list(pool: &PgPool) -> Vec<Item>{
    sqlx::query_as::<_,Item>("SELECT id,name,quantity,storage_area From inventory")
        .fetch_all(pool.get_ref())
        .await
        .unwrap()
}