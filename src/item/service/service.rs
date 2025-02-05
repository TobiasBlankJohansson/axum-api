use sqlx::PgPool;
use crate::item::repository::repository::Repository;

pub struct Service;

impl Service {
    fn get_item_list(pool: &PgPool){
        Repository::inventory_list(pool)
    }
}
